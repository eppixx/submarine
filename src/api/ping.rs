use log::info;

use crate::{data::Info, Client, SubsonicError};

impl Client {
    /// pings server and sends its [Info]
    pub async fn ping(&self) -> Result<Info, SubsonicError> {
        info!(
            "ping {}, {}, {}, {}",
            self.server_url, self.auth.user, self.auth.hash, self.auth.salt
        );
        let body = self.request("ping", None, None).await?;
        Ok(body.info)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn ping_convert() {
        let response_txt = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)"
              }
            }"##;
        let info = serde_json::from_str::<OuterResponse>(response_txt)
            .unwrap()
            .inner
            .info;
        assert_eq!(info.status, String::from("ok"));
        assert_eq!(info.version, String::from("1.16.1"));
        assert_eq!(info.r#type, Some(String::from("navidrome")));
    }

    #[test]
    fn convert_error() {
        let response_txt = r##"
            {
              "subsonic-response": {
                "status": "failed",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "error": {
                  "code": 40,
                  "message": "Wrong username or password"
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_txt)
            .unwrap()
            .inner;
        assert_eq!(response.info.status, String::from("failed"));
        assert_eq!(response.info.version, String::from("1.16.1"));
        if let ResponseType::Error { error } = response.data {
            assert_eq!(error.code, 40);
            assert_eq!(error.message, String::from("Wrong username or password"));
        } else {
            panic!("wrong type");
        }
    }
}
