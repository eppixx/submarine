use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError, Parameter,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deleteUser
    pub async fn delete_user(&self, username: impl Into<String>) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("username", username);

        let body = self.request("deleteUser", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
