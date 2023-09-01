use crate::Client;

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#getCatptions
    pub fn get_captions(
        &self,
        id: impl Into<String>,
        format: Option<impl Into<String>>,
    ) -> String {
        let mut paras: std::collections::HashMap<&str, String> = self.auth.clone().into();
        paras.insert("id", id.into());
        if let Some(format) = format {
            paras.insert("format", format.into());
        }

        let mut url: String = self.server_url.clone() + "/rest/getCaptions?";
        for p in paras {
            url += &("&".to_owned() + p.0 + "=" + &p.1);
        }

        url
    }
}
