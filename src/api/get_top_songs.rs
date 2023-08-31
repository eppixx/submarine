use crate::data::{Child, ResponseType};
use crate::{Client, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getTopSongs
    pub async fn get_top_songs(
        &self,
        artist: impl Into<String>,
        count: Option<i32>, //defaults to 50
    ) -> Result<Vec<Child>, SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        paras.insert("artist", artist.into());
        if let Some(count) = count {
            paras.insert("count", count.to_string());
        }

        let body = self.request("getTopSongs", Some(paras), None).await?;
        if let ResponseType::TopSongs { top_songs } = body.data {
            Ok(top_songs.song)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type TopSongs but found wrong type",
            )))
        }
    }
}

mod tests {
    //TODO better test
    #[test]
    fn conversion_get_top_songs() {
        let response_body = r##"
{
  "subsonic-response": {
    "status": "ok",
    "version": "1.16.1",
    "type": "navidrome",
    "serverVersion": "0.49.3 (8b93962f)",
    "topSongs": {}
  }
}"##;
        let response = serde_json::from_str::<crate::data::OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let crate::data::ResponseType::TopSongs { top_songs } = response.data {
            assert_eq!(top_songs.song.is_empty(), true);
        } else {
            panic!("wrong type {:?}", response.data);
        }
    }
}
