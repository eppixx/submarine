use serde::Deserialize;

use super::data::Artist;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
struct Category {
    // _name: String,
    artist: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MetaArtists {
    index: Vec<Category>,
    // last_modified: i64,
    // ignored_articles: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    artists: MetaArtists,
}

impl Client {
    pub async fn get_artists(&self) -> Result<Vec<Artist>, SubsonicError> {
        let body = self.request("getArtists", None, None).await?;
        let json = serde_json::from_str::<Response>(&body)?;

        //flatten response to Vec<Artist>
        let artists: Vec<Artist> = json
            .artists
            .index
            .iter()
            .flat_map(|i| i.artist.clone())
            .collect();

        Ok(artists)
    }
}
