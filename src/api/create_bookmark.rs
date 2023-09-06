use crate::data::{Info, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createBookmark
    pub async fn create_bookmark(
        &self,
        id: impl Into<String>,
        position: i64, // in ms
        comment: Option<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());
        paras.insert("position", position.to_string());
        if let Some(comment) = comment {
            paras.insert("comment", comment.into());
        }

        let body = self.request("createBookmark", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
