use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getCatptions
    pub fn get_captions_url(
        &self,
        id: impl Into<String>,
        format: Option<impl Into<String>>,
    ) -> String {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());
        if let Some(format) = format {
            paras.insert("format", format.into());
        }

        let mut url: String = self.server_url.clone() + "/rest/getCaptions?";
        for p in paras {
            url += &("&".to_owned() + p.0 + "=" + &p.1);
        }

        url
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getCatptions
    pub async fn get_captions(
        &self,
        id: impl Into<String>,
        format: Option<impl Into<String>>,
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.get_captions_url(id, format)).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}
