use super::data::Child;
use super::{Client, SubsonicError};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    song: Child,
}

impl Client {
    pub async fn get_track(&self, id: &str) -> Result<Child, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        let body = self.request("getSong", Some(paras), None).await?;
        let response = serde_json::from_str::<Response>(&body)?;

        Ok(response.song)
    }
}
