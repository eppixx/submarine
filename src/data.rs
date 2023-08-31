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
    pub info: Info,

    /// the type is dependent on the request
    #[serde(flatten)]
    pub data: ResponseType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
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
    Error {
        error: Error,
    },
    Playlists {
        playlists: Playlists,
    },
    PlaylistWithSongs {
        playlist: PlaylistWithSongs,
    },
    #[serde(rename_all = "camelCase")]
    ScanStatus {
        scan_status: ScanStatus,
    },
    Song {
        song: Box<Child>,
    },
    Artists {
        artists: ArtistsId3,
    },
    Artist {
        artist: ArtistId3,
    },
    Album {
        album: Album,
    },
    #[serde(rename_all = "camelCase")]
    AlbumList {
        album_list: AlbumList,
    },
    #[serde(rename_all = "camelCase")]
    AlbumList2 {
        album_list2: AlbumList2,
    },
    #[serde(rename_all = "camelCase")]
    MusicFolders {
        music_folders: MusicFolders,
    },
    License {
        license: License,
    },
    Indexes {
        indexes: Indexes,
    },
    #[serde(rename_all = "camelCase")]
    MusicDirectory {
        directory: Directory,
    },
    Genres {
        genres: Genres,
    },
    // order is important or it will allways be matched to ping
    Ping {},
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ScanStatus {
    pub scanning: bool,
    pub count: usize,
    pub folder_count: Option<usize>,
    pub last_scan: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ArtistsId3 {
    pub index: Vec<IndexId3>,
}

/// Indexes artists
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct IndexId3 {
    pub name: String,
    pub artist: Vec<ArtistId3>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistId3 {
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    #[serde(default, with = "option_user_rating")]
    pub user_rating: Option<UserRating>,
    // pub average_rating: Option<f64>,
    #[serde(default, rename = "album")]
    pub albums: Vec<Album>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRating {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList {
    #[serde(default, rename = "album")]
    pub albums: Vec<Album>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList2 {
    #[serde(default, rename = "album")]
    pub albums: Vec<Album>,
}

/// some responses don't send the song list<br>
///to combat this compare song_count with songs and use get_album if necessary
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist_id: Option<String>,
    pub cover_art: Option<String>,
    pub song_count: i32,
    pub duration: i32,
    pub play_count: Option<i64>,
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    #[serde(default, rename = "song")]
    pub songs: Vec<Child>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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
    // pub average_rating: Option<f32>,
    pub play_count: Option<i64>,
    pub disc_number: Option<i32>,
    pub created: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub album_id: Option<String>,
    pub artist_id: Option<String>,
    pub typ: Option<MediaType>,
    pub bookmark_position: Option<i64>,
    pub original_width: Option<i32>,
    pub original_height: Option<i32>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum MediaType {
    Music,
    Podcast,
    Audiobook,
    Video,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Playlists {
    #[serde(default, rename = "playlist")]
    pub playlists: Vec<Playlist>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    #[serde(flatten)]
    pub info: Playlist,
    #[serde(default)]
    pub songs: Vec<Child>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub trial_expires: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolders {
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: i32,
    pub name: Option<String>,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "navidrome")] {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Indexes {
            #[serde(default)]
            pub index: Vec<ArtistIndex>,
        }
    } else {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Indexes {
            #[serde(default)]
            pub shortcut: Vec<ArtistId3>,
            #[serde(default)]
            pub index: Vec<Index>,
            #[serde(default)]
            pub child: Vec<Child>,
            pub last_modified: chrono::DateTime<chrono::offset::FixedOffset>,
            pub ignore_articles: String,
        }

        #[derive(Debug, Deserialize, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Index {
            pub name: String,
            pub artist: Vec<ArtistId3>,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    #[serde(default)]
    pub child: Vec<Child>,
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    #[serde(default, with = "option_user_rating")]
    pub user_rating: Option<UserRating>,
    // pub average_rating: Option<AverageRating>,
    pub play_count: Option<i64>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Genres {
    pub genre: Vec<Genre>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub value: String, // TODO check other implementations if it exists there as well
    pub song_count: Option<i32>,
    pub album_count: Option<i32>,
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
