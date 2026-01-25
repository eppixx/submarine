use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

use super::create_user::Roles;

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#updateUser>
    pub async fn update_user(
        &self,
        username: impl Into<String>,
        password: Option<impl Into<String>>, // plain or hex encoded
        email: Option<impl Into<String>>,
        roles: Option<Roles>,
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
        if let Some(roles) = roles {
            if let Some(ldap) = roles.ldap_authenticated {
                paras.push("ldapAuthenticated", ldap.to_string());
            }
            if let Some(admin) = roles.admin_role {
                paras.push("adminRole", admin.to_string());
            }
            if let Some(settings) = roles.settings_role {
                paras.push("settingsRole", settings.to_string());
            }
            if let Some(stream) = roles.stream_role {
                paras.push("streamRole", stream.to_string());
            }
            if let Some(jukebox) = roles.jukebox_role {
                paras.push("jukeboxRole", jukebox.to_string());
            }
            if let Some(download) = roles.download_role {
                paras.push("downloadrole", download.to_string());
            }
            if let Some(upload) = roles.upload_role {
                paras.push("uploadRole", upload.to_string());
            }
            if let Some(playlist) = roles.playlist_role {
                paras.push("playlistRole", playlist.to_string());
            }
            if let Some(cover_art) = roles.cover_art_role {
                paras.push("coverArtRole", cover_art.to_string());
            }
            if let Some(comment) = roles.comment_role {
                paras.push("commentRole", comment.to_string());
            }
            if let Some(podcast) = roles.podcast_role {
                paras.push("podcastRole", podcast.to_string());
            }
            if let Some(share) = roles.share_role {
                paras.push("shareRole", share.to_string());
            }
            if let Some(video_conversion) = roles.video_conversion_role {
                paras.push("videoConversionRole", video_conversion.to_string());
            }
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
