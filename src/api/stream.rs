use crate::Client;

impl Client {
    pub fn get_stream_url(&self, id: &str) -> String {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("id", String::from(id));

        let mut url: String = self.server_url.clone() + "/rest/stream?";
        for p in paras {
            url += &("&".to_owned() + p.0 + "=" + &p.1);
        }

        url
    }
}
