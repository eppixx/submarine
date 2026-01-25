use crate::data::{AlbumInfo, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: <http://www.subsonic.org/pages/api.jsp#getAlbumInfo2>
    pub async fn get_album_info2(&self, id: impl Into<String>) -> Result<AlbumInfo, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("getAlbumInfo2", Some(paras), None).await?;
        if let ResponseType::AlbumInfo { album_info } = body.data {
            Ok(album_info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type AlbumInfo but found wrong type",
            )))
        }
    }
}

#[cfg(all(test, not(feature = "navidrome")))]
mod tests {
    //TODO add test
}

#[cfg(all(test, feature = "navidrome"))]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_album_info2() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "albumInfo": {
      "notes": "Hi-Fi Serious is the third studio album by British alternative rock band A. The album's lead single Nothing was a big hit for the band. reaching #9 on the UK singles chart, while the follow-up single Starbucks reached #20. In the year following the album's release, the band were voted \"Best British Band\" at the Kerrang awards while the album was voted \"Worst Album\". Speculation has circulated that the band were able to achieve the two feats simultaneously by leading the album with a 'misleading' single that wasn't particularly representative of the band's sound throughout the rest of the album, <a href=\"https://www.last.fm/music/A/Hi-Fi+Serious\">Read more on Last.fm</a>.",
      "musicBrainzId": "7c5cc754-8ce4-4959-a164-541098b4a476",
      "lastFmUrl": "https://www.last.fm/music/A/Hi-Fi+Serious"
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::AlbumInfo { album_info } = response.data {
            assert_eq!(
                album_info.music_brainz_id.as_deref(),
                Some("7c5cc754-8ce4-4959-a164-541098b4a476")
            );
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
