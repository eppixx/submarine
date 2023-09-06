use crate::data::{IndexId3, ResponseType};
use crate::{Client, SubsonicError, Parameter};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getArtists
    pub async fn get_artists(
        &self,
        music_folder_id: Option<String>,
    ) -> Result<Vec<IndexId3>, SubsonicError> {
        let mut paras = Parameter::new();
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }

        let body = self.request("getArtists", Some(paras), None).await?;
        if let ResponseType::Artists { artists } = body.data {
            Ok(artists.index)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Artists but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_artists() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "artists": {
                  "index": [
                    {
                      "name": "#",
                      "artist": [
                        {
                          "id": "5338af695f89024e8a57c08ccfe1b814",
                          "name": "+44",
                          "albumCount": 1,
                          "coverArt": "ar-5338af695f89024e8a57c08ccfe1b814_0"
                        }
                      ]
                    },
                    {
                      "name": "A",
                      "artist": [
                        {
                          "id": "0cc175b9c0f1b6a831c399e269772661",
                          "name": "A",
                          "albumCount": 1,
                          "coverArt": "ar-0cc175b9c0f1b6a831c399e269772661_0"
                        }
                      ]
                    }
                  ]
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Artists { artists } = response.data {
            assert_eq!(artists.index.len(), 2);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
