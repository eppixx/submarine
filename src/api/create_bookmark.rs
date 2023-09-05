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

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_create_bookmark() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)"
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Ping {} = response.data {
            assert_eq!(&response.info.status, "ok");
        } else {
            panic!("wrong type");
        }
    }
}
