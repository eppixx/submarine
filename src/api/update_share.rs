use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#updateShare
    pub async fn update_share(
        &self,
        id: impl Into<String>,
        description: Option<impl Into<String>>,
        expires: Option<usize>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());
        if let Some(desc) = description {
            paras.insert("description", desc.into());
        }
        if let Some(expires) = expires {
            paras.insert("expires", expires.to_string());
        }

        let body = self.request("updateShare", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
