use crate::{
    data::{ResponseType, User},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getUser
    pub async fn get_user(&self, username: impl Into<String>) -> Result<User, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("username", username);

        let body = self.request("getUser", Some(paras), None).await?;
        if let ResponseType::User { user } = body.data {
            Ok(user)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type User but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_user() {
        let response_body = r##"
{
    "subsonic-response": {
        "status": "ok",
        "version": "1.16.1",
        "type": "navidrome",
        "serverVersion": "0.49.3 (8b93962f)",
        "user": {
            "username": "eppixx",
            "scrobblingEnabled": true,
            "adminRole": false,
            "settingsRole": false,
            "downloadRole": true,
            "uploadRole": false,
            "playlistRole": false,
            "coverArtRole": false,
            "commentRole": false,
            "podcastRole": false,
            "streamRole": true,
            "jukeboxRole": false,
            "shareRole": false,
            "videoConversionRole": false
        }
    }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::User { user } = response.data {
            assert_eq!(&user.username, "eppixx");
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
