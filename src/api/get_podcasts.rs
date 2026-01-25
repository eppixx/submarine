use crate::{
    data::{PodcastChannel, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#getPodcasts>
    pub async fn get_podcasts(
        &self,
        include_episodes: Option<bool>,
        id: Option<impl Into<String>>,
    ) -> Result<Vec<PodcastChannel>, SubsonicError> {
        let mut paras = Parameter::new();
        if let Some(episodes) = include_episodes {
            paras.push("includeEpisodes", episodes.to_string());
        }
        if let Some(id) = id {
            paras.push("id", id);
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
