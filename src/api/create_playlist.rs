use crate::data::{PlaylistWithSongs, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    pub async fn create_playlist(
        &self,
        name: &str,
        tracks: Vec<String>,
    ) -> Result<PlaylistWithSongs, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("name", String::from(name));
        for id in tracks {
            paras.insert("songId", id);
        }

        let body = self.request("createPlaylist", Some(paras), None).await?;
        if let ResponseType::PlaylistWithSongs { playlist } = body.data {
            Ok(playlist)
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
    fn conversion_get_album() {
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
            assert_eq!(playlist.songs.len(), 0);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
