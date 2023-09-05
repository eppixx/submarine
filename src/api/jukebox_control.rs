use std::fmt::Display;

use crate::{
    data::{JukeboxPlaylist, JukeboxStatus, ResponseType},
    Client, SubsonicError,
};

pub enum Action {
    Status,
    Start,
    Stop,
    Clear,
    Shuffle,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status => write!(f, "status"),
            Self::Start => write!(f, "start"),
            Self::Stop => write!(f, "stop"),
            Self::Clear => write!(f, "lcear"),
            Self::Shuffle => write!(f, "shuffle"),
        }
    }
}

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control(&self, action: Action) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", action.to_string());

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_get(&self) -> Result<JukeboxPlaylist, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("get"));

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxPlaylist { jukebox_playlist } = body.data {
            Ok(jukebox_playlist)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxPlaylist but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_set(
        &self,
        id: Vec<impl Into<String>>,
    ) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("set"));
        for id in id {
            paras.insert("id", id.into());
        }

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_skip(
        &self,
        index: i32,
        offset: Option<i32>,
    ) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("skip"));
        paras.insert("index", index.to_string());
        if let Some(offset) = offset {
            paras.insert("offset", offset.to_string());
        }

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_add(
        &self,
        id: Vec<impl Into<String>>,
    ) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("add"));
        for id in id {
            paras.insert("id", id.into());
        }

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_remove(&self, index: i32) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("remove"));
        paras.insert("index", index.to_string());

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#jukeboxControl
    pub async fn jukebox_control_set_gain(
        &self,
        gain: f32,
    ) -> Result<JukeboxStatus, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("action", String::from("setGain"));
        paras.insert("gain", gain.to_string());

        let body = self.request("jukeboxControl", Some(paras), None).await?;
        if let ResponseType::JukeboxStatus { jukebox_status } = body.data {
            Ok(jukebox_status)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type JukeboxStatus but found wrong type",
            )))
        }
    }
}

// TODO add test
