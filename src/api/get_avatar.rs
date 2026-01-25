use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// creates a url to the avatar of the given user
    ///
    /// reference: <http://www.subsonic.org/pages/api.jsp#getAvatar>
    pub fn get_avatar_url(&self, username: impl Into<String>) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("username", username);

        url::Url::parse_with_params(&format!("{}/rest/getAvatar", self.server_url), paras.0)
    }

    /// reference: <http://www.subsonic.org/pages/api.jsp#getAvatar>
    pub async fn get_avatar(&self, username: impl Into<String>) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_avatar_url(username)?).await {
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
    async fn create_get_avatar_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://subsonic.example", auth);
        let url = client.get_avatar_url("name").unwrap();

        assert_eq!("https://subsonic.example/rest/getAvatar?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&username=name", &url.to_string());
    }
}
