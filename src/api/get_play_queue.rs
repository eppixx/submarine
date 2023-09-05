use crate::data::{Info, PlayQueue, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getPlayQueue
    pub async fn get_play_queue(&self) -> Result<Result<PlayQueue, Info>, SubsonicError> {
        let paras = std::collections::HashMap::new();

        let body = self.request("getPlayQueue", Some(paras), None).await?;
        match body.data {
            ResponseType::Ping {} => Ok(Err(body.info)),
            ResponseType::PlayQueue { play_queue } => Ok(Ok(play_queue)),
            _ => Err(SubsonicError::Submarine(String::from(
                "expected type PlayQueue or Ping but found wrong type",
            ))),
        }
    }
}

//TODO add more tests
#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_play_queue() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "playQueue": {
      "entry": [
        {
          "id": "",
          "isDir": false,
          "path": "//.",
          "created": "0001-01-01T00:00:00Z",
          "type": "music",
          "isVideo": false
        },
        {
          "id": "",
          "isDir": false,
          "path": "//.",
          "created": "0001-01-01T00:00:00Z",
          "type": "music",
          "isVideo": false
        }
      ],
      "current": "fd14abe8db3b16c6f06929441be194ed",
      "username": "eppixx",
      "changed": "2021-08-25T12:12:24.811330913Z",
      "changedBy": "Sublime Music"
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::PlayQueue { play_queue } = response.data {
            assert_eq!(play_queue.entry.len(), 2);
            assert_eq!(&play_queue.username, "eppixx");
        } else {
            panic!("wrong type");
        }
    }
}

/*


*/
