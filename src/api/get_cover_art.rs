use std::fmt::Write;

use crate::{Client, SubsonicError};

impl Client {
    pub fn get_cover_art_url(&self, id: &str) -> String {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("id", String::from(id));

        let mut url: String = self.server_url.clone() + "/rest/getCoverArt?";
        for p in paras {
            let _ = write!(&mut url, "&{}={}", p.0, p.1);
        }

        url
    }

    pub async fn get_cover_art(&self, id: &str) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_cover_art_url(id)).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}
