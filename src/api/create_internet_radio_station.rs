use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createInternetRadioStation
    pub async fn create_internet_radio_station(
        &self,
        stream_url: impl Into<String>,
        name: impl Into<String>,
        homepage_url: Option<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("streamUrl", stream_url.into());
        paras.insert("name", name.into());
        if let Some(url) = homepage_url {
            paras.insert("homepageUrl", url.into());
        }

        let body = self
            .request("createInternetRadioStation", Some(paras), None)
            .await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
