use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError, Parameter,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deletePlaylist
    pub async fn delete_playlist(&self, id: impl Into<String>) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("createPlaylist", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
