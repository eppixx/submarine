use crate::{
    data::{ChatMessage, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getChatMessages
    pub async fn get_chat_messages(
        &self,
        since: Option<i64>,
    ) -> Result<Vec<ChatMessage>, SubsonicError> {
        let mut paras = Parameter::new();
        if let Some(since) = since {
            paras.push("since", since.to_string());
        }

        let body = self.request("getChatMessages", Some(paras), None).await?;
        if let ResponseType::ChatMessages { chat_messages } = body.data {
            Ok(chat_messages.chat_message)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type ChatMessages but found wrong type",
            )))
        }
    }
}

//TODO add test
