use super::{Client, SubsonicError};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Status {
    scanning: bool,
    count: Option<usize>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    scan_status: Status,
}

pub enum ScanStatus {
    /// server is currently scanning the library
    Scanning,
    /// current number of scan routines of the server; increments after scanning
    Count(usize),
}

impl Client {
    pub async fn get_scan_status(&self) -> Result<ScanStatus, SubsonicError> {
        let body = self.request("getScanStatus", None, None).await?;
        let response = serde_json::from_str::<Response>(&body)?;

        match response.scan_status {
            Status { scanning: true, .. } => Ok(ScanStatus::Scanning),
            Status {
                count: Some(value), ..
            } => Ok(ScanStatus::Count(value)),
            _ => Err(SubsonicError::Server("counting is none".into())),
        }
    }
}
