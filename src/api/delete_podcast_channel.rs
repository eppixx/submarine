use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deletePodcastChannel
    pub async fn delete_podcast_channel(
        &self,
        id: impl Into<String>,
    ) -> Result<Info, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("id", id.into());

        let body = self
            .request("deletePodcastChannel", Some(paras), None)
            .await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}

// TODO add test
