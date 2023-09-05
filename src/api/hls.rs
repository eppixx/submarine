use crate::Client;

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#stream
    pub fn hls_url(
        &self,
        id: impl Into<String>,
        bit_rate: Vec<i32>,
        audio_rate: Option<impl Into<String>>,
    ) -> String {
        let mut paras = std::collections::HashMap::new();
        self.auth.add_parameter(&mut paras);
        paras.insert("id", id.into());
        for bit_rate in bit_rate {
            paras.insert("bitRate", bit_rate.to_string());
        }
        if let Some(audio_rate) = audio_rate {
            paras.insert("audioRate", audio_rate.into());
        }

        let mut url: String = self.server_url.clone() + "/rest/hls.m3u8?";
        for p in paras {
            url += &("&".to_owned() + p.0 + "=" + &p.1);
        }

        url
    }
}

//TODO add function that downloads m3u8 playlist
//TODO add test
