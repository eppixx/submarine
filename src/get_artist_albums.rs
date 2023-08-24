use super::data::Album;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct _MetaArtist {
    _id: String,
    _name: String,
    _album: Vec<Album>,
}

#[derive(Deserialize, Debug)]
struct _Response {
    _artist: _MetaArtist,
}

//TODO may remove
impl super::Client {
    // pub async fn _get_artist_albums(&self, id: &str) -> Result<Vec<album::Album>, super::SubsonicError> {
    //     //input for request
    //     let mut paras = std::collections::HashMap::new();
    //     paras.insert("id", String::from(id));

    //     //send request
    //     let body = self.request("getArtist", Some(paras), None).await?;
    //     let json = serde_json::from_str::<_Response>(&body)?;

    //     //convert
    //     let albums = json
    //         ._artist
    //         ._album
    //         .iter()
    //         .map(|a| {
    //             let mut album = album::Album::from(a);
    //             album.set_artist_id(&json._artist._id);
    //             album.set_artist_title(&json._artist._name);
    //             album
    //         })
    //         .collect();
    //     Ok(albums)
    // }
}
