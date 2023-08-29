use serde::Deserialize;

use super::data::Album;
use super::{Client, SubsonicError};

#[derive(Deserialize, Debug)]
struct AlbumList {
    _album: Option<Vec<Album>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    _album_list: AlbumList,
}

//TODO may remove
impl Client {
    pub async fn _get_albums(&self) -> Result<Vec<Album>, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("type", String::from("alphabeticalByName"));
        paras.insert("size", String::from("500")); //500 is maximum

        //cycle through pages of albums
        let mut i = 0;
        let mut result = vec![];
        loop {
            let mut paras = paras.clone();
            paras.insert("offset", format!("{}", i));

            let body = self
                .request("getAlbumList", Some(paras.clone()), None)
                .await?;
            let response = serde_json::from_str::<Response>(&body)?;
            if response._album_list._album.is_none() {
                break;
            }

            result.append(&mut response._album_list._album.unwrap_or_default());
            i += 500;
        }

        Ok(result)
    }
}
