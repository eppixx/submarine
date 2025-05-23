use serde::{Deserialize, Serialize};

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

/// Status of Response from server
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Status {
    Ok,
    #[default]
    Error,
}

/// This is send by every request from server. If a request is 'Status::Ok' the returned
/// Info will be omitted. Requests that doesn't send something
/// back this will be returned instead.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(default, with = "status")]
    pub status: Status,

    /// protocol version
    pub version: String,

    /// identifier of server; returned by navidrome and maybe other servers
    pub r#type: Option<String>,

    /// version of server
    pub server_version: Option<String>,
}

/// every data that is dependent on the request
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseType {
    Error {
        error: Error,
    },
    Playlists {
        playlists: Playlists,
    },
    #[serde(rename_all = "camelCase")]
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
        artist: ArtistWithAlbumsId3,
    },
    Album {
        album: AlbumWithSongsId3,
    },
    #[serde(rename_all = "camelCase")]
    AlbumList {
        album_list: AlbumList,
    },
    #[serde(rename_all = "camelCase")]
    AlbumList2 {
        album_list2: AlbumList,
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
    Videos {
        videos: Videos,
    },
    #[serde(rename_all = "camelCase")]
    VideoInfo {
        video_info: VideoInfo,
    },
    #[serde(rename_all = "camelCase")]
    ArtistInfo {
        artist_info: ArtistInfo,
    },
    #[serde(rename_all = "camelCase")]
    ArtistInfo2 {
        artist_info2: ArtistInfo,
    },
    #[serde(rename_all = "camelCase")]
    AlbumInfo {
        album_info: AlbumInfo,
    },
    #[serde(rename_all = "camelCase")]
    SimilarSongs {
        similar_songs: SimilarSongs,
    },
    #[serde(rename_all = "camelCase")]
    SimilarSongs2 {
        similar_songs2: SimilarSongs,
    },
    #[serde(rename_all = "camelCase")]
    TopSongs {
        top_songs: TopSongs,
    },
    #[serde(rename_all = "camelCase")]
    RandomSongs {
        random_songs: Songs,
    },
    #[serde(rename_all = "camelCase")]
    SongsByGenre {
        songs_by_genre: Songs,
    },
    #[serde(rename_all = "camelCase")]
    NowPlaying {
        now_playing: NowPlaying,
    },
    Starred {
        starred: Starred,
    },
    #[serde(rename_all = "camelCase")]
    SearchResult2 {
        search_result2: SearchResult2,
    },
    #[serde(rename_all = "camelCase")]
    SearchResult3 {
        search_result3: SearchResult3,
    },
    Lyrics {
        lyrics: Lyrics,
    },
    Shares {
        shares: Shares,
    },
    Podcasts {
        podcasts: Podcasts,
    },
    #[serde(rename_all = "camelCase")]
    NewestPodcasts {
        newest_podcasts: NewestPodcasts,
    },
    #[serde(rename_all = "camelCase")]
    JukeboxStatus {
        jukebox_status: JukeboxStatus,
    },
    #[serde(rename_all = "camelCase")]
    JukeboxPlaylist {
        jukebox_playlist: JukeboxPlaylist,
    },
    #[serde(rename_all = "camelCase")]
    InternetRadionStations {
        internet_radio_stations: InternetRadioStations,
    },
    #[serde(rename_all = "camelCase")]
    ChatMessages {
        chat_messages: ChatMessages,
    },
    User {
        user: User,
    },
    Users {
        users: Users,
    },
    Bookmarks {
        bookmarks: Bookmarks,
    },
    #[serde(rename_all = "camelCase")]
    PlayQueue {
        play_queue: PlayQueue,
    },
    // order is important or it will allways be matched to ping
    Ping {},
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Serialize)]
pub struct ScanStatus {
    pub scanning: bool,
    pub count: Option<i64>,
    pub folder_count: Option<usize>, //navidrome specific
    pub last_scan: Option<chrono::DateTime<chrono::offset::FixedOffset>>, // navidrome specific
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ArtistsId3 {
    pub index: Vec<IndexId3>,
}

/// Indexes artists
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct IndexId3 {
    pub name: String,
    pub artist: Vec<ArtistId3>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistId3 {
    pub id: String,
    pub name: String,
    pub cover_art: Option<String>,
    pub artist_image_url: Option<String>,
    pub album_count: i32,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

impl PartialEq for ArtistId3 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ArtistId3 {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithAlbumsId3 {
    #[serde(flatten)]
    pub base: ArtistId3,
    #[serde(default)]
    pub album: Vec<AlbumId3>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum UserRating {
    One,
    Two,
    Three,
    Four,
    Five,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumList {
    pub album: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AlbumWithSongsId3 {
    #[serde(flatten)]
    pub base: AlbumId3,
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    pub id: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub is_dir: Option<bool>,
    #[serde(default)] // navidrome specific
    pub title: String,
    #[serde(default)]
    pub name: String,
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
    pub average_rating: Option<f64>,
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

impl PartialEq for Child {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Child {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum MediaType {
    Music,
    Podcast,
    Audiobook,
    Video,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Playlists {
    #[serde(default)]
    pub playlist: Vec<Playlist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub allowed_user: Option<Vec<String>>,
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub owner: Option<String>,
    pub public: Option<bool>,
    pub song_count: i32,
    pub duration: i32,
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    pub changed: chrono::DateTime<chrono::offset::FixedOffset>,
    pub cover_art: Option<String>,
}

impl PartialEq for Playlist {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Playlist {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    #[serde(flatten)]
    pub base: Playlist,
    #[serde(default)]
    pub entry: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub valid: bool,
    pub email: Option<String>,
    pub license_expires: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub trial_expires: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolders {
    pub music_folder: Vec<MusicFolder>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    pub id: i32,
    pub name: Option<String>,
}

impl PartialEq for MusicFolder {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for MusicFolder {}

cfg_if::cfg_if! {
    if #[cfg(feature = "navidrome")] {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Indexes {
            #[serde(default)]
            pub index: Vec<IndexId3>,
        }
    } else {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
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

        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct Index {
            pub name: String,
            pub artist: Vec<ArtistId3>,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    #[serde(default)]
    pub child: Vec<Child>,
    pub id: String,
    pub parent: Option<String>,
    pub name: String,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    // #[serde(default, with = "option_user_rating")]
    // pub user_rating: Option<UserRating>,
    pub average_rating: Option<f64>,
    pub play_count: Option<i64>,
}

impl PartialEq for Directory {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Directory {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Genres {
    pub genre: Vec<Genre>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    pub value: String, // TODO check other implementations if it exists there as well
    pub song_count: Option<i32>,
    pub album_count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Videos {
    pub video: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    pub id: String,
    #[serde(default)]
    pub captions: Vec<Captions>,
    #[serde(default)]
    pub audio_track: Vec<AudioTrack>,
    #[serde(default)]
    pub conversion: Vec<VideoConversion>,
}

impl PartialEq for VideoInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for VideoInfo {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Captions {
    pub id: String,
    pub name: Option<String>,
}

impl PartialEq for Captions {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Captions {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    pub id: String,
    pub name: Option<String>,
    pub language_code: Option<String>,
}

impl PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for AudioTrack {}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoConversion {
    pub id: String,
    pub bit_rate: Option<i32>, // in kbs
    pub audio_track_id: Option<i32>,
}

impl PartialEq for VideoConversion {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for VideoConversion {}

cfg_if::cfg_if! {
    if #[cfg(feature = "navidrome")] {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct ArtistInfoBase {
            pub biography: Option<String>,
            pub music_brainz_id: Option<String>,
            pub last_fm_rrl: Option<String>,
            pub small_image_url: Option<String>,
            pub medium_image_url: Option<String>,
            pub large_image_url: Option<String>,
        }
    } else {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct ArtistInfoBase {
            #[serde(default)]
            pub biography: Vec<String>,
            #[serde(default)]
            pub music_brainz_id: Vec<String>,
            #[serde(default)]
            pub last_fm_rrl: Vec<String>,
            #[serde(default)]
            pub small_image_url: Vec<String>,
            #[serde(default)]
            pub medium_image_url: Vec<String>,
            #[serde(default)]
            pub large_image_url: Vec<String>,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ArtistInfo {
    #[serde(flatten)]
    pub base: ArtistInfoBase,
    #[serde(default)]
    pub similar_artist: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub artist_image_url: Option<String>,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    #[serde(default, with = "option_user_rating")]
    pub user_rating: Option<UserRating>,
    pub average_rating: Option<f64>,
}

impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Artist {}

cfg_if::cfg_if! {
    if #[cfg(feature = "navidrome")] {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct AlbumInfo {
            pub notes: Option<String>,
            pub music_brainz_id: Option<String>,
            pub last_fm_rrl: Option<String>,
            pub small_image_url: Option<String>,
            pub medium_image_url: Option<String>,
            pub large_image_url: Option<String>,
        }
    } else {
        #[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
        #[serde(rename_all = "camelCase")]
        pub struct AlbumInfo {
            #[serde(default)]
            pub notes: Vec<String>,
            #[serde(default)]
            pub music_brainz_id: Vec<String>,
            #[serde(default)]
            pub last_fm_rrl: Vec<String>,
            #[serde(default)]
            pub small_image_url: Vec<String>,
            #[serde(default)]
            pub medium_image_url: Vec<String>,
            #[serde(default)]
            pub large_image_url: Vec<String>,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SimilarSongs {
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TopSongs {
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Songs {
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NowPlaying {
    #[serde(default)]
    pub entry: Vec<NowPlayingEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NowPlayingEntry {
    #[serde(flatten)]
    pub child: Child,
    pub username: String, // this is not a typo
    pub minutes_ago: i32,
    pub player_id: i32,
    pub player_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Starred {
    #[serde(default)]
    pub artist: Vec<Artist>,
    #[serde(default)]
    pub album: Vec<Child>,
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Starred2 {
    #[serde(default)]
    pub artist: Vec<ArtistId3>,
    #[serde(default)]
    pub album: Vec<AlbumId3>,
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumId3 {
    pub id: String,
    pub name: String,
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub cover_art: Option<String>,
    pub song_count: i32,
    pub duration: i32,
    pub play_count: Option<i64>,
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    pub starred: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    #[serde(default)]
    pub genres: Vec<std::collections::HashMap<String, String>>,
}

impl PartialEq for AlbumId3 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for AlbumId3 {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult2 {
    #[serde(default)]
    pub artist: Vec<Artist>,
    #[serde(default)]
    pub album: Vec<Child>,
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult3 {
    #[serde(default)]
    pub artist: Vec<ArtistId3>,
    #[serde(default)]
    pub album: Vec<AlbumId3>,
    #[serde(default)]
    pub song: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub value: Option<String>, // maybe navidrome only?
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Shares {
    #[serde(default)]
    pub share: Vec<Share>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    #[serde(default)]
    pub entry: Vec<Child>,
    pub id: String,
    pub url: String,
    pub description: Option<String>,
    pub username: String,
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    pub expires: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub last_visited: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
    pub visit_count: i32,
}

impl PartialEq for Share {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Share {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Podcasts {
    #[serde(default)]
    pub channel: Vec<PodcastChannel>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PodcastChannel {
    #[serde(default)]
    pub episode: Vec<PodcastEpisode>,
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub cover_art: Option<String>,
    pub original_image_url: Option<String>,
    pub status: PodcastStatus,
    pub error_message: Option<String>,
}

impl PartialEq for PodcastChannel {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for PodcastChannel {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewestPodcasts {
    #[serde(default)]
    pub episode: Vec<PodcastEpisode>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisode {
    #[serde(flatten)]
    pub child: Child,
    pub stream_id: Option<String>,
    pub channel_id: String,
    pub description: Option<String>,
    pub status: PodcastStatus,
    pub publish_date: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JukeboxStatus {
    pub current_index: i32,
    pub playing: bool,
    pub gain: f32,
    pub position: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JukeboxPlaylist {
    #[serde(flatten)]
    pub status: JukeboxStatus,
    #[serde(default)]
    pub entry: Vec<Child>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Deleted,
    Skipped,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InternetRadioStations {
    #[serde(default)]
    pub internet_radio_station: Vec<InternetRadioStation>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InternetRadioStation {
    pub id: String,
    pub name: String,
    pub stream_url: String,
    pub home_page_url: Option<String>,
}

impl PartialEq for InternetRadioStation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for InternetRadioStation {}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessages {
    #[serde(default)]
    pub chat_message: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub username: String,
    pub time: i64,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    #[serde(default)]
    pub user: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(default)]
    pub folder: Vec<i32>,
    pub username: String,
    pub email: Option<String>,
    pub scrobbling_enabled: bool,
    pub max_bit_rate: Option<i32>,
    pub admin_role: bool,
    pub settings_role: bool,
    pub download_role: bool,
    pub upload_role: bool,
    pub playlist_role: bool,
    pub cover_art_role: bool,
    pub comment_role: bool,
    pub podcast_role: bool,
    pub stream_role: bool,
    pub jukebox_role: bool,
    pub video_conversion_role: bool,
    pub avatar_last_changed: Option<chrono::DateTime<chrono::offset::FixedOffset>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bookmarks {
    #[serde(default)]
    pub bookmark: Vec<Bookmark>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub entry: Child,
    pub position: i64,
    pub username: String,
    pub comment: Option<String>,
    pub created: chrono::DateTime<chrono::offset::FixedOffset>,
    pub changed: chrono::DateTime<chrono::offset::FixedOffset>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueue {
    #[serde(default)]
    pub entry: Vec<Child>,
    pub current: Option<String>, // current playing track, TODO check if reference is wrong
    pub position: Option<i64>,   // in ms
    pub username: String,
    pub changed: chrono::DateTime<chrono::offset::FixedOffset>,
    pub changed_by: String, // client name
}

mod status {
    use super::Status;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Status, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<&str> = Option::deserialize(deserializer)?;
        match s {
            Some("ok") => Ok(Status::Ok),
            Some("failed") => Ok(Status::Error),
            _ => Ok(Status::Error),
        }
    }
}

mod option_user_rating {
    use crate::data::UserRating;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<UserRating>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<i32> = Option::deserialize(deserializer)?;
        match s {
            Some(1) => Ok(Some(UserRating::One)),
            Some(2) => Ok(Some(UserRating::Two)),
            Some(3) => Ok(Some(UserRating::Three)),
            Some(4) => Ok(Some(UserRating::Four)),
            Some(5) => Ok(Some(UserRating::Five)),
            _ => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<UserRating>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let number = match value {
            None => return serializer.serialize_none(),
            Some(UserRating::One) => 1,
            Some(UserRating::Two) => 2,
            Some(UserRating::Three) => 3,
            Some(UserRating::Four) => 4,
            Some(UserRating::Five) => 5,
        };
        serializer.serialize_i32(number)
    }
}
