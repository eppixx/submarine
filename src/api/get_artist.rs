use crate::data::{Artist, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    pub async fn get_artist(&self, id: &str) -> Result<Artist, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let body = self.request("getArtist", Some(paras), None).await?;
        if let ResponseType::Artist { artist } = body.data {
            Ok(artist)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_artist() {
        let response_body = r##"
        {
          "subsonic-response": {
            "status": "ok",
            "version": "1.16.1",
            "type": "navidrome",
            "serverVersion": "0.49.3 (8b93962f)",
            "artist": {
              "id": "0cc175b9c0f1b6a831c399e269772661",
              "name": "A",
              "coverArt": "ar-0cc175b9c0f1b6a831c399e269772661_0",
              "albumCount": 1,
              "album": [
                {
                  "id": "c58e1cddef76983860e21603bc85c363",
                  "parent": "0cc175b9c0f1b6a831c399e269772661",
                  "isDir": true,
                  "title": "Hi-Fi Serious",
                  "name": "Hi-Fi Serious",
                  "album": "Hi-Fi Serious",
                  "artist": "A",
                  "year": 2002,
                  "coverArt": "al-c58e1cddef76983860e21603bc85c363_6113f6cd",
                  "duration": 2647,
                  "created": "2021-08-11T16:33:47.650644719Z",
                  "artistId": "0cc175b9c0f1b6a831c399e269772661",
                  "songCount": 12,
                  "isVideo": false
                }
              ]
            }
          }
        }"##;

        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Artist { artist } = response.data {
            assert_eq!(artist.albums.len(), 1);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}