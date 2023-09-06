use crate::{
    data::{Info, ResponseType},
    Client, SubsonicError, Parameter,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#scrobble
    pub async fn scrobble(
        &self,
        id_at_time: Vec<(impl Into<String>, Option<usize>)>,
        submission: Option<bool>, // defaults to true
    ) -> Result<Info, SubsonicError> {
        let mut paras = Parameter::new();
        for (id, time) in id_at_time {
            paras.push("id", id);
            if let Some(time) = time {
                paras.push("time", time.to_string());
            }
        }
        if let Some(submission) = submission {
            paras.push("submission", submission.to_string());
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
