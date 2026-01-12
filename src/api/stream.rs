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
        match reqwest::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("could not build reqwest client with timeout")
            .get(self.stream_url(
                id,
                max_bit_rate,
                format,
                time_offset,
                size,
                estimate_content_length,
                converted,
            )?)
            .send()
            .await
        {
            Err(e) => Err(SubsonicError::Connection(e)),
            Ok(result) => match result.error_for_status() {
                Ok(result) => {
                    let bytes = result.bytes().await?.to_vec();
                    Ok(bytes)
                },
                Err(e) => Err(SubsonicError::Connection(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{auth::AuthBuilder, Client};

    #[test]
    fn create_stream_url() {
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("https://target.com", auth);
        let url = client
            .stream_url("testId", None, None::<&str>, None, None::<&str>, None, None)
            .unwrap();

        assert_eq!("https://target.com/rest/stream?u=peter&v=v0.16.1&c=submarine-lib&t=d4a5b2db9781fba37ec95f0312ade67a&s=&f=json&id=testId", &url.to_string());
    }

    #[tokio::test]
    async fn assert_timeout() {
        let (tx, mut rx) = tokio::sync::oneshot::channel();

        // start a mock server we control which accepts everything but sends nothing,
        // making sure our stream request times out
        let listener = tokio::net::TcpListener::bind("127.0.0.1:58491").await.expect("could not open listener");
        tokio::spawn(async move {
            let mut hanging_sockets = Vec::new();
            loop {
                tokio::select! {
                    biased;

                    res = listener.accept() => match res {
                        Ok((sock, addr)) => {
                            hanging_sockets.push((sock, addr));
                        },
                        Err(e) => eprintln!("error accepting client: {e}"),

                    },

                    _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {},
                }

                if matches!(rx.try_recv(), Ok(()) | Err(tokio::sync::oneshot::error::TryRecvError::Closed)) {
                    break;
                }
            }
        });

        // actually make the stream request
        let auth = AuthBuilder::new("peter", "v0.16.1")
            ._salt("")
            .hashed("change_me_password");
        let client = Client::new("http://127.0.0.1:58491", auth);

        let before = std::time::SystemTime::now();
        let res = client.stream(
            "",
            None,
            None::<String>,
            None,
            None::<String>,
            None,
            None
        )
            .await;
        let delta = before.elapsed().expect("could not calculate duration: unstable system clock");
        tx.send(()).expect("server stopped?"); // stop the mock server

        assert!(res.is_err());
        let err = res.expect_err("not an error after check?");
        assert!(matches!(err, crate::SubsonicError::Connection(_)));
        let crate::SubsonicError::Connection(inner) = err else {
            panic!("not a subsonic error after check?");
        };
        assert!(inner.is_timeout());
        assert!(delta.as_secs() >= 30 && delta.as_secs() <= 31); // 1s of "grace"
    }
}
