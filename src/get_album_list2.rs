use serde::Deserialize;

use super::data::Album;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
struct MetaAlbumList2 {
    album: Option<Vec<Album>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    album_list2: MetaAlbumList2,
    // status: String,
    // version: String,
}

impl Client {
    pub async fn get_album_list2(&self) -> Result<Vec<Album>, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("type", String::from("alphabeticalByArtist"));
        paras.insert("size", String::from("500")); //500 is maximum

        let mut i = 0;
        let mut result = vec![];
        loop {
            let mut paras = paras.clone();
            paras.insert("offset", format!("{}", i));

            let body = self.request("getAlbumList2", Some(paras.clone()), None).await?;
            let response =
                serde_json::from_str::<Response>(&body)?;
            if response.album_list2.album.is_none() {
                break;
            }

            result.append(&mut response.album_list2.album.unwrap_or_default());
            i += 500;
        }

        Ok(result)
    }
}
