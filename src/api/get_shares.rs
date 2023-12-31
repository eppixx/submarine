use crate::{
    data::{ResponseType, Share},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getShares
    pub async fn get_shares(&self) -> Result<Vec<Share>, SubsonicError> {
        let body = self.request("getShares", None, None).await?;
        if let ResponseType::Shares { shares } = body.data {
            Ok(shares.share)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Shares but found wrong type",
            )))
        }
    }
}

//TODO test
