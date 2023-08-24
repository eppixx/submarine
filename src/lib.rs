/// api description of subsonic: http://www.subsonic.org/pages/api.jsp
/// xsd schema: http://www.subsonic.org/pages/inc/api/schema/subsonic-rest-api-1.16.1.xsd
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
pub mod star;
pub mod stream;
pub mod unstar;
pub mod update_playlist;

use log::{info, trace, warn};
use serde::Deserialize;
use thiserror::Error;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Client {
    server_url: String,
    auth: auth::Auth,
    client: reqwest::Client,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            server_url: self.server_url.clone(),
            auth: self.auth.clone(),
            client: self.client.clone(),
        }
    }
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

    pub async fn request(
        &self,
        path: &str,
        parameter: Option<HashMap<&str, String>>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<String, SubsonicError> {
        let parameter = parameter.unwrap_or_default();
        let headers = headers.unwrap_or_default();

        let mut paras: HashMap<&str, String> = self.auth.clone().into();
        for p in parameter {
            paras.insert(p.0, p.1);
        }

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

    ///on success sends server_name, username, hash and salt back
    pub async fn ping(&self) -> Result<(String, String, String, String), SubsonicError> {
        info!(
            "ping {}, {}, {}, {}",
            self.server_url, self.auth.user, self.auth.hash, self.auth.salt
        );
        let body = self.request("ping", None, None).await?;

        #[derive(Deserialize, Debug)]
        struct Error {
            code: i32,
            message: String,
        }

        #[derive(Deserialize, Debug)]
        struct Response {
            status: String,
            error: Option<Error>,
        }

        let json = serde_json::from_str::<Response>(&body)?;
        if json.status != "ok" {
            match json.error {
                None => unreachable!(),
                Some(e) => return Err(SubsonicError::Server(format!("{}, {}", e.code, e.message))),
            }
        }

        Ok((
            self.server_url.clone(),
            self.auth.user.clone(),
            self.auth.hash.clone(),
            self.auth.salt.clone(),
        ))
    }
}
