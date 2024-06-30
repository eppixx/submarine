use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#updatePlaylist
    pub async fn update_playlist(
        &self,
        playlist_id: impl Into<String>,
        name: Option<impl Into<String>>,
        comment: Option<impl Into<String>>,
        public: Option<bool>,
        songs_id_to_add: Vec<impl Into<String>>,
        songs_index_to_remove: Vec<i64>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("playlistId", playlist_id);
        if let Some(name) = name {
            paras.push("name", name);
        }
        if let Some(comment) = comment {
            paras.push("comment", comment);
        }
        if let Some(public) = public {
            paras.push("public", public.to_string());
        }
        for id in songs_id_to_add {
            paras.push("songIdToAdd", id);
        }
        for id in songs_index_to_remove {
            paras.push("songIndexToRemove", id.to_string());
        }

        let body = self.request("updatePlaylist", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
