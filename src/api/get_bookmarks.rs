use crate::{
    data::{Bookmark, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getBookmarks
    pub async fn get_bookmarks(&self) -> Result<Vec<Bookmark>, SubsonicError> {
        let paras: std::collections::HashMap<&str, String> = self.auth.clone().into();

        let body = self.request("getBookmarks", Some(paras), None).await?;
        if let ResponseType::Bookmarks { bookmarks } = body.data {
            Ok(bookmarks.bookmark)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Bookmark but found wrong type",
            )))
        }
    }
}

//TODO add more test
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
}
