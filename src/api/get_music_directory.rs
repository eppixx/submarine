use crate::{
    data::{Directory, ResponseType},
    Client, SubsonicError,
};

impl Client {
    pub async fn get_music_directory(
        &self,
        music_directory_id: String,
    ) -> Result<Directory, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("musicDirectoryId", music_directory_id);

        let body = self.request("getMusicDirectory", Some(paras), None).await?;
        if let ResponseType::MusicDirectory { directory } = body.data {
            Ok(directory)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type MusicDirectory but found wrong type",
            )))
        }
    }
}

//TODO add test
