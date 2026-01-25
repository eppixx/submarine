use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Roles {
    pub(crate) ldap_authenticated: Option<bool>, // defaults to false
    pub(crate) admin_role: Option<bool>,         // defaults to false
    pub(crate) settings_role: Option<bool>,      // defaults to true
    pub(crate) stream_role: Option<bool>,        // defaults to true
    pub(crate) jukebox_role: Option<bool>,       // defaults to false
    pub(crate) download_role: Option<bool>,      // defaults to false
    pub(crate) upload_role: Option<bool>,        // defaults to false
    pub(crate) playlist_role: Option<bool>,      // /defaults to false
    pub(crate) cover_art_role: Option<bool>,     // /defaults to false
    pub(crate) comment_role: Option<bool>,       // /defaults to false
    pub(crate) podcast_role: Option<bool>,       // /defaults to false
    pub(crate) share_role: Option<bool>,         // /defaults to false
    pub(crate) video_conversion_role: Option<bool>, // /defaults to false
}

impl Roles {
    pub fn ldap_authenticated(&mut self, ldap: bool) {
        self.ldap_authenticated = Some(ldap);
    }

    pub fn admin_role(&mut self, admin_role: bool) {
        self.admin_role = Some(admin_role);
    }

    pub fn settings_role(&mut self, settings_role: bool) {
        self.settings_role = Some(settings_role);
    }

    pub fn stream_role(&mut self, stream_role: bool) {
        self.stream_role = Some(stream_role);
    }

    pub fn jukebox_role(&mut self, jukebox_role: bool) {
        self.jukebox_role = Some(jukebox_role);
    }

    pub fn download_role(&mut self, download_role: bool) {
        self.download_role = Some(download_role);
    }

    pub fn upload_role(&mut self, upload_role: bool) {
        self.upload_role = Some(upload_role);
    }

    pub fn playlist_role(&mut self, playlist_role: bool) {
        self.playlist_role = Some(playlist_role);
    }

    pub fn cover_art_role(&mut self, cover_art_role: bool) {
        self.cover_art_role = Some(cover_art_role);
    }

    pub fn comment_role(&mut self, comment_role: bool) {
        self.comment_role = Some(comment_role);
    }

    pub fn podcast_role(&mut self, podcast_role: bool) {
        self.podcast_role = Some(podcast_role);
    }

    pub fn share_role(&mut self, share_role: bool) {
        self.share_role = Some(share_role);
    }

    pub fn video_conversion_role(&mut self, video_conversion_role: bool) {
        self.video_conversion_role = Some(video_conversion_role);
    }
}

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#createUser>
    pub async fn create_user(
        &self,
        username: impl Into<String>,
        password: impl Into<String>, // plain or hex encoded
        email: impl Into<String>,
        roles: Option<Roles>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("username", username);
        paras.push("password", password);
        paras.push("email", email);
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
