use crate::data::{ResponseType, ScanStatus};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getScanStatus
    pub async fn get_scan_status(&self) -> Result<ScanStatus, SubsonicError> {
        let paras = std::collections::HashMap::new();

        let body = self.request("getScanStatus", Some(paras), None).await?;
        if let ResponseType::ScanStatus { scan_status } = body.data {
            Ok(scan_status)
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
    fn conversion_get_scan_status() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "scanStatus": {
                  "scanning": false,
                  "count": 18352,
                  "folderCount": 1205,
                  "lastScan": "2023-08-29T21:38:07.001850244Z"
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::ScanStatus { scan_status } = response.data {
            assert_eq!(scan_status.scanning, false);
            assert_eq!(scan_status.count, Some(18352));
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
