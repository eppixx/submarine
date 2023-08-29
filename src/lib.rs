//! This is a pure rust implementation for the subsonic API
//!
//! API description of subsonic: <http://www.subsonic.org/pages/api.jsp><br>
//! xsd schema: <http://www.subsonic.org/pages/inc/api/schema/subsonic-rest-api-1.16.1.xsd><br>
//! This implements everything up to version v1.16.1.

pub mod auth;
pub mod create_playlist;
pub mod data;
pub mod delete_playlist;
pub mod get_album;
pub mod get_album_list2;
pub mod get_albums;
pub mod get_artist;
pub mod get_artist_albums;
pub mod get_artists;
pub mod get_cover_art;
pub mod get_playlist;
pub mod get_playlists;
pub mod get_scan_status;
pub mod get_song;
pub mod ping;
pub mod star;
pub mod stream;
pub mod unstar;
pub mod update_playlist;

use log::{info, trace, warn};
use thiserror::Error;

use std::collections::HashMap;


/// A client which all requests are send through.<br>
/// Example
/// ```no_run
/// #[tokio::main]
/// async fn main() {
///     use submarine::{auth::AuthBuilder, Client};
///
///     let auth = AuthBuilder::new("peter", "v0.16.1")
///         .client_name("my_music_app")
///         .hashed("change_me_password");
///     let client = Client::new("https://target.com", auth);
///     client.ping().await.unwrap();
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    server_url: String,
    auth: auth::Auth,
    client: reqwest::Client,
}

#[derive(Error, Debug)]
pub enum SubsonicError {
    #[error("Verbindugsfehler")]
    Connection(#[from] reqwest::Error),
    #[error("Konvertierungsfehler")]
    Conversion(#[from] serde_json::Error),
    #[error("Kein Server gefunden")]
    NoServerFound,
    #[error("Server sendet den Fehler: {0}")]
    Server(String),
}

impl Client {
    pub fn new(server_url: &str, auth: auth::Auth) -> Self {
        let client = Self {
            server_url: String::from(server_url),
            auth,
            client: reqwest::Client::new(),
        };
        info!("created client {:?}", client);
        client
    }

    pub(crate) async fn request(
        &self,
        path: &str,
        parameter: Option<HashMap<&str, String>>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<String, SubsonicError> {
        let parameter = parameter.unwrap_or_default();
        let headers = headers.unwrap_or_default();

        let mut paras: HashMap<&str, String> = self.auth.clone().into();
        parameter.iter().for_each(|p| _ = paras.insert(p.0, p.1.into()));

        let request = self
            .client
            .post(self.server_url.clone() + "/rest/" + path)
            .headers(headers)
            .form(&paras);

        trace!("request from server: {}, para: {:?}", path, paras);
        let body = match request.send().await {
            Ok(body) => {
                if body.status() != 200 {
                    warn!("Could not fetch previous request to {}", path);
                    return Err(SubsonicError::NoServerFound);
                }
                body.text().await?
            }
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        //remove subsonic-response
        //TODO find better solution than string -> json -> string -> json
        let json: serde_json::Value = match serde_json::from_str(&body) {
            Ok(k) => k,
            Err(e) => return Err(SubsonicError::Conversion(e)),
        };

        let ser = serde_json::to_string(&json["subsonic-response"])?;

        Ok(ser)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::{Error, ResponseType, OuterResponse},
    };

    #[test]
    fn basic_conversion() {
        let oracle = vec![
            (
                ResponseType::Ping {},
                r##"
                {
                    "subsonic-response":{
                         "status":"ok",
                         "version":"1.16.1",
                         "type":"navidrome",
                         "serverVersion":"0.49.3 (8b93962f)"
                    }
                }"##,
            ),
            (
                ResponseType::Error {
                    error: Error {
                        code: 40,
                        message: String::from("Wrong username or password"),
                    },
                },
                r##"
                {
                    "subsonic-response":{
                        "status":"failed",
                        "version":"1.16.1",
                        "type":"navidrome",
                        "serverVersion":"0.49.3 (8b93962f)",
                        "error":{
                            "code":40,
                            "message":"Wrong username or password"
                        }
                    }
                }"##,
            ),
        ];

        for (target, response_body) in oracle {
            let response = serde_json::from_str::<OuterResponse>(response_body)
                .unwrap()
                .inner;
            assert_eq!(target, response.data);
        }
    }
}
