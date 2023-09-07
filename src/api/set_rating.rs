use crate::{
    data::{Info, ResponseType},
    Client, Parameter, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#setRating
    pub async fn set_rating(
        &self,
        id: impl Into<String>,
        rating: i8,
    ) -> Result<Info, SubsonicError> {
        if !(0..=5).contains(&rating) {
            return Err(SubsonicError::InvalidArgs(String::from(
                "rating must between 0-5",
            )));
        }

        let mut paras = Parameter::new();
        paras.push("id", id);
        paras.push("rating", rating.to_string());

        let body = self.request("star", Some(paras), None).await?;
        if let ResponseType::Ping {} = body.data {
            Ok(body.info)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type Ping but found wrong type",
            )))
        }
    }
}
