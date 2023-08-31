use crate::data::{ArtistInfo, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getArtistInfo
    pub async fn get_artist_info(
        &self,
        id: impl Into<String>,
        count: Option<i32>, //defaults to 20
        include_not_present: Option<bool>,
    ) -> Result<ArtistInfo, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("id", id.into());
        if let Some(count) = count {
            paras.insert("count", count.to_string());
        }
        if let Some(include) = include_not_present {
            paras.insert("includeNotPresent", include.to_string());
        }

        let body = self.request("getArtistInfo", Some(paras), None).await?;
        if let ResponseType::ArtistInfo { artist_info } = body.data {
            Ok(artist_info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type ArtistInfo but found wrong type",
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
    fn conversion_get_artist_info() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "artistInfo": {
      "biography": "Formed in 2005 in Los Angeles, CA, +44 was a band from two blink-182 members Mark Hoppus (Bass Guitar/Vocals) and Travis Barker (Drums/Keyboards), who has also played in a variety of musical projects including The Aquabats, Box Car Racer, and Transplants. +44 also featured Shane Gallagher (Lead Guitar), from The Nervous Return, and Craig Fairbaugh (Rhythm Guitar/Backing Vocals), from Lars Frederiksen and the Bastards, The Forgotten, and Mercy Killers.  <a target='_blank' href=\"https://www.last.fm/music/%252B44\" rel=\"nofollow\">Read more on Last.fm</a>",
      "musicBrainzId": "c2a44e93-3a2b-44aa-bd8b-7a71bb76e3b5",
      "lastFmUrl": "https://www.last.fm/music/%252B44",
      "similarArtist": [
        {
          "id": "1484948f7b3941a3220d43b1d92d4d3d",
          "name": "Box Car Racer",
          "albumCount": 1,
          "coverArt": "ar-1484948f7b3941a3220d43b1d92d4d3d_0"
        },
        {
          "id": "e1e3b206dc89b1aab8f9b2221a751e48",
          "name": "Good Charlotte",
          "albumCount": 3,
          "coverArt": "ar-e1e3b206dc89b1aab8f9b2221a751e48_0"
        }
      ]
    }
  }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::ArtistInfo { artist_info } = response.data {
            assert_eq!(artist_info.similar_artist.len(), 2);
            assert_eq!(
                &artist_info.similar_artist.first().unwrap().name,
                "Box Car Racer"
            );
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
