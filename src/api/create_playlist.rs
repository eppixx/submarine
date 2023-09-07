use crate::data::{PlaylistWithSongs, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createPlaylist
    pub async fn create_playlist(
        &self,
        name: impl Into<String>,
        song_id: Vec<impl Into<String>>,
    ) -> Result<PlaylistWithSongs, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("name", name);
        for id in song_id {
            paras.push("songId", id);
        }

        let body = self.request("createPlaylist", Some(paras), None).await?;
        if let ResponseType::PlaylistWithSongs { playlist } = body.data {
            Ok(playlist)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type PlaylistWithSongs but found wrong type",
            )))
        }
    }

    pub async fn overwrite_playlist(
        &self,
        playlist_id: impl Into<String>,
        song_id: Vec<impl Into<String>>,
    ) -> Result<PlaylistWithSongs, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("playlistId", playlist_id.into());
        for id in song_id {
            paras.push("songId", id.into());
        }

        let body = self.request("createPlaylist", Some(paras), None).await?;
        if let ResponseType::PlaylistWithSongs { playlist } = body.data {
            Ok(playlist)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type PlaylistWithSongs but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_create_playlist() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "playlist": {
                  "id": "7d6f3c43-cb3a-4195-b437-a9e1429a8a22",
                  "name": "create_playlist",
                  "songCount": 0,
                  "duration": 0,
                  "public": false,
                  "owner": "eppixx",
                  "created": "2023-08-29T16:18:09.131163859Z",
                  "changed": "2023-08-29T16:18:09.131163907Z",
                  "coverArt": "pl-7d6f3c43-cb3a-4195-b437-a9e1429a8a22_64ee1a41"
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::PlaylistWithSongs { playlist } = response.data {
            assert_eq!(playlist.entry.len(), 0);
            assert_eq!(playlist.base.duration, 0);
        } else {
            panic!("wrong type: {response:?}");
        }
    }

    #[test]
    fn conversion_overwrite_playlist() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "playlist": {
      "id": "0ffa92dd-7927-43df-b97d-5352cbd50aa8",
      "name": "test",
      "songCount": 1,
      "duration": 140,
      "public": false,
      "owner": "eppixx",
      "created": "2023-09-01T12:51:08.099027452Z",
      "changed": "2023-09-05T23:16:58Z",
      "coverArt": "pl-0ffa92dd-7927-43df-b97d-5352cbd50aa8_64f7b6ea",
      "entry": [
        {
          "id": "7f4afc229780891f5a8aa2b9415a7e0f",
          "parent": "70d4e438ad35fb648ed673875f1dc09a",
          "isDir": false,
          "title": "Santa Claus Is Coming to Town",
          "album": "Rock Christmas",
          "artist": "Ella Fitzgerald",
          "track": 13,
          "year": 1991,
          "coverArt": "mf-7f4afc229780891f5a8aa2b9415a7e0f_61151f62",
          "size": 3617603,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "duration": 140,
          "bitRate": 200,
          "path": "Various Artists/Rock Christmas/13 - Santa Claus Is Coming to Town.mp3",
          "playCount": 3,
          "played": "2022-12-05T15:54:34Z",
          "discNumber": 1,
          "created": "2021-08-12T13:33:57.283852964Z",
          "albumId": "70d4e438ad35fb648ed673875f1dc09a",
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
        if let ResponseType::PlaylistWithSongs { playlist } = response.data {
            assert_eq!(playlist.entry.len(), 1);
            assert_eq!(playlist.entry.first().unwrap().bit_rate, Some(200));
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
