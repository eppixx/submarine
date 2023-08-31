use crate::{
    data::{Genre, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// http://www.subsonic.org/pages/api.jsp#getGenres
    pub async fn get_genres(&self) -> Result<Vec<Genre>, SubsonicError> {
        let paras = std::collections::HashMap::new();

        let body = self.request("getGenres", Some(paras), None).await?;
        if let ResponseType::Genres { genres } = body.data {
            Ok(genres.genre)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Genres but found wrong type",
            )))
        }
    }
}

mod tests {
    #[test]
    fn conversion_get_indexes() {
        let response_body = r##"
            {
                "subsonic-response": {
                    "status": "ok",
                    "version": "1.16.1",
                    "type": "navidrome",
                    "serverVersion": "0.49.3 (8b93962f)",
                    "genres": {
                        "genre": [
                            {
                                "value": "Rock",
                                "songCount": 2860,
                                "albumCount": 211
                            },
                            {
                                "value": "Pop",
                                "songCount": 1336,
                                "albumCount": 87
                            }
                        ]
                    }
                }
            }"##;

        let response = serde_json::from_str::<crate::data::OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let crate::data::ResponseType::Genres { genres } = response.data {
            assert_eq!(genres.genre.first().unwrap().value, "Rock");
            assert_eq!(genres.genre.first().unwrap().song_count, Some(2860));
        } else {
            panic!("wrong type");
        }
    }
}
