use super::{Client, SubsonicError};
use crate::data::{ResponseType, ScanStatus};

impl Client {
    pub async fn get_scan_status(&self) -> Result<ScanStatus, SubsonicError> {
        let body = self.request("getScanStatus", None, None).await?;
        if let ResponseType::ScanStatus { status } = body.data {
            Ok(status)
            // match status {
            //     ScanStatus::Status { scanning: true, .. } => Ok(ScanStatus::Scanning),
            //     Status {
            //         count: Some(value), ..
            //     } => Ok(ScanStatus::Count(value)),
            //     _ => Err(SubsonicError::Server("counting is none".into())),
            // }
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}
