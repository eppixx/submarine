use crate::data::{Info, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#savePlayQueue
    pub async fn save_play_queue(
        &self,
        id: Vec<impl Into<String>>,
        current: Option<impl Into<String>>,
        position: Option<i64>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        for id in id {
            paras.insert("id", id.into());
        }
        if let Some(current) = current {
            paras.insert("current", current.into());
        }
        if let Some(position) = position {
            paras.insert("position", position.to_string());
        }

        let body = self.request("savePlayQueue", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
