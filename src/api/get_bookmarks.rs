use crate::{
    data::{Bookmark, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getBookmarks
    pub async fn get_bookmarks(&self) -> Result<Vec<Bookmark>, SubsonicError> {
        let body = self.request("getBookmarks", None, None).await?;
        if let ResponseType::Bookmarks { bookmarks } = body.data {
            Ok(bookmarks.bookmark)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Bookmark but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_empty_get_bookmarks() {
        let response_body = r##"
{
    "subsonic-response": {
        "status": "ok",
        "version": "1.16.1",
        "type": "navidrome",
        "serverVersion": "0.49.3 (8b93962f)",
        "bookmarks": {}
    }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Bookmarks { bookmarks } = response.data {
            assert_eq!(bookmarks.bookmark.len(), 0);
        } else {
            panic!("wrong type");
        }
    }

    #[test]
    fn conversion_get_bookmarks() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "bookmarks": {
      "bookmark": [
        {
          "entry": {
            "id": "7f4afc229780891f5a8aa2b9415a7e0f",
            "parent": "70d4e438ad35fb648ed673875f1dc09a",
            "isDir": false,
            "title": "Santa Claus Is Coming to Town",
            "album": "Rock Christmas",
            "artist": "Ella Fitzgerald",
            "track": 13,
            "year": 1991,
            "coverArt": "mf-7f4afc229780891f5a8aa2b9415a7e0f_64f7569f",
            "size": 3617603,
            "contentType": "audio/mpeg",
            "suffix": "mp3",
            "duration": 140,
            "bitRate": 200,
            "path": "Various Artists/Rock Christmas/13 - Santa Claus Is Coming to Town.mp3",
            "playCount": 3,
            "played": "2022-12-05T15:54:34Z",
            "discNumber": 1,
            "created": "2023-09-05T16:26:07Z",
            "albumId": "70d4e438ad35fb648ed673875f1dc09a",
            "type": "music",
            "isVideo": false,
            "bookmarkPosition": 5000
          },
          "position": 5000,
          "username": "eppixx",
          "comment": "test bookmark",
          "created": "2023-09-05T16:26:07Z",
          "changed": "2023-09-05T16:26:07Z"
        }
      ]
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Bookmarks { bookmarks } = response.data {
            assert_eq!(bookmarks.bookmark.first().unwrap().entry.track, Some(13));
            assert_eq!(bookmarks.bookmark.first().unwrap().position, 5000);
        } else {
            panic!("wrong type");
        }
    }
}
