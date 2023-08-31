use crate::data::{ResponseType, VideoInfo};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getVideoInfo
    pub async fn get_video_info(&self, id: impl Into<String>) -> Result<VideoInfo, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());

        let body = self.request("getVideoInfo", Some(paras), None).await?;
        if let ResponseType::VideoInfo { video_info } = body.data {
            Ok(video_info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type VideoInfo but found wrong type",
            )))
        }
    }
}

// TODO add test
