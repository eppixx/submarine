use super::{Client, SubsonicError};

impl Client {
    pub async fn unstar_track(&self, id: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let _ = self.request("unstar", Some(paras), None).await?;

        Ok(())
    }

    pub async fn unstar_album(&self, id: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("albumId", String::from(id));

        let _ = self.request("unstar", Some(paras), None).await?;

        Ok(())
    }

    pub async fn unstar_artist(&self, id: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("artistId", String::from(id));

        let _ = self.request("unstar", Some(paras), None).await?;

        Ok(())
    }
}
