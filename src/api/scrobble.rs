use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#scrobble
    pub async fn scrobble(
        &self,
        id_at_time: Vec<(impl Into<String>, Option<usize>)>,
        submission: Option<bool>, // defaults to true
    ) -> Result<Info, SubsonicError> {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        for (id, time) in id_at_time {
            paras.insert("id", id.into());
            if let Some(time) = time {
                paras.insert("time", time.to_string());
            }
        }
        if let Some(submission) = submission {
            paras.insert("submission", submission.to_string());
        }

        let body = self.request("scrobble", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn ping_convert() {
        let response_txt = r##"
{
    "subsonic-response": {
        "status": "ok",
        "version": "1.16.1",
        "type": "navidrome",
        "serverVersion": "0.49.3 (8b93962f)"
    }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_txt)
            .unwrap()
            .inner;
        if let ResponseType::Ping {} = response.data {
            assert_eq!(&response.info.status, "ok");
        } else {
            panic!("wrong type");
        }
    }
}
