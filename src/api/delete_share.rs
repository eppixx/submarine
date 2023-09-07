use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deleteShare
    pub async fn delete_share(&self, id: impl Into<String>) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("deleteShare", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
