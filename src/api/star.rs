use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#star
    pub async fn star(
        &self,
        id: Vec<impl Into<String>>,
        album_id: Vec<impl Into<String>>,
        artist_id: Vec<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        for id in id {
            paras.insert("id", id.into());
        }
        for id in album_id {
            paras.insert("albumId", id.into());
        }
        for id in artist_id {
            paras.insert("artistId", id.into());
        }

        let body = self.request("star", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
