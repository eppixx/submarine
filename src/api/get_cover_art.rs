use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getCoverArt
    pub fn get_cover_art_url(&self, id: impl Into<String>, size: Option<i32>) -> String {
        let mut paras = Parameter::new();
        paras.push("id", id);
        if let Some(size) = size {
            paras.push("size", size.to_string());
        }

        let mut url: String = self.server_url.clone() + "/rest/getCoverArt?";
        for p in paras.0 {
            url += &("&".to_owned() + &p.0 + "=" + &p.1);
        }

        url
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getCoverArt
    pub async fn get_cover_art(
        &self,
        id: impl Into<String>,
        size: Option<i32>,
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_cover_art_url(id, size)).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

//TODO add test
