use crate::data::{Child, ResponseType};
use crate::{Client, SubsonicError, Parameter};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getTopSongs
    pub async fn get_top_songs(
        &self,
        artist: impl Into<String>,
        count: Option<i32>, //defaults to 50
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("artist", artist);
        if let Some(count) = count {
            paras.push("count", count.to_string());
        }

        let body = self.request("getTopSongs", Some(paras), None).await?;
        if let ResponseType::TopSongs { top_songs } = body.data {
            Ok(top_songs.song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type TopSongs but found wrong type",
            )))
        }
    }
}

mod tests {
    //TODO better test
    #[test]
    fn conversion_empty_get_top_songs() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "topSongs": {}
  }
}"##;
        let response = serde_json::from_str::<crate::data::OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let crate::data::ResponseType::TopSongs { top_songs } = response.data {
            assert_eq!(top_songs.song.is_empty(), true);
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }

    #[test]
    fn conversion_get_top_songs() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "topSongs": {
      "song": [
        {
          "id": "1bd66355b36ebb2cd2b498a6ba95bd83",
          "parent": "c58e1cddef76983860e21603bc85c363",
          "isDir": false,
          "title": "Nothing",
          "album": "Hi-Fi Serious",
          "artist": "A",
          "track": 1,
          "year": 2002,
          "coverArt": "mf-1bd66355b36ebb2cd2b498a6ba95bd83_6113f6cd",
          "size": 5483084,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 223,
          "bitRate": 192,
          "path": "A/Hi-Fi Serious/01 - Nothing.mp3",
          "discNumber": 1,
          "created": "2021-08-11T16:33:47.650644719Z",
          "albumId": "c58e1cddef76983860e21603bc85c363",
          "artistId": "0cc175b9c0f1b6a831c399e269772661",
          "type": "music",
          "isVideo": false
        },
        {
          "id": "fa07456b89d9ec65fac9b0eff9a2a4ca",
          "parent": "c58e1cddef76983860e21603bc85c363",
          "isDir": false,
          "title": "Starbucks",
          "album": "Hi-Fi Serious",
          "artist": "A",
          "track": 6,
          "year": 2002,
          "coverArt": "mf-fa07456b89d9ec65fac9b0eff9a2a4ca_6113f6cd",
          "size": 4876014,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 198,
          "bitRate": 192,
          "path": "A/Hi-Fi Serious/06 - Starbucks.mp3",
          "discNumber": 1,
          "created": "2021-08-11T16:33:47.650408043Z",
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
        if let crate::data::ResponseType::TopSongs { top_songs } = response.data {
            assert_eq!(top_songs.song.len(), 2);
            assert_eq!(top_songs.song.first().unwrap().duration, Some(223));
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
