use log::info;
use serde::Deserialize;

use super::{Client, SubsonicError};

impl Client {
    ///on success sends server_name, username, hash and salt back
    pub async fn ping(&self) -> Result<(String, String, String, String), SubsonicError> {
        info!(
            "ping {}, {}, {}, {}",
            self.server_url, self.auth.user, self.auth.hash, self.auth.salt
        );
        let body = self.request("ping", None, None).await?;

        #[derive(Deserialize, Debug)]
        struct Response {
            status: String,
            error: Option<crate::data::Error>,
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

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Response {
        status: String,
        error: Option<crate::data::Error>,
    }

    #[test]
    fn ping_convert() {
        let response_txt = r##"{"status":"ok","version":"1.16.1","type":"navidrome","serverVersion":"0.49.3 (8b93962f)"}"##;
        let response = serde_json::from_str::<Response>(response_txt).unwrap();
        println!("{response:?}");
        assert_eq!(response.status, String::from("ok"));
    }

    #[test]
    fn convert_error() {
        let response_txt = r##"{"status":"failed","version":"1.16.1","type":"navidrome","serverVersion":"0.49.3 (8b93962f)","error":{"code":40,"message":"Wrong username or password"}}"##;
        let response = serde_json::from_str::<Response>(response_txt).unwrap();
        println!("{response:?}");
        assert_eq!(response.status, String::from("failed"));
        let error = response.error.unwrap();
        assert_eq!(error.code, 40);
        assert_eq!(error.message, String::from("Wrong username or password"));
    }
}
