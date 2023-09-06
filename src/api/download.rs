use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#download
    pub fn download_url(&self, id: impl Into<String>) -> String {
        format!("{}/rest/download?{}", self.server_url, id.into())
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#download
    pub async fn download(&self, id: impl Into<String>) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.download_url(id)).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

//TODO add test
