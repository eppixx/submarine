use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#deletePlaylist
    pub async fn delete_playlist(&self, id: impl Into<String>) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());

        let body = self.request("createPlaylist", Some(paras), None).await?;
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
    fn ping_convert() {
        let response_txt = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)"
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_txt)
            .unwrap()
            .inner;
        if let ResponseType::Ping {} = response.data {
            assert_eq!(&response.info.status, "ok");
        } else {
            panic!("wrong type");
        }
    }
}
