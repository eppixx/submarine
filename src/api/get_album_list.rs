use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use crate::data::{Album, ResponseType};
use crate::{Client, SubsonicError};

#[derive(Debug, PartialEq, Eq)]
pub enum Order {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    AlphabeticalByName,
    AlphabeticalByArtist,
    Starred,
}

impl FromStr for Order {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "random" => Ok(Self::Random),
            "newest" => Ok(Self::Newest),
            "highest" => Ok(Self::Highest),
            "frequent" => Ok(Self::Frequent),
            "recent" => Ok(Self::Recent),
            "alphabeticalByName" => Ok(Self::AlphabeticalByName),
            "alphabeticalByArtist" => Ok(Self::AlphabeticalByArtist),
            "starred" => Ok(Self::Starred),
            _ => Err(()),
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Random => write!(f, "random"),
            Self::Newest => write!(f, "newest"),
            Self::Highest => write!(f, "highest"),
            Self::Frequent => write!(f, "frequent"),
            Self::Recent => write!(f, "recent"),
            Self::AlphabeticalByName => write!(f, "alphabeticalByName"),
            Self::AlphabeticalByArtist => write!(f, "alphabeticalByArtist"),
            Self::Starred => write!(f, "starred"),
        }
    }
}

impl Client {
    pub(crate) fn create_paras(size: Option<usize>, offset: Option<usize>, music_folder_id: Option<impl Into<String>>) -> HashMap<&'static str, String> {
        let mut paras = std::collections::HashMap::new();
        if let Some(size) = size {
            paras.insert("size", size.to_string()); //500 is maximum
        }
        if let Some(offset) = offset {
            paras.insert("offset", offset.to_string());
        }
        if let Some(folder_id) = music_folder_id {
            paras.insert("musicFolderId", folder_id.into());
        }
        paras
    }

    pub async fn get_album_list(&self, order: Order, size: Option<usize>, offset: Option<usize>, music_folder_id: Option<impl Into<String>>) -> Result<Vec<Album>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", order.to_string());

        let body = self.request("getAlbumList", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.albums)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }

    pub async fn get_album_list_by_year(&self, from_year: Option<usize>, to_year: Option<usize>, size: Option<usize>, offset: Option<usize>, music_folder_id: Option<impl Into<String>>) -> Result<Vec<Album>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", String::from("byYear"));
        if let Some(from) = from_year {
            paras.insert("fromYear", from.to_string());
        }
        if let Some(to) = to_year {
            paras.insert("toYear", to.to_string());
        }

        let body = self.request("getAlbumList", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.albums)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }

    pub async fn get_album_list_by_genre(&self, genre: impl Into<String>, size: Option<usize>, offset: Option<usize>, music_folder_id: Option<impl Into<String>>) -> Result<Vec<Album>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", String::from("byGenre"));
        paras.insert("genre", genre.into());

        let body = self.request("getAlbumList", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.albums)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{
        data::{OuterResponse, ResponseType},
    };

    use super::Order;

    #[test]
    fn conversion_order() {
        let oracle = vec![
            Order::Random,
            Order::Newest,
            Order::Highest,
            Order::Frequent,
            Order::Recent,
            Order::AlphabeticalByName,
            Order::AlphabeticalByArtist,
            Order::Starred,
        ];
        for test in oracle {
            println!("testing: {test:?}");
            println!("  to_string: {}", test.to_string());
            println!("  to_order: {:?}", Order::from_str(&test.to_string()));
            assert_eq!(
                test,
                Order::from_str(&test.to_string()).unwrap(),
                "{test:?}"
            );
        }
    }

    #[test]
    fn conversion_get_album_list() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "albumList": {
                  "album": [
                    {
                      "id": "c686e803911d72cf98bd077c72326759",
                      "parent": "f97bc375b1576bdfd1116641304c4cf0",
                      "isDir": true,
                      "title": "Voodooism the Sequel",
                      "name": "Voodooism the Sequel",
                      "album": "Voodooism the Sequel",
                      "artist": "cYsmix",
                      "year": 2014,
                      "coverArt": "al-c686e803911d72cf98bd077c72326759_6476fade",
                      "starred": "2023-06-24T10:30:05Z",
                      "duration": 4846,
                      "playCount": 53,
                      "played": "2023-08-29T11:11:58Z",
                      "created": "2023-05-31T07:51:00.680123782Z",
                      "artistId": "f97bc375b1576bdfd1116641304c4cf0",
                      "songCount": 9,
                      "isVideo": false
                    },
                    {
                      "id": "9a53fb46776475612d41954e0e7a7a18",
                      "parent": "8dd319f210acbb691fd4f809e487e17f",
                      "isDir": true,
                      "title": "Planet Punk",
                      "name": "Planet Punk",
                      "album": "Planet Punk",
                      "artist": "Die Ärzte",
                      "year": 1995,
                      "genre": "Punk",
                      "coverArt": "al-9a53fb46776475612d41954e0e7a7a18_62687631",
                      "starred": "2022-05-10T09:40:50Z",
                      "duration": 3469,
                      "playCount": 29,
                      "played": "2023-03-21T10:03:18Z",
                      "created": "2021-08-12T14:34:03.204608575Z",
                      "artistId": "8dd319f210acbb691fd4f809e487e17f",
                      "songCount": 17,
                      "isVideo": false
                    }
                  ]
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::AlbumList { album_list } = response.data {
            assert_eq!(album_list.albums.len(), 2);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
