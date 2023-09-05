use crate::data::{ResponseType, ScanStatus};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#startScan
    pub async fn start_scan(&self) -> Result<ScanStatus, SubsonicError> {
        let paras: std::collections::HashMap<&str, String> = self.auth.clone().into();

        let body = self.request("startScan", Some(paras), None).await?;
        if let ResponseType::ScanStatus { scan_status } = body.data {
            Ok(scan_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}

//TODO add test
