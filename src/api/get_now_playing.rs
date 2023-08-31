use crate::data::{NowPlaying, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getNowPlaying
    pub async fn get_now_playing(
        &self,
    ) -> Result<NowPlaying, SubsonicError> {
        let body = self.request("getNowPlaying",None, None).await?;
        if let ResponseType::NowPlaying { now_playing } = body.data {
            Ok(now_playing)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type NowPlaying but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_now_playing() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "nowPlaying": {
      "entry": [
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
          "isVideo": false,
          "username": "eppixx",
          "minutesAgo": 0,
          "playerId": 1,
          "playerName": "NavidromeUI"
        }
      ]
    }
  }
}"##;
        let response = serde_json::from_str::<crate::data::OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let crate::data::ResponseType::NowPlaying { now_playing } = response.data {
            assert_eq!(now_playing.entry.first().unwrap().child.album.as_deref(), Some("Showdown"));
            assert_eq!(now_playing.entry.first().unwrap().username, "eppixx");
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
