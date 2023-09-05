use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#updateUser
    pub async fn update_user(
        &self,
        username: impl Into<String>,
        password: Option<impl Into<String>>, // plain or hex encoded
        email: Option<impl Into<String>>,
        ldap_authenticated: Option<bool>,
        admin_role: Option<bool>,
        settings_role: Option<bool>,
        stream_role: Option<bool>,
        jukebox_role: Option<bool>,
        download_role: Option<bool>,
        upload_role: Option<bool>,
        playlist_role: Option<bool>,
        cover_art_role: Option<bool>,
        comment_role: Option<bool>,
        podcast_role: Option<bool>,
        share_role: Option<bool>,
        video_conversion_role: Option<bool>,
        music_folder_id: Option<impl Into<String>>,
        max_bit_rate: Option<i16>,
    ) -> Result<Info, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("username", username.into());
        if let Some(password) = password {
            paras.insert("password", password.into());
        }
        if let Some(email) = email {
            paras.insert("email", email.into());
        }
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
        if let Some(bit_rate) = max_bit_rate {
            let valid_bit_rates = vec![
                0, 32, 40, 48, 56, 64, 80, 96, 116, 128, 160, 192, 224, 256, 320,
            ];
            if !valid_bit_rates.contains(&bit_rate) {
                return Err(SubsonicError::InvalidArgs(String::from("max_bit_rate")));
            }
            paras.insert("maxBitRate", bit_rate.to_string());
        }

        let body = self.request("updateUser", Some(paras), None).await?;
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
