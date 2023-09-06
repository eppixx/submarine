use crate::data::{Info, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deleteBookmark
    pub async fn delete_bookmark(&self, id: impl Into<String>) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());

        let body = self.request("deleteBookmark", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
