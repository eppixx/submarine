use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#changePassword
    pub async fn change_password(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("username", username.into());
        paras.insert("password", password.into());

        let body = self.request("changePassword", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
