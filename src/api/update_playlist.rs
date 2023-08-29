use std::fmt::Display;

use crate::{Client, SubsonicError};

enum Keys {
    PlaylistId,
    Name,
    Comment,
    Public,
    SongIdToAdd,
    SongIndexToRemove,
}

impl Display for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlaylistId => write!(f, "playlistId"),
            Self::Name => write!(f, "name"),
            Self::Comment => write!(f, "comment"),
            Self::Public => write!(f, "public"),
            Self::SongIdToAdd => write!(f, "songIdToAdd"),
            Self::SongIndexToRemove => write!(f, "songIndexToRemove"),
        }
    }
}

impl Client {
    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_remove_multiple(
        &self,
        id: &str,
        range: std::ops::Range<usize>,
    ) -> Result<(), SubsonicError> {
        if range.is_empty() {
            return Ok(());
        }
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(id));
        let key = Keys::SongIndexToRemove.to_string();
        for i in range {
            paras.insert(key.as_str(), i.to_string());
        }

        let _ = self.request("updatePlaylist", Some(paras), None).await?;
        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_remove(
        &self,
        id: &str,
        index: usize,
    ) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(id));
        let key = Keys::SongIndexToRemove.to_string();
        paras.insert(key.as_str(), index.to_string());

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_append(
        &self,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(playlist_id));
        let key = Keys::SongIdToAdd.to_string();
        paras.insert(key.as_str(), String::from(track_id));

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_append_multiple(
        &self,
        playlist_id: &str,
        track_ids: &[&str],
    ) -> Result<(), SubsonicError> {
        if track_ids.is_empty() {
            return Ok(());
        }
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(playlist_id));
        let key = Keys::SongIdToAdd.to_string();
        for id in track_ids {
            paras.insert(key.as_str(), id.to_string());
        }

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_rename(&self, id: &str, name: &str) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(id));
        let key = Keys::Name.to_string();
        paras.insert(key.as_str(), String::from(name));

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_comment(
        &self,
        id: &str,
        comment: &str,
    ) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(id));
        let key = Keys::Comment.to_string();
        paras.insert(key.as_str(), String::from(comment));

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }

    /// Quelle: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist_public(
        &self,
        id: &str,
        public: bool,
    ) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = Keys::PlaylistId.to_string();
        paras.insert(key.as_str(), String::from(id));
        let key = Keys::Public.to_string();
        paras.insert(key.as_str(), public.to_string());

        let _ = self.request("updatePlaylist", Some(paras), None).await?;

        Ok(())
    }
}
