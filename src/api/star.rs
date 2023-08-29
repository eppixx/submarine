use std::fmt::Display;

use crate::{Client, SubsonicError};

/// parameter for the star method
pub enum StarTarget {
    SongOrFolder,
    Album,
    Artist,
}

impl Display for StarTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SongOrFolder => write!(f, "id"),
            Self::Album => write!(f, "albumId"),
            Self::Artist => write!(f, "artistId"),
        }
    }
}

impl Client {
    /// Quelle: http://www.subsonic.org/pages/api.jsp#star
    pub async fn star(&self, id: &str, target: StarTarget) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = target.to_string();
        paras.insert(key.as_str(), String::from(id));

        let _ = self.request("star", Some(paras), None).await?;
        Ok(())
    }
}
