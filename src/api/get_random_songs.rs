use crate::data::{Child, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getRandomSongs
    pub async fn get_random_songs(
        &self,
        size: Option<i32>, //defaults to 10
        genre: Option<impl Into<String>>,
        from_year: Option<i32>,
        to_year: Option<i32>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Parameter::new();
        if let Some(size) = size {
            paras.push("size", size.to_string());
        }
        if let Some(genre) = genre {
            paras.push("genre", genre);
        }
        if let Some(from_year) = from_year {
            paras.push("fromYear", from_year.to_string());
        }
        if let Some(to_year) = to_year {
            paras.push("toYear", to_year.to_string());
        }
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }

        let body = self.request("getRandomSongs", Some(paras), None).await?;
        if let ResponseType::RandomSongs { random_songs } = body.data {
            Ok(random_songs.song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type RandomSongs but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_get_random_songs() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "randomSongs": {
      "song": [
        {
          "id": "abd3c3bc92c3985e2ff77f9a36edcfa9",
          "parent": "9c1fd785e88b12723f758f2f31b516d8",
          "isDir": false,
          "title": "Showdown (radio edit)",
          "album": "Showdown",
          "artist": "Pendulum",
          "track": 2,
          "year": 2009,
          "genre": "Dance",
          "coverArt": "mf-abd3c3bc92c3985e2ff77f9a36edcfa9_647723e6",
          "size": 4341006,
          "contentType": "audio/ogg",
          "suffix": "ogg",
          "duration": 203,
          "bitRate": 167,
          "path": "Pendulum/Showdown/02 - Showdown (radio edit).ogg",
          "discNumber": 1,
          "created": "2023-05-31T10:50:41.85017274Z",
          "albumId": "9c1fd785e88b12723f758f2f31b516d8",
          "artistId": "bb8b3b51eb16a9a78b8d3e57cdc30e62",
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
        if let crate::data::ResponseType::RandomSongs { random_songs } = response.data {
            assert_eq!(
                random_songs.song.first().unwrap().album.as_deref(),
                Some("Showdown")
            );
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
