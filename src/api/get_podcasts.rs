use crate::{
    data::{PodcastChannel, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getPodcasts
    pub async fn get_podcasts(
        &self,
        include_episodes: Option<bool>,
        id: Option<impl Into<String>>,
    ) -> Result<Vec<PodcastChannel>, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        if let Some(episodes) = include_episodes {
            paras.insert("includeEpisodes", episodes.to_string());
        }
        if let Some(id) = id {
            paras.insert("id", id.into());
        }

        let body = self.request("getPodcasts", Some(paras), None).await?;
        if let ResponseType::Podcasts { podcasts } = body.data {
            Ok(podcasts.channel)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Podcasts but found wrong type",
            )))
        }
    }
}

// TODO add test
