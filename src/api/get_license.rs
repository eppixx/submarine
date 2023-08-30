use crate::{
    data::{License, ResponseType},
    Client, SubsonicError,
};

impl Client {
    pub async fn get_license(&self) -> Result<License, SubsonicError> {
        let paras = std::collections::HashMap::new();

        let body = self.request("getLicense", Some(paras), None).await?;
        if let ResponseType::License { license } = body.data {
            Ok(license)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_license() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "license": {
                  "valid": true
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::License { license } = response.data {
            assert_eq!(license.valid, true);
        } else {
            panic!("wrong type");
        }
    }
}
