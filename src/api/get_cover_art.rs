use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getCoverArt
    pub fn get_cover_art_url(&self, id: impl Into<String>, size: Option<i32>) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("id", id);
        if let Some(size) = size {
            paras.push("size", size.to_string());
        }

        url::Url::parse_with_params(&format!("{}/rest/getCoverArt", self.server_url), paras.0)
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getCoverArt
    pub async fn get_cover_art(
        &self,
        id: impl Into<String>,
        size: Option<i32>,
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_cover_art_url(id, size)?).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::AuthBuilder, Client};

    #[tokio::test]
    async fn create_get_cover_art_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://target.com", auth);
        let url = client.get_cover_art_url("testId", None).unwrap();

        assert_eq!("https://target.com/rest/getCoverArt?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId", &url.to_string());
    }
}
