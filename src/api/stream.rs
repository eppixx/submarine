use crate::{Client, SubsonicError};

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
    ) -> String {
        let mut paras = std::collections::HashMap::new();
        self.auth.add_parameter(&mut paras);
        paras.insert("id", id.into());
        if let Some(bit_rate) = max_bit_rate {
            paras.insert("maxBitRate", bit_rate.to_string());
        }
        if let Some(format) = format {
            paras.insert("format", format.into());
        }
        if let Some(offset) = time_offset {
            paras.insert("timeOffset", offset.to_string());
        }
        if let Some(size) = size {
            paras.insert("size", size.into());
        }
        if let Some(content_length) = estimate_content_length {
            paras.insert("estimateContentLength", content_length.to_string());
        }
        if let Some(converted) = converted {
            paras.insert("converted", converted.to_string());
        }

        let mut url: String = self.server_url.clone() + "/rest/stream?";
        for p in paras {
            url += &("&".to_owned() + p.0 + "=" + &p.1);
        }

        url
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
        ))
        .await
        {
            Ok(result) => result,
            Err(e) => return Err(SubsonicError::Connection(e)),
        };

        let bytes = result.bytes().await?.to_vec();
        Ok(bytes)
    }
}
