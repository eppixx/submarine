use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#createUser
    pub async fn create_user(
        &self,
        username: impl Into<String>,
        password: impl Into<String>, // plain or hex encoded
        email: impl Into<String>,
        ldap_authenticated: Option<bool>,    // defaults to false
        admin_role: Option<bool>,            // defaults to false
        settings_role: Option<bool>,         // defaults to true
        stream_role: Option<bool>,           // defaults to true
        jukebox_role: Option<bool>,          // defaults to false
        download_role: Option<bool>,         // defaults to false
        upload_role: Option<bool>,           // defults to false
        playlist_role: Option<bool>,         // /defaults to false
        cover_art_role: Option<bool>,        // /defaults to false
        comment_role: Option<bool>,          // /defaults to false
        podcast_role: Option<bool>,          // /defaults to false
        share_role: Option<bool>,            // /defaults to false
        video_conversion_role: Option<bool>, // /defaults to false
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("username", username.into());
        paras.insert("password", password.into());
        paras.insert("email", email.into());
        if let Some(ldap) = ldap_authenticated {
            paras.insert("ldapAuthenticated", ldap.to_string());
        }
        if let Some(admin) = admin_role {
            paras.insert("adminRole", admin.to_string());
        }
        if let Some(settings) = settings_role {
            paras.insert("settingsRole", settings.to_string());
        }
        if let Some(stream) = stream_role {
            paras.insert("streamRole", stream.to_string());
        }
        if let Some(jukebox) = jukebox_role {
            paras.insert("jukeboxRole", jukebox.to_string());
        }
        if let Some(download) = download_role {
            paras.insert("downloadrole", download.to_string());
        }
        if let Some(upload) = upload_role {
            paras.insert("uploadRole", upload.to_string());
        }
        if let Some(playlist) = playlist_role {
            paras.insert("playlistRole", playlist.to_string());
        }
        if let Some(cover_art) = cover_art_role {
            paras.insert("coverArtRole", cover_art.to_string());
        }
        if let Some(comment) = comment_role {
            paras.insert("commentRole", comment.to_string());
        }
        if let Some(podcast) = podcast_role {
            paras.insert("podcastRole", podcast.to_string());
        }
        if let Some(share) = share_role {
            paras.insert("shareRole", share.to_string());
        }
        if let Some(video_conversion) = video_conversion_role {
            paras.insert("videoConversionRole", video_conversion.to_string());
        }
        if let Some(folder) = music_folder_id {
            paras.insert("musicFolderId", folder.into());
        }

        let body = self.request("createUser", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}

//TODO add test
