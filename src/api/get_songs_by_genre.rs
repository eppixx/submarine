use crate::data::{Child, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#getSongsByGenre>
    pub async fn get_songs_by_genre(
        &self,
        genre: impl Into<String>,
        count: Option<i32>,  //defaults to 10
        offset: Option<i32>, // defaults to 0
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("genre", genre);
        if let Some(count) = count {
            paras.push("count", count.to_string());
        }
        if let Some(offset) = offset {
            paras.push("offset", offset.to_string());
        }
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }

        let body = self.request("getSongsByGenre", Some(paras), None).await?;
        if let ResponseType::SongsByGenre { songs_by_genre } = body.data {
            Ok(songs_by_genre.song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type SongsByGenre but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_get_songs_by_genre() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "songsByGenre": {
      "song": [
        {
          "id": "93b9e7bafe4ecdddd9754024db69ef98",
          "parent": "4c35b5858998ba9f7beeb5412f4c51a1",
          "isDir": false,
          "title": "'54, '74, '90, 2010",
          "album": "MTV Unplugged In New York",
          "artist": "Sportfreunde Stiller",
          "track": 8,
          "year": 2009,
          "genre": "Pop",
          "coverArt": "mf-93b9e7bafe4ecdddd9754024db69ef98_647727b8",
          "size": 3724910,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 170,
          "bitRate": 168,
          "path": "Sportfreunde Stiller/MTV Unplugged In New York/08 - '54, '74, '90, 2010.mp3",
          "discNumber": 1,
          "created": "2023-05-31T11:51:07.367452963Z",
          "albumId": "4c35b5858998ba9f7beeb5412f4c51a1",
          "artistId": "e5f1d585330eb12e46f703ee1f64b695",
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
        if let crate::data::ResponseType::SongsByGenre { songs_by_genre } = response.data {
            assert_eq!(songs_by_genre.song.first().unwrap().year, Some(2009));
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
