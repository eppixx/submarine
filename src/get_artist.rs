use serde::Deserialize;

use super::data::Album;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
pub struct MetaArtist {
    #[serde(flatten)]
    pub info: super::data::Artist,
    pub album: Vec<Album>,
}

#[derive(Deserialize, Debug)]
struct Response {
    artist: MetaArtist,
}

impl Client {
    pub async fn get_artist(&self, id: &str) -> Result<MetaArtist, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let body = self.request("getArtist", Some(paras), None).await?;
        let response = serde_json::from_str::<Response>(&body)?;

        Ok(response.artist)
    }
}
