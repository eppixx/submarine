use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// this creates a url to id for download purpuses
    ///
    /// reference: <http://www.subsonic.org/pages/api.jsp#download>
    pub fn download_url(&self, id: impl Into<String>) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("id", id);

        url::Url::parse_with_params(&format!("{}/rest/download", self.server_url), paras.0)
    }

    /// reference: <http://www.subsonic.org/pages/api.jsp#download>
    pub async fn download(&self, id: impl Into<String>) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.download_url(id)?).await {
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
    async fn create_download_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://subsonic.example", auth);
        let url = client.download_url("testId").unwrap();

        assert_eq!("https://subsonic.example/rest/download?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId", &url.to_string());
    }
}
