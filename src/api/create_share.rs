use crate::{
    data::{ResponseType, Share},
    Client, SubsonicError, Parameter,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createShare
    pub async fn create_share(
        &self,
        id: impl Into<String>,
        description: Option<impl Into<String>>,
        expires: Option<usize>,
    ) -> Result<Vec<Share>, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);
        if let Some(desc) = description {
            paras.push("description", desc);
        }
        if let Some(expires) = expires {
            paras.push("expires", expires.to_string());
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
