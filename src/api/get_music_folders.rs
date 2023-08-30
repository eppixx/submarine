use crate::{Client, data::{MusicFolders, ResponseType}, SubsonicError};


impl Client {
    pub async fn get_music_folders(&self) -> Result<MusicFolders, SubsonicError> {
        let paras = std::collections::HashMap::new();

        let body = self.request("getMusicFolders", Some(paras), None).await?;
        if let ResponseType::MusicFolders { music_folders } = body.data {
            Ok(music_folders)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type MusicFolders but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_music_folders() {
        let response_body = r##"
            {
                "subsonic-response": {
                    "status": "ok",
                    "version": "1.16.1",
                    "type": "navidrome",
                    "serverVersion": "0.49.3 (8b93962f)",
                    "musicFolders": {
                        "musicFolder": [
                            {
                                "id": 0,
                                "name": "Music Library"
                            }
                        ]
                    }
                }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::MusicFolders { music_folders } = response.data {
            assert_eq!(music_folders.music_folder.first().unwrap().id, 0);
        } else {
            panic!("wrong type");
        }
    }
}
