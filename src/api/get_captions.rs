use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// this creates a url to the given captions id
    ///
    /// reference: <http://www.subsonic.org/pages/api.jsp#getCatptions>
    pub fn get_captions_url(
        &self,
        id: impl Into<String>,
        format: Option<impl Into<String>>,
    ) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("id", id);
        if let Some(format) = format {
            paras.push("format", format);
        }

        url::Url::parse_with_params(&format!("{}/rest/getCaptions", self.server_url), paras.0)
    }

    /// reference: <http://www.subsonic.org/pages/api.jsp#getCatptions>
    pub async fn get_captions(
        &self,
        id: impl Into<String>,
        format: Option<impl Into<String>>,
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_captions_url(id, format)?).await {
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
    async fn create_get_captions_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://target.com", auth);
        let url = client.get_captions_url("testId", None::<&str>).unwrap();

        assert_eq!("https://target.com/rest/getCaptions?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId", &url.to_string());
    }
}
