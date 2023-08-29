//! This is a pure rust implementation for the subsonic API
//!
//! API description of subsonic: <http://www.subsonic.org/pages/api.jsp><br>
//! xsd schema: <http://www.subsonic.org/pages/inc/api/schema/subsonic-rest-api-1.16.1.xsd><br>
//! This implements everything up to version v1.16.1.

pub mod auth;
pub mod data;
pub mod api;

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
    #[error("Connection error")]
    Connection(#[from] reqwest::Error),
    #[error("Conversion error")]
    Conversion(#[from] serde_json::Error),
    #[error("No server found")]
    NoServerFound,
    #[error("Server sends error: {0}")]
    Server(String),
    #[error("Submarine error: {0}")]
    Submarine(String),
    #[error("Submarine invalid arguments: {0}")]
    InvalidArgs(String),
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
    ) -> Result<data::Response, SubsonicError> {
        let parameter = parameter.unwrap_or_default();
        let headers = headers.unwrap_or_default();

        let mut paras: HashMap<&str, String> = self.auth.clone().into();
        parameter
            .iter()
            .for_each(|p| _ = paras.insert(p.0, p.1.into()));

        let request = self
            .client
            .post(self.server_url.clone() + "/rest/" + path)
            .headers(headers)
            .form(&paras);

        trace!("request from server: {}, para: {:?}", path, paras);
        let body: String = match request.send().await {
            Ok(body) => {
                if body.status() != 200 {
                    warn!("Could not fetch previous request to {}", path);
                    return Err(SubsonicError::NoServerFound);
                }
                body.text().await?
            }
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let response: data::OuterResponse = serde_json::from_str(&body)?;
        Ok(response.inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Error, OuterResponse, ResponseType};

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
