use crate::{
    data::{Directory, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getMusicDirectory
    pub async fn get_music_directory(
        &self,
        music_directory_id: impl Into<String>,
    ) -> Result<Directory, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("musicDirectoryId", music_directory_id.into());

        let body = self.request("getMusicDirectory", Some(paras), None).await?;
        if let ResponseType::MusicDirectory { directory } = body.data {
            Ok(directory)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type MusicDirectory but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_music_directory() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "directory": {
      "child": [
        {
          "id": "ae9a4537ec07222c4948ddd7f9e76baf",
          "parent": "5338af695f89024e8a57c08ccfe1b814",
          "isDir": true,
          "title": "When Your Heart Stops Beating",
          "name": "When Your Heart Stops Beating",
          "album": "When Your Heart Stops Beating",
          "artist": "+44",
          "year": 2006,
          "genre": "Unbekannt",
          "coverArt": "al-ae9a4537ec07222c4948ddd7f9e76baf_611688d0",
          "duration": 2813,
          "playCount": 1,
          "played": "2022-11-30T22:24:31Z",
          "created": "2021-08-13T15:33:38.7188554Z",
          "artistId": "5338af695f89024e8a57c08ccfe1b814",
          "songCount": 13,
          "isVideo": false
        }
      ],
      "id": "5338af695f89024e8a57c08ccfe1b814",
      "name": "+44",
      "playCount": 1,
      "played": "2022-11-30T22:24:31Z",
      "albumCount": 1
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::MusicDirectory { directory } = response.data {
            assert_eq!(&directory.name, "+44");
        } else {
            panic!("wrong type");
        }
    }
}
