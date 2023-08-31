use crate::api::get_album_list::Order;
use crate::{
    data::{Child, ResponseType},
    Client, SubsonicError,
};

impl Client {
    pub async fn get_album_list2(
        &self,
        order: Order,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", order.to_string());

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }

    pub async fn get_album_list2_by_year(
        &self,
        from_year: Option<usize>,
        to_year: Option<usize>,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", String::from("byYear"));
        if let Some(from) = from_year {
            paras.insert("fromYear", from.to_string());
        }
        if let Some(to) = to_year {
            paras.insert("toYear", to.to_string());
        }

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }

    pub async fn get_album_list2_by_genre(
        &self,
        genre: impl Into<String>,
        size: Option<usize>,
        offset: Option<usize>,
        music_folder_id: Option<impl Into<String>>,
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = Self::create_paras(size, offset, music_folder_id);
        paras.insert("type", String::from("byGenre"));
        paras.insert("genre", genre.into());

        let body = self.request("getAlbumList2", Some(paras), None).await?;
        if let ResponseType::AlbumList { album_list } = body.data {
            Ok(album_list.album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "got send wrong type; submarine fault?",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_album_list() {
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
            assert_eq!(album_list2.album.first().unwrap().year, Some(2014));
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
