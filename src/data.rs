use serde::Deserialize;

/// returned by the server
#[derive(Debug, Deserialize)]
pub(crate) struct OuterResponse {
    #[serde(rename = "subsonic-response")]
    pub(crate) inner: Response,
}

/// contains every possible response from a subsonic server
#[derive(Debug, Deserialize)]
pub(crate) struct Response {
    /// will be send with every response
    #[serde(flatten)]
    pub info: MetaInfo,

    /// the type is dependent on the request
    #[serde(flatten)]
    pub data: ResponseType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaInfo {
    /// either "ok" or "failed"
    pub status: String,

    /// protocol version
    pub version: String,

    /// identifier of server; returned by navidrome and maybe other servers
    pub r#type: Option<String>,

    /// version of server
    pub server_version: Option<String>,
}

/// every data that is dependent on the request
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ResponseType {
    Error { error: Error },
    // order is important or it will allways be matched to ping
    Ping {},
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    #[serde(default, with = "option_date_serde")]
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    #[serde(default, with = "option_user_rating")]
    pub user_rating: Option<UserRating>,
    pub average_rating: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum UserRating {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist_id: Option<String>,
    pub cover_art: Option<String>,
    pub song_count: i32,
    pub duration: i32,
    pub play_count: Option<i64>,
    #[serde(with = "date_serde")]
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    #[serde(default, with = "option_date_serde")]
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub year: Option<i32>,
    pub genre: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub id: String,
    pub parent: Option<String>,
    pub is_dir: bool,
    pub title: String,
    pub album: Option<String>,
    pub artist: Option<String>,
    pub track: Option<i32>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub cover_art: Option<String>,
    pub size: Option<i64>,
    pub content_type: Option<String>,
    pub suffix: Option<String>,
    pub transcoded_content_type: Option<String>,
    pub transcoded_suffix: Option<String>,
    pub duration: Option<i32>,
    pub bit_rate: Option<i32>,
    pub path: Option<String>,
    pub is_video: Option<bool>,
    pub user_rating: Option<i32>,
    pub average_rating: Option<f32>,
    pub play_count: Option<i64>,
    pub disc_number: Option<i32>,
    #[serde(default, with = "option_date_serde")]
    pub created: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    #[serde(default, with = "option_date_serde")]
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub album_id: Option<String>,
    pub artist_id: Option<String>,
    pub typ: Option<MediaType>,
    pub bookmark_position: Option<i64>,
    pub original_width: Option<i32>,
    pub original_height: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum MediaType {
    Music,
    Podcast,
    Audiobook,
    Video,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub allowed_user: Option<Vec<String>>,
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub owner: Option<String>,
    pub public: Option<bool>,
    pub song_count: i32,
    // pub duration: i32,
    // pub created: String,
    pub changed: String,
    pub cover_art: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    #[serde(flatten)]
    pub info: Playlist,
    #[serde(default)]
    pub entry: Vec<Child>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

mod date_serde {
    use chrono::{offset::FixedOffset, DateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)
    }
}

mod option_date_serde {
    use chrono::{offset::FixedOffset, DateTime};
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => Ok(Some(
                DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}

mod option_user_rating {
    use crate::data::UserRating;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<UserRating>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<i32> = Option::deserialize(deserializer)?;
        match s {
            Some(s) if s == 1 => Ok(Some(UserRating::One)),
            Some(s) if s == 2 => Ok(Some(UserRating::Two)),
            Some(s) if s == 3 => Ok(Some(UserRating::Three)),
            Some(s) if s == 4 => Ok(Some(UserRating::Four)),
            Some(s) if s == 5 => Ok(Some(UserRating::Five)),
            _ => Ok(None),
        }
    }
}
