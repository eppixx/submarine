use serde::Deserialize;

use super::data::Child;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
pub struct MetaAlbum {
    #[serde(flatten)]
    pub info: super::data::Album,
    pub song: Vec<Child>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    album: MetaAlbum,
}

impl Client {
    pub async fn get_album(&self, id: &str) -> Result<MetaAlbum, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let body = self.request("getAlbum", Some(paras), None).await?;
        let response = serde_json::from_str::<Response>(&body)?;

        Ok(response.album)
    }
}
