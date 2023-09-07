use crate::data::{ResponseType, Starred};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getStarred2
    pub async fn get_starred2(
        &self,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Starred, SubsonicError> {
        let mut paras = Parameter::new();
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }

        let body = self.request("getStarred", Some(paras), None).await?;
        if let ResponseType::Starred { starred } = body.data {
            Ok(starred)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Starred but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_starred() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "starred": {
      "artist": [
        {
          "id": "dcb01bc8072f079f20a1b9362c85e78f",
          "name": "Fox Stevenson",
          "albumCount": 5,
          "starred": "2023-08-22T13:58:56Z",
          "coverArt": "ar-dcb01bc8072f079f20a1b9362c85e78f_0"
        }
      ],
      "album": [
        {
          "id": "c686e803911d72cf98bd077c72326759",
          "parent": "f97bc375b1576bdfd1116641304c4cf0",
          "isDir": true,
          "title": "Voodooism the Sequel",
          "name": "Voodooism the Sequel",
          "album": "Voodooism the Sequel",
          "artist": "cYsmix",
          "year": 2014,
          "coverArt": "al-c686e803911d72cf98bd077c72326759_6476fade",
          "starred": "2023-06-24T10:30:05Z",
          "duration": 4846,
          "playCount": 54,
          "played": "2023-08-30T11:19:10Z",
          "created": "2023-05-31T07:51:00.680123782Z",
          "artistId": "f97bc375b1576bdfd1116641304c4cf0",
          "songCount": 9,
          "isVideo": false
        }
      ],
      "song": [
        {
          "id": "99c094f817132dba6efd704ff8cd8591",
          "parent": "fb6471038781aec682de3e9baae984af",
          "isDir": false,
          "title": "Inner",
          "album": "Polarity",
          "artist": "fusq",
          "track": 2,
          "year": 2017,
          "genre": "Electronic",
          "coverArt": "mf-99c094f817132dba6efd704ff8cd8591_64c77eec",
          "size": 5277285,
          "contentType": "audio/mpeg",
          "suffix": "mp3",
          "starred": "2023-08-06T10:09:21Z",
          "duration": 326,
          "bitRate": 128,
          "path": "fusq/Polarity/02 - Inner.mp3",
          "playCount": 9,
          "played": "2023-08-27T21:07:23Z",
          "discNumber": 1,
          "created": "2023-07-31T10:27:22.451379914Z",
          "albumId": "fb6471038781aec682de3e9baae984af",
          "artistId": "896c06fd74161314241790d9372657eb",
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
        if let crate::data::ResponseType::Starred { starred } = response.data {
            assert_eq!(starred.artist.first().unwrap().name, "Fox Stevenson");
            assert_eq!(
                starred.album.first().unwrap().artist.as_deref(),
                Some("cYsmix")
            );
            assert_eq!(starred.song.first().unwrap().title, "Inner");
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
