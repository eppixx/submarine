use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#addChatMessage
    pub async fn add_chat_messages(
        &self,
        message: impl Into<String>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("message", message);

        let body = self.request("addChatMessage", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
