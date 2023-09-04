use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#refreshPodcasts
    pub async fn refresh_podcasts(&self) -> Result<Info, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();

        let body = self.request("refreshPodcasts", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}

// TODO add test
