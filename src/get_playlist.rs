use serde::Deserialize;

use super::data::PlaylistWithSongs;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
struct Response {
    playlist: PlaylistWithSongs,
}

impl Client {
    pub async fn get_playlist(&self, id: &str) -> Result<PlaylistWithSongs, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", String::from(id));

        //send request
        let body = self.request("getPlaylist", Some(paras), None).await?;
        let json = serde_json::from_str::<Response>(&body)?;

        let playlist = json.playlist;
        Ok(playlist)
    }
}
