use crate::data::{AlbumWithSongsId3, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getAlbum
    pub async fn get_album(
        &self,
        id: impl Into<String>,
    ) -> Result<AlbumWithSongsId3, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("getAlbum", Some(paras), None).await?;
        if let ResponseType::Album { album } = body.data {
            Ok(album)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Album but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_album() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "album": {
                  "id": "c58e1cddef76983860e21603bc85c363",
                  "name": "Hi-Fi Serious",
                  "artist": "A",
                  "artistId": "0cc175b9c0f1b6a831c399e269772661",
                  "coverArt": "al-c58e1cddef76983860e21603bc85c363_6113f6cd",
                  "songCount": 12,
                  "duration": 2647,
                  "created": "2021-08-11T16:33:47.650644719Z",
                  "year": 2002,
                  "song": [
                    {
                      "id": "1bd66355b36ebb2cd2b498a6ba95bd83",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Nothing",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 1,
                      "year": 2002,
                      "coverArt": "mf-1bd66355b36ebb2cd2b498a6ba95bd83_6113f6cd",
                      "size": 5483084,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 223,
                      "bitRate": 192,
                      "path": "A/Hi-Fi Serious/01 - Nothing.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650644719Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "e1b1b6945485137e453550321f6bf91a",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Something's Going On",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 2,
                      "year": 2002,
                      "coverArt": "mf-e1b1b6945485137e453550321f6bf91a_6113f6cd",
                      "size": 2962827,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 178,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/02 - Something's Going On.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.65028698Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "5476bbde93090258e2d91e8333163133",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "6 O'Clock on a Tube Stop",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 3,
                      "year": 2002,
                      "coverArt": "mf-5476bbde93090258e2d91e8333163133_6113f6cd",
                      "size": 4773069,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 194,
                      "bitRate": 192,
                      "path": "A/Hi-Fi Serious/03 - 6 O'Clock on a Tube Stop.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650580818Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "46e8fd063711e3c6f88f4e063d1ac4b2",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Going Down",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 4,
                      "year": 2002,
                      "coverArt": "mf-46e8fd063711e3c6f88f4e063d1ac4b2_6113f6cd",
                      "size": 4108889,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 249,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/04 - Going Down.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650440512Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "aea80dc76da38db5fb94cf67ce6812eb",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Took It Away",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 5,
                      "year": 2002,
                      "coverArt": "mf-aea80dc76da38db5fb94cf67ce6812eb_6113f6cd",
                      "size": 3514157,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 212,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/05 - Took It Away.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650140526Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "fa07456b89d9ec65fac9b0eff9a2a4ca",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Starbucks",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 6,
                      "year": 2002,
                      "coverArt": "mf-fa07456b89d9ec65fac9b0eff9a2a4ca_6113f6cd",
                      "size": 4876014,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 198,
                      "bitRate": 192,
                      "path": "A/Hi-Fi Serious/06 - Starbucks.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650408043Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "43edab171c9544674baf582dd953377e",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "The Springs",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 7,
                      "year": 2002,
                      "coverArt": "mf-43edab171c9544674baf582dd953377e_6113f6cd",
                      "size": 2809163,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 168,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/07 - The Springs.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650498544Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "ade555981621e542f615b843239536f4",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Shut Yer Face",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 8,
                      "year": 2002,
                      "coverArt": "mf-ade555981621e542f615b843239536f4_6113f6cd",
                      "size": 3721870,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 225,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/08 - Shut Yer Face.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.65019575Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "80462b21e558a989a7c42b696f44ee33",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Pacific Ocean Blue",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 9,
                      "year": 2002,
                      "coverArt": "mf-80462b21e558a989a7c42b696f44ee33_6113f6cd",
                      "size": 3517558,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 212,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/09 - Pacific Ocean Blue.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650240487Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "39ff6d2cde818eae5fe09c0bd08d4fdc",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "The Distance",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 10,
                      "year": 2002,
                      "coverArt": "mf-39ff6d2cde818eae5fe09c0bd08d4fdc_6113f6cd",
                      "size": 5336171,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 217,
                      "bitRate": 192,
                      "path": "A/Hi-Fi Serious/10 - The Distance.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650619528Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "84ac8ed66d026ca2be76e755fa829485",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "W.D.Y.C.A.I.",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 11,
                      "year": 2002,
                      "coverArt": "mf-84ac8ed66d026ca2be76e755fa829485_6113f6cd",
                      "size": 3424232,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 207,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/11 - W.D.Y.C.A.I..mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650529855Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    },
                    {
                      "id": "124b3574a6d76fd9d6048f9874668f35",
                      "parent": "c58e1cddef76983860e21603bc85c363",
                      "isDir": false,
                      "title": "Hi-Fi Serious",
                      "album": "Hi-Fi Serious",
                      "artist": "A",
                      "track": 12,
                      "year": 2002,
                      "coverArt": "mf-124b3574a6d76fd9d6048f9874668f35_6113f6cd",
                      "size": 5861575,
                      "contentType": "audio/mpeg",
                      "suffix": "mp3",
                      "duration": 359,
                      "bitRate": 128,
                      "path": "A/Hi-Fi Serious/12 - Hi-Fi Serious.mp3",
                      "discNumber": 1,
                      "created": "2021-08-11T16:33:47.650350291Z",
                      "albumId": "c58e1cddef76983860e21603bc85c363",
                      "artistId": "0cc175b9c0f1b6a831c399e269772661",
                      "type": "music",
                      "isVideo": false
                    }
                  ]
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Album { album } = response.data {
            assert_eq!(album.song.len(), 12);
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
