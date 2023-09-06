use crate::{
    data::{Indexes, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getIndexes
    pub async fn get_indexes(
        &self,
        music_folder_id: Option<impl Into<String>>,
        if_modified_since: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    ) -> Result<Indexes, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        if let Some(folder) = music_folder_id {
            paras.insert("musicFolderId", folder.into());
        }
        if let Some(modified_date) = if_modified_since {
            paras.insert("ifModifiedSince", modified_date.to_string());
        }

        let body = self.request("getIndexes", Some(paras), None).await?;
        if let ResponseType::Indexes { indexes } = body.data {
            Ok(indexes)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Indexes but found wrong type",
            )))
        }
    }
}

#[cfg(all(test, not(feature = "navidrome")))]
mod tests {
    //TODO add test
}

#[cfg(all(test, feature = "navidrome"))]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_indexes() {
        let response_body = r##"
            {
                "subsonic-response": {
                    "status": "ok",
                    "version": "1.16.1",
                    "type": "navidrome",
                    "serverVersion": "0.49.3 (8b93962f)",
                    "indexes": {
                        "index": [
                            {
                                "name": "#",
                                "artist": [
                                    {
                                        "id": "5338af695f89024e8a57c08ccfe1b814",
                                        "name": "+44",
                                        "albumCount": 1,
                                        "coverArt": "ar-5338af695f89024e8a57c08ccfe1b814_0"
                                    }
                                ]
                            }
                        ]
                    }
                }
            }"##;

        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::Indexes { indexes } = response.data {
            assert_eq!(&indexes.index.first().unwrap().name, "#");
        } else {
            panic!("wrong type");
        }
    }
}
