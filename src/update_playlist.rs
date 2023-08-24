use super::{Client, SubsonicError};

impl Client {
    pub async fn update_playlist_remove_multiple(
        &self,
        id: &str,
        range: std::ops::Range<usize>,
    ) -> Result<(), SubsonicError> {
        if range.is_empty() {
            return Ok(());
        }
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", String::from(id));
        for i in range {
            paras.insert("songIndexToRemove", i.to_string());
        }

        let _ = self.request("updatePlaylist", Some(paras), None).await?;
        Ok(())
    }

    pub async fn update_playlist_remove(&self, id: &str, index: usize) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", String::from(id));
        paras.insert("songIndexToRemove", index.to_string());

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    pub async fn update_playlist_append(
        &self,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", String::from(playlist_id));
        paras.insert("songIdToAdd", String::from(track_id));

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    pub async fn update_playlist_append_multiple(
        &self,
        playlist_id: &str,
        track_ids: &[&str],
    ) -> Result<(), SubsonicError> {
        if track_ids.is_empty() {
            return Ok(());
        }
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", String::from(playlist_id));
        for id in track_ids {
            paras.insert("songIdToAdd", id.to_string());
        }

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    pub async fn update_playlist_rename(&self, id: &str, name: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", String::from(id));
        paras.insert("name", String::from(name));

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }
}
