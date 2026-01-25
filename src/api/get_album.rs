use crate::data::{AlbumWithSongsId3, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#getAlbum>
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

    #[test]
    fn conversion_navidrome_0_55_2() {
        let response_body = r##"
{"subsonic-response":{"status":"ok","version":"1.16.1","type":"navidrome","serverVersion":"0.55.2 (a057a680)","openSubsonic":true,"album":{"id":"01bGeONZpedtpVt0kElyJq","name":"Jungle","artist":"Jungle","artistId":"3QnCg1lCxRTAkh42IISnQg","coverArt":"al-01bGeONZpedtpVt0kElyJq_60fbf37a","songCount":12,"duration":2351,"created":"2025-04-18T22:04:08.829566426Z","year":2014,"genre":"Alternative","userRating":0,"genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"musicBrainzId":"45850ffc-62e6-43a7-800f-c54d69f8dc1f","isCompilation":false,"sortName":"jungle","discTitles":[],"originalReleaseDate":{"year":2014,"month":7,"day":14},"releaseDate":{"year":2014,"month":7,"day":14},"releaseTypes":["album"],"recordLabels":[{"name":"XL Recordings"}],"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","explicitStatus":"","version":"","song":[{"id":"Fwt1897zxsszWTtd2AghWU","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"The Heat","album":"Jungle","artist":"Jungle","track":1,"year":2014,"genre":"Alternative","coverArt":"mf-Fwt1897zxsszWTtd2AghWU_60fbf37a","size":7974474,"contentType":"audio/mpeg","suffix":"mp3","duration":196,"bitRate":320,"path":"Jungle/Jungle/01-01 - The Heat.mp3","discNumber":1,"created":"2025-04-18T22:04:08.830701505Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":110,"comment":"","sortName":"the heat","mediaType":"song","musicBrainzId":"c9302e47-3c86-464e-a1ae-03e90c43d1e0","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-5.06,"albumGain":-4.37,"trackPeak":1.067553,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"sUR0bCwj9E97OHg4UBpten","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Accelerate","album":"Jungle","artist":"Jungle","track":2,"year":2014,"genre":"Alternative","coverArt":"mf-sUR0bCwj9E97OHg4UBpten_60fbf37a","size":7513164,"contentType":"audio/mpeg","suffix":"mp3","duration":184,"bitRate":320,"path":"Jungle/Jungle/01-02 - Accelerate.mp3","discNumber":1,"created":"2025-04-18T22:04:08.831356074Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":101,"comment":"","sortName":"accelerate","mediaType":"song","musicBrainzId":"3f4e440c-223c-4df5-8fa2-526a39319935","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-4.22,"albumGain":-4.37,"trackPeak":1.039621,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"IhL01sSLUDtC36DrtHTNaN","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Busy Earnin’","album":"Jungle","artist":"Jungle","track":3,"year":2014,"genre":"Alternative","coverArt":"mf-IhL01sSLUDtC36DrtHTNaN_60fbf37a","size":7402336,"contentType":"audio/mpeg","suffix":"mp3","duration":181,"bitRate":320,"path":"Jungle/Jungle/01-03 - Busy Earnin’.mp3","discNumber":1,"created":"2025-04-18T22:04:08.830170149Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":100,"comment":"","sortName":"busy earnin'","mediaType":"song","musicBrainzId":"9555c3ba-7376-44a0-8d01-858501286966","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-6.09,"albumGain":-4.37,"trackPeak":1.066777,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"K7dh1pz5BbHNdigeLAgXZ1","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Platoon","album":"Jungle","artist":"Jungle","track":4,"year":2014,"genre":"Alternative","coverArt":"mf-K7dh1pz5BbHNdigeLAgXZ1_60fbf37a","size":7827676,"contentType":"audio/mpeg","suffix":"mp3","duration":192,"bitRate":320,"path":"Jungle/Jungle/01-04 - Platoon.mp3","discNumber":1,"created":"2025-04-18T22:04:08.831023374Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":199,"comment":"","sortName":"platoon","mediaType":"song","musicBrainzId":"4315e8df-9969-4fad-bf9b-2197d9296e79","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-4.91,"albumGain":-4.37,"trackPeak":1.056204,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"ND8sHfWLkziDhJp3pH5vJ2","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Drops","album":"Jungle","artist":"Jungle","track":5,"year":2014,"genre":"Alternative","coverArt":"mf-ND8sHfWLkziDhJp3pH5vJ2_60fbf37a","size":7075687,"contentType":"audio/mpeg","suffix":"mp3","duration":173,"bitRate":320,"path":"Jungle/Jungle/01-05 - Drops.mp3","discNumber":1,"created":"2025-04-18T22:04:08.829695079Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":107,"comment":"","sortName":"drops","mediaType":"song","musicBrainzId":"33958c73-7a17-437c-bde3-bcdd4acf921e","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-2.11,"albumGain":-4.37,"trackPeak":1.022431,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"heksD5jDmQzd4B4BmLp2X6","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Time","album":"Jungle","artist":"Jungle","track":6,"year":2014,"genre":"Alternative","coverArt":"mf-heksD5jDmQzd4B4BmLp2X6_60fbf37a","size":8677543,"contentType":"audio/mpeg","suffix":"mp3","duration":213,"bitRate":320,"path":"Jungle/Jungle/01-06 - Time.mp3","discNumber":1,"created":"2025-04-18T22:04:08.829877104Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":119,"comment":"","sortName":"time","mediaType":"song","musicBrainzId":"7421538a-460a-42f6-92d0-56740c738df7","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-4.04,"albumGain":-4.37,"trackPeak":0.982478,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"hv3WbHzSpb0bUt7crvy6Jv","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Smoking Pixels","album":"Jungle","artist":"Jungle","track":7,"year":2014,"genre":"Alternative","coverArt":"mf-hv3WbHzSpb0bUt7crvy6Jv_60fbf37a","size":4408093,"contentType":"audio/mpeg","suffix":"mp3","duration":107,"bitRate":320,"path":"Jungle/Jungle/01-07 - Smoking Pixels.mp3","discNumber":1,"created":"2025-04-18T22:04:08.829944181Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":92,"comment":"","sortName":"smoking pixels","mediaType":"song","musicBrainzId":"dc3be8ce-50fe-4020-8b7e-2cfd5bd04b28","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-1.3,"albumGain":-4.37,"trackPeak":0.988719,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"NCJoXfkNPK9H2mQcb4LfPq","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Julia","album":"Jungle","artist":"Jungle","track":8,"year":2014,"genre":"Alternative","coverArt":"mf-NCJoXfkNPK9H2mQcb4LfPq_60fbf37a","size":7944370,"contentType":"audio/mpeg","suffix":"mp3","duration":195,"bitRate":320,"path":"Jungle/Jungle/01-08 - Julia.mp3","discNumber":1,"created":"2025-04-18T22:04:08.830068476Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":108,"comment":"","sortName":"julia","mediaType":"song","musicBrainzId":"7430685d-bb65-4225-8f16-fb29e14ce0a4","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-4.64,"albumGain":-4.37,"trackPeak":1.027802,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"rUstPcEXNinssz7WwvIDTK","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Crumbler","album":"Jungle","artist":"Jungle","track":9,"year":2014,"genre":"Alternative","coverArt":"mf-rUstPcEXNinssz7WwvIDTK_60fbf37a","size":7412405,"contentType":"audio/mpeg","suffix":"mp3","duration":182,"bitRate":320,"path":"Jungle/Jungle/01-09 - Crumbler.mp3","discNumber":1,"created":"2025-04-18T22:04:08.82977983Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":199,"comment":"","sortName":"crumbler","mediaType":"song","musicBrainzId":"d592d38b-792a-4b63-906a-d1e0d6d2bd70","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-3.49,"albumGain":-4.37,"trackPeak":1.028186,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"NtQd5cpx4fjvbOb6i2x0ie","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Son of a Gun","album":"Jungle","artist":"Jungle","track":10,"year":2014,"genre":"Alternative","coverArt":"mf-NtQd5cpx4fjvbOb6i2x0ie_60fbf37a","size":8477172,"contentType":"audio/mpeg","suffix":"mp3","duration":208,"bitRate":320,"path":"Jungle/Jungle/01-10 - Son of a Gun.mp3","discNumber":1,"created":"2025-04-18T22:04:08.831176985Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":98,"comment":"","sortName":"son of a gun","mediaType":"song","musicBrainzId":"09f30ea6-9b9d-4151-b332-b03f58e0a280","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-3.01,"albumGain":-4.37,"trackPeak":1.025435,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"2L7TbSf7LVPtiZv62o4xCj","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Lucky I Got What I Want","album":"Jungle","artist":"Jungle","track":11,"year":2014,"genre":"Alternative","coverArt":"mf-2L7TbSf7LVPtiZv62o4xCj_60fbf37a","size":10406566,"contentType":"audio/mpeg","suffix":"mp3","duration":256,"bitRate":320,"path":"Jungle/Jungle/01-11 - Lucky I Got What I Want.mp3","discNumber":1,"created":"2025-04-18T22:04:08.830856448Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":186,"comment":"","sortName":"lucky i got what i want","mediaType":"song","musicBrainzId":"89f549ff-1ce7-4f3e-ab77-1f5f649c8ead","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-0.6,"albumGain":-4.37,"trackPeak":1.000349,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""},{"id":"alGc4RNn1PkluUVYuH70bQ","parent":"01bGeONZpedtpVt0kElyJq","isDir":false,"title":"Lemonade Lake","album":"Jungle","artist":"Jungle","track":12,"year":2014,"genre":"Alternative","coverArt":"mf-alGc4RNn1PkluUVYuH70bQ_60fbf37a","size":10517658,"contentType":"audio/mpeg","suffix":"mp3","duration":259,"bitRate":320,"path":"Jungle/Jungle/01-12 - Lemonade Lake.mp3","discNumber":1,"created":"2025-04-18T22:04:08.829566426Z","albumId":"01bGeONZpedtpVt0kElyJq","artistId":"3QnCg1lCxRTAkh42IISnQg","type":"music","isVideo":false,"bpm":94,"comment":"","sortName":"lemonade lake","mediaType":"song","musicBrainzId":"5fd0a6c8-d750-4259-8ac9-a2ab91a18739","genres":[{"name":"Alternative"},{"name":"Electro"},{"name":"Pop"},{"name":"R\u0026B"}],"replayGain":{"trackGain":-2.91,"albumGain":-4.37,"trackPeak":1.04579,"albumPeak":1.067553},"channelCount":2,"samplingRate":44100,"bitDepth":0,"moods":[],"artists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayArtist":"Jungle","albumArtists":[{"id":"3QnCg1lCxRTAkh42IISnQg","name":"Jungle"}],"displayAlbumArtist":"Jungle","contributors":[],"displayComposer":"","explicitStatus":""}]}}}
"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Album { album } = response.data {
            assert_eq!(album.song.len(), 12);
            assert_eq!(album.base.name, String::from("Jungle"));
            assert_eq!(album.song[0].title, String::from("The Heat"));
            assert_eq!(album.base.genres[0].get("name"), Some(&String::from("Alternative")));
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
