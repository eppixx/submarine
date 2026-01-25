use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#unstar>
    pub async fn unstar(
        &self,
        id: Vec<impl Into<String>>,
        album_id: Vec<impl Into<String>>,
        artist_id: Vec<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        for id in id {
            paras.push("id", id);
        }
        for id in album_id {
            paras.push("albumId", id);
        }
        for id in artist_id {
            paras.push("artistId", id);
        }

        let body = self.request("unstar", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
