use crate::data::{PlaylistWithSongs, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getPlaylist
    pub async fn get_playlist(&self, id: impl Into<String>) -> Result<PlaylistWithSongs, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());

        // send request
        let body = self.request("getPlaylist", Some(paras), None).await?;
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
    fn conversion_get_playlist() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "playlist": {
                  "id": "298b44b2-4000-4072-8815-8bd23fc445a1",
                  "name": "andere Playlist",
                  "songCount": 2,
                  "duration": 1009,
                  "public": false,
                  "owner": "eppixx",
                  "created": "2022-03-03T19:16:34.451528252Z",
                  "changed": "2023-01-31T03:52:11Z",
                  "coverArt": "pl-298b44b2-4000-4072-8815-8bd23fc445a1_63d8906b",
                  "entry": [
                    {
                      "id": "edfd5eee9a438816e184e9a0312a5dec",
                      "parent": "d83cf43ca2d2acdd016e109ef17773ea",
                      "isDir": false,
                      "title": "Ascension",
                      "album": "100% No Feeble Cheering",
                      "artist": "Ark",
                      "track": 17,
                      "year": 2012,
                      "coverArt": "mf-edfd5eee9a438816e184e9a0312a5dec_60fbf745",
                      "size": 14639292,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "starred": "2022-01-02T02:11:32Z",
                      "duration": 447,
                      "bitRate": 260,
                      "path": "Balloon Party/100% No Feeble Cheering/17 - Ascension.mp3",
                      "playCount": 3,
                      "played": "2022-12-03T10:23:17Z",
                      "discNumber": 1,
                      "created": "2021-07-24T12:06:17.638197466Z",
                      "albumId": "d83cf43ca2d2acdd016e109ef17773ea",
                      "type": "music",
                      "isVideo": false
                    },
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
        println!("{response:?}");
        if let ResponseType::PlaylistWithSongs { playlist } = response.data {
            assert_eq!(playlist.base.song_count, 2);
        } else {
            panic!("wrong type");
        }
    }
}
