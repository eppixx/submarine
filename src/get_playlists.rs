use serde::Deserialize;

use super::data::Playlist;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
struct MetaPlaylist {
    #[serde(default)] // when user does not have a playlist, returns empty Vec
    playlist: Vec<Playlist>,
}

#[derive(Deserialize, Debug)]
struct Response {
    playlists: MetaPlaylist,
}

impl Client {
    pub async fn get_playlists(&self) -> Result<Vec<Playlist>, SubsonicError> {
        let body = self.request("getPlaylists", None, None).await?;
        let json = serde_json::from_str::<Response>(&body)?;

        let playlists: Vec<Playlist> = json.playlists.playlist;

        Ok(playlists)
    }
}
