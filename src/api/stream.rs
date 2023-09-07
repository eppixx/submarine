use crate::{Client, Parameter, SubsonicError};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#stream
    pub fn stream_url(
        &self,
        id: impl Into<String>,
        max_bit_rate: Option<i32>,             // 0 for no limit
        format: Option<impl Into<String>>,     // file ending, raw for disable
        time_offset: Option<i64>,              // video only
        size: Option<impl Into<String>>,       // video only in "widthxheight" format
        estimate_content_length: Option<bool>, // restrict length
        converted: Option<bool>,               // video only
    ) -> Result<url::Url, url::ParseError> {
        let mut paras = Parameter::new();
        self.auth.add_parameter(&mut paras);
        paras.push("id", id);
        if let Some(bit_rate) = max_bit_rate {
            paras.push("maxBitRate", bit_rate.to_string());
        }
        if let Some(format) = format {
            paras.push("format", format);
        }
        if let Some(offset) = time_offset {
            paras.push("timeOffset", offset.to_string());
        }
        if let Some(size) = size {
            paras.push("size", size);
        }
        if let Some(content_length) = estimate_content_length {
            paras.push("estimateContentLength", content_length.to_string());
        }
        if let Some(converted) = converted {
            paras.push("converted", converted.to_string());
        }

        url::Url::parse_with_params(&format!("{}/rest/stream", self.server_url), paras.0)
    }

    /// reference: http://www.subsonic.org/pages/api.jsp#stream
    pub async fn stream(
        &self,
        id: impl Into<String>,
        max_bit_rate: Option<i32>,             // 0 for no limit
        format: Option<impl Into<String>>,     // file ending, raw for disable
        time_offset: Option<i64>,              // video only
        size: Option<impl Into<String>>,       // video only in "widthxheight" format
        estimate_content_length: Option<bool>, // restrict length
        converted: Option<bool>,               // video only
    ) -> Result<Vec<u8>, SubsonicError> {
        let result = match reqwest::get(self.stream_url(
            id,
            max_bit_rate,
            format,
            time_offset,
            size,
            estimate_content_length,
            converted,
        )?)
        .await
        {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::AuthBuilder, Client};

    #[tokio::test]
    async fn create_stream_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://target.com", auth);
        let url = client
            .stream_url("testId", None, None::<&str>, None, None::<&str>, None, None)
            .unwrap();

        assert_eq!("https://target.com/rest/stream?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId", &url.to_string());
    }
}
