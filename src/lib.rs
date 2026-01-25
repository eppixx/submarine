//! This is a pure rust implementation for the subsonic API
//!
//! API description of subsonic: <http://www.subsonic.org/pages/api.jsp><br>
//! xsd schema: <http://www.subsonic.org/pages/inc/api/schema/subsonic-rest-api-1.16.1.xsd><br>
//! This implements everything up to version v1.16.1.

/// Contains definition of API calls
pub(crate) mod api;
/// Constains structs for authenticating with a subsonic server
pub mod auth;
/// Contains definitions of return types of the subsonic REST API
pub mod data;

use log::{info, trace, warn};
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;

use crate::data::ResponseType;

/// A client which all requests are send through
///
/// Example:
/// ```no_run
/// use submarine::{auth::AuthBuilder, Client};
///
/// #[tokio::main]
/// async fn main() {
///     let auth = AuthBuilder::new("peter", "v0.16.1") // username and target version
///         .client_name("my_music_app")   // how your client will show up on the server
///         .hashed("change_me_password"); // create a hash from plain password
///     let client = Client::new("https://subsonic.example", auth); // creating a client
///     client.ping().await.unwrap();      // sending requests
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    server_url: String,
    auth: auth::Auth,
    client: reqwest::Client,
}

/// Errors which can occur while using subsonic
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
    #[error("Route not implemented on server: {0}")]
    ServerRouteNotImplemented(String),
    #[error("Url parsing error: {0}")]
    UrlParsing(#[from] url::ParseError),
}

#[derive(Debug, Serialize, Default, Clone, PartialEq, Eq)]
pub(crate) struct Parameter(Vec<(String, String)>);

impl Parameter {
    fn new() -> Self {
        Self(vec![])
    }

    fn push(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.push((key.into(), value.into()));
    }
}

impl Client {
    /// Creating a new client
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
        parameter: Option<Parameter>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<data::Response, SubsonicError> {
        let mut paras = parameter.unwrap_or_default();
        self.auth.add_parameter(&mut paras);
        let headers = headers.unwrap_or_default();

        trace!(
            "request from server: {}, para: {:?}, header: {:?}",
            path,
            paras,
            headers
        );
        let request = self
            .client
            .post(self.server_url.clone() + "/rest/" + path)
            .headers(headers)
            .query(&paras);

        let body: String = match request.send().await {
            Ok(body) => match body.status() {
                StatusCode::OK => body.text().await?,
                StatusCode::GONE
                    if body.text().await? == "This endpoint will not be implemented" =>
                {
                    return Err(SubsonicError::ServerRouteNotImplemented(String::from(
                        "navidrome",
                    )))
                }
                _ => {
                    warn!("Could not fetch previous request to {}", path);
                    return Err(SubsonicError::NoServerFound);
                }
            },
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let response: data::OuterResponse = serde_json::from_str(&body)?;
        match response.inner.data {
            ResponseType::Error { error } => Err(SubsonicError::Server(error.to_string())),
            _ => Ok(response.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType, Status};

    #[test]
    fn basic_conversion() {
        let response = r##"
            {
              "subsonic-response": {
                 "status":"ok",
                 "version":"1.16.1",
                 "type":"navidrome",
                 "serverVersion":"0.49.3 (8b93962f)"
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response)
            .unwrap()
            .inner;
        assert_eq!(response.info.status, Status::Ok);
    }

    #[test]
    fn convert_error() {
        let response = r##"
            {
              "subsonic-response": {
                "status":"failed",
                "version":"1.16.1",
                "type":"navidrome",
                "serverVersion":"0.49.3 (8b93962f)",
                "error": {
                  "code":40,
                  "message":"Wrong username or password"
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response)
            .unwrap()
            .inner;
        assert_eq!(response.info.status, Status::Error);
        if let ResponseType::Error { error } = response.data {
            assert_eq!(error.code, 40);
            assert_eq!(&error.message, "Wrong username or password");
        }
    }
}
