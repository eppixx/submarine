use serde::Deserialize;

use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
pub struct NewPlaylist {
    pub id: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    playlist: NewPlaylist,
    // name: String,
    // song_count: i32,
    // duration: i32,
    // public: bool,
    // owner: String,
    // created:
    // changed:
}

impl Client {
    pub async fn create_playlist(
        &self,
        name: &str,
        tracks: Vec<String>,
    ) -> Result<NewPlaylist, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("name", String::from(name));
        for id in tracks {
            paras.insert("songId", id);
        }

        let body = self.request("createPlaylist", Some(paras), None).await?;
        let response = serde_json::from_str::<Response>(&body)?;

        Ok(response.playlist)
    }
}
