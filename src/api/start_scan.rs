use crate::data::{ResponseType, ScanStatus};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#startScan>
    pub async fn start_scan(&self) -> Result<ScanStatus, SubsonicError> {
        let body = self.request("startScan", None, None).await?;
        if let ResponseType::ScanStatus { scan_status } = body.data {
            Ok(scan_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type ScanStatus but found wrong type",
            )))
        }
    }
}

//TODO add test
