use crate::{
    data::{InternetRadioStation, ResponseType},
    Client, SubsonicError,
};

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getInternetRadioStations
    pub async fn get_internet_radio_stations(
        &self,
    ) -> Result<Vec<InternetRadioStation>, SubsonicError> {
        let paras: std::collections::HashMap<&str, String> = self.auth.clone().into();

        let body = self
            .request("getInternetRadioStations", Some(paras), None)
            .await?;
        if let ResponseType::InternetRadionStations {
            internet_radio_stations,
        } = body.data
        {
            Ok(internet_radio_stations.internet_radio_station)
        } else {
            Err(SubsonicError::Submarine(String::from(
                "expected type InternetRadioStations but found wrong type",
            )))
        }
    }
}

//TODO add better test
#[cfg(test)]
mod tests {
    use crate::data::{OuterResponse, ResponseType};

    #[test]
    fn conversion_empty_get_internet_radio_stations() {
        let response_body = r##"
{
    "subsonic-response": {
        "status": "ok",
        "version": "1.16.1",
        "type": "navidrome",
        "serverVersion": "0.49.3 (8b93962f)",
        "internetRadioStations": {}
    }
}"##;
        let response = serde_json::from_str::<OuterResponse>(response_body)
            .unwrap()
            .inner;
        if let ResponseType::InternetRadionStations {
            internet_radio_stations,
        } = response.data
        {
            assert_eq!(internet_radio_stations.internet_radio_station, vec![]);
        } else {
            panic!("wrong type");
        }
    }
}
