use crate::api::get_album_list::Order;
use crate::{
    data::{Child, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getAlbumList2
    pub async fn get_album_list2(
        &self,
        order: Order,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.push("type", order.to_string());
        paras.push("openSubsonic", String::from("false"));

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList2 { album_list2 } = body.data {
            Ok(album_list2.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type AlbumList2 but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getAlbumList2
    pub async fn get_album_list2_by_year(
        &self,
        from_year: Option<usize>,
        to_year: Option<usize>,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.push("type", "byYear");
        if let Some(from) = from_year {
            paras.push("fromYear", from.to_string());
        }
        if let Some(to) = to_year {
            paras.push("toYear", to.to_string());
        }

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList2 { album_list2 } = body.data {
            Ok(album_list2.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type AlbumList2 but found wrong type",
            )))
        }
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#getAlbumList2
    pub async fn get_album_list2_by_genre(
        &self,
        genre: impl Into<String>,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.push("type", "byGenre");
        paras.push("genre", genre);

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList2 { album_list2 } = body.data {
            Ok(album_list2.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type AlbumList2 but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{Child, OuterResponse, ResponseType};

    #[test]
    fn conversion_get_album_list2_navidrome_v0_49_3() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "albumList2": {
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
        if let ResponseType::AlbumList2 { album_list2 } = response.data {
            assert_eq!(album_list2.album.len(), 2);
            assert_eq!(album_list2.album[0].year, Some(2014));
            assert_eq!(album_list2.album[0].starred.is_some(), true);
        } else {
            panic!("wrong type: {response:?}");
        }
    }

    #[test]
    fn conversion_album_navidrome_v0_49_3() {
        let json = r##"
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
          }
        "##;
        let album = serde_json::from_str::<Child>(json).unwrap();
        assert_eq!(album.id, "c686e803911d72cf98bd077c72326759");
        assert_eq!(album.title, "Voodooism the Sequel");
    }

    #[test]
    fn conversion_album_navidrome_v0_55_2() {
        let json = r##"
            {
                "id": "3CHbxxPvEVgBdQ4ZDbNtHa",
                "name": "Digital Nightmare",
                "artist": "TWRP",
                "artistId": "2LQ6xOBp5XYBchZXrdoxqM",
                "coverArt": "al-3CHbxxPvEVgBdQ4ZDbNtHa_67a21f32",
                "songCount": 12,
                "duration": 2553,
                "playCount": 1,
                "created": "2025-02-04T14:26:45.988791965Z",
                "starred": "2025-03-07T16:10:42.076304046Z",
                "year": 2024,
                "genre": "Rock",
                "played": "2025-04-10T09:02:33.749199289Z",
                "userRating": 0,
                "genres": [
                  {
                    "name": "Rock"
                  }
                ],
                "musicBrainzId": "c2803234-c812-47dc-af01-ba7c691810a8",
                "isCompilation": false,
                "sortName": "digital nightmare",
                "discTitles": [],
                "originalReleaseDate": {
                  "year": 2024,
                  "month": 3,
                  "day": 22
                },
                "releaseDate": {
                  "year": 2024,
                  "month": 3,
                  "day": 22
                },
                "releaseTypes": [
                  "album"
                ],
                "recordLabels": [
                  {
                    "name": "TWRP"
                  }
                ],
                "moods": [],
                "artists": [
                  {
                    "id": "2LQ6xOBp5XYBchZXrdoxqM",
                    "name": "TWRP"
                  }
                ],
                "displayArtist": "TWRP",
                "explicitStatus": "",
                "version": ""
              }
        "##;
        let album = serde_json::from_str::<Child>(json).unwrap();
        assert_eq!(album.id, "3CHbxxPvEVgBdQ4ZDbNtHa");
        assert_eq!(album.name, "Digital Nightmare");
    }

    #[test]
    fn conversion_get_album_list2_navidrome_v0_55_2() {
        let response_body = r##"
        {
          "subsonic-response": {
          "status": "ok",
          "version": "1.16.1",
          "type": "navidrome",
          "serverVersion": "0.55.2 (a057a680)",
          "openSubsonic": true,
          "albumList2": {
            "album": [
              {
                "id": "3CHbxxPvEVgBdQ4ZDbNtHa",
                "name": "Digital Nightmare",
                "artist": "TWRP",
                "artistId": "2LQ6xOBp5XYBchZXrdoxqM",
                "coverArt": "al-3CHbxxPvEVgBdQ4ZDbNtHa_67a21f32",
                "songCount": 12,
                "duration": 2553,
                "playCount": 1,
                "created": "2025-02-04T14:26:45.988791965Z",
                "starred": "2025-03-07T16:10:42.076304046Z",
                "year": 2024,
                "genre": "Rock",
                "played": "2025-04-10T09:02:33.749199289Z",
                "userRating": 0,
                "genres": [
                  {
                    "name": "Rock"
                  }
                ],
                "musicBrainzId": "c2803234-c812-47dc-af01-ba7c691810a8",
                "isCompilation": false,
                "sortName": "digital nightmare",
                "discTitles": [],
                "originalReleaseDate": {
                  "year": 2024,
                  "month": 3,
                  "day": 22
                },
                "releaseDate": {
                  "year": 2024,
                  "month": 3,
                  "day": 22
                },
                "releaseTypes": [
                  "album"
                ],
                "recordLabels": [
                  {
                    "name": "TWRP"
                  }
                ],
                "moods": [],
                "artists": [
                  {
                    "id": "2LQ6xOBp5XYBchZXrdoxqM",
                    "name": "TWRP"
                  }
                ],
                "displayArtist": "TWRP",
                "explicitStatus": "",
                "version": ""
              },
              {
                "id": "0oXK6zalrnagcJz4J0LjiP",
                "name": "New \\u0026 Improved",
                "artist": "TWRP",
                "artistId": "2LQ6xOBp5XYBchZXrdoxqM",
                "coverArt": "al-0oXK6zalrnagcJz4J0LjiP_67a21f77",
                "songCount": 10,
                "duration": 2491,
                "playCount": 1,
                "created": "2025-02-04T14:26:45.95609747Z",
                "starred": "2025-02-15T22:46:32.636798151Z",
                "year": 2021,
                "genre": "Rock",
                "played": "2025-04-09T17:21:16.868976101Z",
                "userRating": 0,
                "genres": [
                  {
                    "name": "Rock"
                  }
                ],
                "musicBrainzId": "3a01a90a-4830-4458-9b25-453f07e571b6",
                "isCompilation": false,
                "sortName": "new \\u0026 improved",
                "discTitles": [],
                "originalReleaseDate": {
                  "year": 2021,
                  "month": 11,
                  "day": 25
                },
                "releaseDate": {
                  "year": 2021,
                  "month": 11,
                  "day": 25
                },
                "releaseTypes": [
                  "album"
                ],
                "recordLabels": [
                  {
                    "name": "TWRP"
                  }
                ],
                "moods": [],
                "artists": [
                  {
                    "id": "2LQ6xOBp5XYBchZXrdoxqM",
                    "name": "TWRP"
                  }
                ],
                "displayArtist": "TWRP",
                "explicitStatus": "",
                "version": ""
              }
            ]
          }
        }
      }
            "##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::AlbumList2 { album_list2 } = response.data {
            assert_eq!(album_list2.album.len(), 2);
            assert_eq!(album_list2.album[0].year, Some(2024));
            assert_eq!(album_list2.album[0].starred.is_some(), true);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
