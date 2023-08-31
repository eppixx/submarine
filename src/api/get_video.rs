use crate::data::{Child, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getVideos
    pub async fn get_videos(&self) -> Result<Vec<Video>, SubsonicError> {
        let body = self.request("getVideos", None, None).await?;
        if let ResponseType::Videos { song } = body.data {
            Ok(*song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Videos but found wrong type",
            )))
        }
    }
}

//TODO add test
