use crate::{
    data::{ResponseType, Share},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createShare
    pub async fn create_share(
        &self,
        id: impl Into<String>,
        description: Option<impl Into<String>>,
        expires: Option<usize>,
    ) -> Result<Vec<Share>, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("id", id.into());
        if let Some(desc) = description {
            paras.insert("description", desc.into());
        }
        if let Some(expires) = expires {
            paras.insert("expires", expires.to_string());
        }

        let body = self.request("createShare", Some(paras), None).await?;
        if let ResponseType::Shares { shares } = body.data {
            Ok(shares.share)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Shares but found wrong type",
            )))
        }
    }
}

//TODO add test
