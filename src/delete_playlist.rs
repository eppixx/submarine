use crate::{Client, SubsonicError};

impl Client {
    pub async fn delete_playlist(&self, id: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let _ = self.request("deletePlaylist", Some(paras), None).await?;

        Ok(())
    }
}
