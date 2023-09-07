use crate::{
    data::{ResponseType, SearchResult3},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#search3
    pub async fn search3(
        &self,
        query: impl Into<String>,
        artist_count: Option<i32>,  // defaults to 20
        artist_offset: Option<i32>, // defaults to 0
        album_count: Option<i32>,   // defaults to 20
        album_offset: Option<i32>,  // defaults to 0
        song_count: Option<i32>,    // defaults to 20
        song_offset: Option<i32>,   // defaults to 0
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<SearchResult3, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("query", query);
        if let Some(artist) = artist_count {
            paras.push("artistCount", artist.to_string());
        }
        if let Some(artist) = artist_offset {
            paras.push("artistOffset", artist.to_string());
        }
        if let Some(album) = album_count {
            paras.push("albumCount", album.to_string());
        }
        if let Some(album) = album_offset {
            paras.push("albumOffset", album.to_string());
        }
        if let Some(song) = song_count {
            paras.push("songCount", song.to_string());
        }
        if let Some(song) = song_offset {
            paras.push("songOffset", song.to_string());
        }
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }

        let body = self.request("search3", Some(paras), None).await?;
        if let ResponseType::SearchResult3 { search_result3 } = body.data {
            Ok(search_result3)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type SearchResult2 but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_album() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "searchResult3": {
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
      ],
      "song": [
        {
          "id": "5476bbde93090258e2d91e8333163133",
          "parent": "c58e1cddef76983860e21603bc85c363",
          "isDir": false,
          "title": "6 O'Clock on a Tube Stop",
          "album": "Hi-Fi Serious",
          "artist": "A",
          "track": 3,
          "year": 2002,
          "coverArt": "mf-5476bbde93090258e2d91e8333163133_6113f6cd",
          "size": 4773069,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 194,
          "bitRate": 192,
          "path": "A/Hi-Fi Serious/03 - 6 O'Clock on a Tube Stop.mp3",
          "discNumber": 1,
          "created": "2021-08-11T16:33:47.650580818Z",
          "albumId": "c58e1cddef76983860e21603bc85c363",
          "artistId": "0cc175b9c0f1b6a831c399e269772661",
          "type": "music",
          "isVideo": false
        }
      ]
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::SearchResult3 { search_result3 } = response.data {
            assert_eq!(
                search_result3.song.first().unwrap().artist.as_deref(),
                Some("A")
            );
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
