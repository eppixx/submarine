use crate::data::{ResponseType, SimilarSongs};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getSimilarSongs2
    pub async fn get_similar_songs2(
        &self,
        id: impl Into<String>,
        count: Option<i32>, //defaults to 50
    ) -> Result<SimilarSongs, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());
        if let Some(count) = count {
            paras.insert("count", count.to_string());
        }

        let body = self.request("getSimilarSongs2", Some(paras), None).await?;
        if let ResponseType::SimilarSongs2 { similar_songs2 } = body.data {
            Ok(similar_songs2)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type SimilarSongs2 but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_get_similar_songs2() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "similarSongs2": {
      "song": [
        {
          "id": "43edab171c9544674baf582dd953377e",
          "parent": "c58e1cddef76983860e21603bc85c363",
          "isDir": false,
          "title": "The Springs",
          "album": "Hi-Fi Serious",
          "artist": "A",
          "track": 7,
          "year": 2002,
          "coverArt": "mf-43edab171c9544674baf582dd953377e_6113f6cd",
          "size": 2809163,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 168,
          "bitRate": 128,
          "path": "A/Hi-Fi Serious/07 - The Springs.mp3",
          "discNumber": 1,
          "created": "2021-08-11T16:33:47.650498544Z",
          "albumId": "c58e1cddef76983860e21603bc85c363",
          "artistId": "0cc175b9c0f1b6a831c399e269772661",
          "type": "music",
          "isVideo": false
        }
      ]
    }
  }
}"##;
        let response = serde_json::from_str::<crate::data::OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let crate::data::ResponseType::SimilarSongs2 { similar_songs2 } = response.data {
            assert_eq!(&similar_songs2.song.first().unwrap().title, "The Springs");
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
