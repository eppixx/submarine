use crate::{Client, SubsonicError, Parameter};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getAvatar
    pub fn get_avatar_url(&self, username: impl Into<String>) -> String {
        let mut paras = Parameter::new();
        paras.push("username", username);

        let mut url: String = self.server_url.clone() + "/rest/getAvatar?";
        for p in paras.0 {
            url += &("&".to_owned() + &p.0 + "=" + &p.1);
        }

        url
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getAvatar
    pub async fn get_avatar(&self, username: impl Into<String>) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_avatar_url(username)).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

//TODO add test
