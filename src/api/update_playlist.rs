use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
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
        let mut paras = std::collections::HashMap::new();
        paras.insert("playlistId", playlist_id.into());
        if let Some(name) = name {
            paras.insert("name", name.into());
        }
        if let Some(comment) = comment {
            paras.insert("comment", comment.into());
        }
        if let Some(public) = public {
            paras.insert("public", public.to_string());
        }
        for id in songs_id_to_add {
            paras.insert("songIdToAdd", id.into());
        }
        for id in songs_index_to_remove {
            paras.insert("songIndexToRemove", id.to_string());
        }

        let body = self.request("createPlaylist", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
