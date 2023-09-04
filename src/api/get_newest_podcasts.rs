use crate::{
    data::{PodcastEpisode, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getNewestPodcasts
    pub async fn get_newest_podcasts(
        &self,
        count: Option<i32>, //defaults to 20
    ) -> Result<Vec<PodcastEpisode>, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        if let Some(count) = count {
            paras.insert("count", count.to_string());
        }

        let body = self.request("getNewestPodcasts", Some(paras), None).await?;
        if let ResponseType::NewestPodcasts { newest_podcasts } = body.data {
            Ok(newest_podcasts.episode)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type NewestPodcasts but found wrong type",
            )))
        }
    }
}

// TODO add test
