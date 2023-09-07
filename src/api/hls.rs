use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#stream
    pub fn hls_url(
        &self,
        id: impl Into<String>,
        bit_rate: Vec<i32>,
        audio_rate: Option<impl Into<String>>,
    ) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("id", id);
        for bit_rate in bit_rate {
            paras.push("bitRate", bit_rate.to_string());
        }
        if let Some(audio_rate) = audio_rate {
            paras.push("audioRate", audio_rate);
        }

        url::Url::parse_with_params(
            &format!("{}{}", self.server_url.clone(), "/rest/hls.m3u8"),
            paras.0,
        )
    }

    pub async fn hls(
        &self,
        id: impl Into<String>,
        bit_rate: Vec<i32>,
        audio_rate: Option<impl Into<String>>,
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.hls_url(id, bit_rate, audio_rate)?).await {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

//TODO add function that downloads m3u8 playlist

#[cfg(test)]
mod tests {
    use crate::{auth::AuthBuilder, Client};

    #[tokio::test]
    async fn create_hls_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://target.com", auth);
        let url = client
            .hls_url(String::from("testId"), vec![128, 196], None::<&str>)
            .unwrap();

        assert_eq!("https://target.com/rest/hls.m3u8?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId&bitRate=128&bitRate=196", &url.to_string());
    }
}
