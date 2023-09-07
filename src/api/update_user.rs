use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
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
        let mut paras = Parameter::new();
        paras.push("username", username);
        if let Some(password) = password {
            paras.push("password", password);
        }
        if let Some(email) = email {
            paras.push("email", email);
        }
        if let Some(ldap) = ldap_authenticated {
            paras.push("ldapAuthenticated", ldap.to_string());
        }
        if let Some(admin) = admin_role {
            paras.push("adminRole", admin.to_string());
        }
        if let Some(settings) = settings_role {
            paras.push("settingsRole", settings.to_string());
        }
        if let Some(stream) = stream_role {
            paras.push("streamRole", stream.to_string());
        }
        if let Some(jukebox) = jukebox_role {
            paras.push("jukeboxRole", jukebox.to_string());
        }
        if let Some(download) = download_role {
            paras.push("downloadrole", download.to_string());
        }
        if let Some(upload) = upload_role {
            paras.push("uploadRole", upload.to_string());
        }
        if let Some(playlist) = playlist_role {
            paras.push("playlistRole", playlist.to_string());
        }
        if let Some(cover_art) = cover_art_role {
            paras.push("coverArtRole", cover_art.to_string());
        }
        if let Some(comment) = comment_role {
            paras.push("commentRole", comment.to_string());
        }
        if let Some(podcast) = podcast_role {
            paras.push("podcastRole", podcast.to_string());
        }
        if let Some(share) = share_role {
            paras.push("shareRole", share.to_string());
        }
        if let Some(video_conversion) = video_conversion_role {
            paras.push("videoConversionRole", video_conversion.to_string());
        }
        if let Some(folder) = music_folder_id {
            paras.push("musicFolderId", folder);
        }
        if let Some(bit_rate) = max_bit_rate {
            let valid_bit_rates = [
                0, 32, 40, 48, 56, 64, 80, 96, 116, 128, 160, 192, 224, 256, 320,
            ];
            if !valid_bit_rates.contains(&bit_rate) {
                return Err(SubsonicError::InvalidArgs(String::from("max_bit_rate")));
            }
            paras.push("maxBitRate", bit_rate.to_string());
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
