use crate::data::{Playlist, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#getPlaylists>
    pub async fn get_playlists(
        &self,
        user_name: Option<impl Into<String>>,
    ) -> Result<Vec<Playlist>, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        if let Some(name) = user_name {
            paras.insert("userName", name.into());
        }

        let body = self.request("getPlaylists", None, None).await?;
        if let ResponseType::Playlists { playlists } = body.data {
            Ok(playlists.playlist)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Playlists but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_playlists() {
        let response_body = r##"
        {
          "subsonic-response": {
            "status": "ok",
            "version": "1.16.1",
            "type": "navidrome",
            "serverVersion": "0.49.3 (8b93962f)",
            "playlists": {
              "playlist": [
                {
                  "id": "5ed9d5f9-2e23-46ae-9636-e62f5e595241",
                  "name": "The Ultimate Setlist",
                  "songCount": 18,
                  "duration": 4702,
                  "public": false,
                  "owner": "eppixx",
                  "created": "2022-07-27T09:53:37.420554962Z",
                  "changed": "2023-01-31T03:52:11Z",
                  "coverArt": "pl-5ee9d5d9-2e23-46ae-9636-e62f5e595241_63d8906b"
                },
                {
                  "id": "298d44b2-4000-4072-8815-8bd23fc445a1",
                  "name": "Playlist",
                  "songCount": 5,
                  "duration": 1009,
                  "public": false,
                  "owner": "eppixx",
                  "created": "2022-03-03T19:16:34.451528252Z",
                  "changed": "2023-01-31T03:52:11Z",
                  "coverArt": "pl-298b44d2-4000-4072-8815-8bd23fc445a1_63d8906b"
                }
              ]
            }
          }
        }"##;

        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Playlists { playlists } = response.data {
            assert_eq!(playlists.playlist.len(), 2);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
