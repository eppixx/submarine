use crate::data::{Child, ResponseType};
use crate::{Client, SubsonicError, Parameter};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getSong
    pub async fn get_song(&self, id: impl Into<String>) -> Result<Child, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("getSong", Some(paras), None).await?;
        if let ResponseType::Song { song } = body.data {
            Ok(*song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Song but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_song() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "song": {
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
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Song { song } = response.data {
            assert_eq!(song.track, Some(13));
        } else {
            panic!("wrong type");
        }
    }
}
