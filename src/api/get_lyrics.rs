use crate::data::{Lyrics, ResponseType};
use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getLyrics
    pub async fn get_lyrics(&self, id: impl Into<String>) -> Result<Lyrics, SubsonicError> {
        let mut paras = Parameter::new();
        paras.push("id", id);

        let body = self.request("getLyrics", Some(paras), None).await?;
        if let ResponseType::Lyrics { lyrics } = body.data {
            Ok(lyrics)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Lyrics but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_get_lyrics_empty() {
        let response_body = r##"
            {
              "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3 (8b93962f)",
                "lyrics": {
                  "value": ""
                }
              }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Lyrics { lyrics } = response.data {
            assert_eq!(lyrics.value.as_deref(), Some(""));
        } else {
            panic!("wrong type: {response:?}");
        }
    }

    #[test]
    fn conversion_get_lyrics() {
        let response_body = r##"
            {
                "subsonic-response": {
                    "status": "ok",
                    "version": "1.16.1",
                    "type": "navidrome",
                    "serverVersion": "0.49.3 (8b93962f)",
                    "lyrics": {
                        "artist": "Ella Fitzgerald",
                        "title": "Santa Claus Is Coming to Town",
                        "value": "You better watch out\nYou better not cry\nBetter not pout\nI'm telling you why\nSanta Claus is coming to town\n\nHe's making a list, And checking it twiceGonna find out Who's naughty\nAnd nice. Santa Claus is coming to town\n\nHe sees you when you're sleeping\nHe knows when you're awake\nHe knows if you've been bad or good\nSo be good for goodness sake\n\nOh! You better watch out! You better not cry. Better not pout, I'm telling\nYou why. Santa Claus is coming to town\nSanta Claus is coming to townEmbedShare URLCopyEmbedCopy"
                    }
                }
            }"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        println!("{response:?}");
        if let ResponseType::Lyrics { lyrics } = response.data {
            assert_eq!(lyrics.artist.as_deref(), Some("Ella Fitzgerald"));
            assert_eq!(
                lyrics.title.as_deref(),
                Some("Santa Claus Is Coming to Town")
            );
        } else {
            panic!("wrong type: {response:?}");
        }
    }
}
