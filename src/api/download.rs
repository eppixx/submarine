use crate::Client;

impl Client {
    /// reference: http://www.subsonic.org/pages/api.jsp#download
    pub fn download(&self, id: impl Into<String>) -> String {
        format!("{}/rest/download?{}", self.server_url, id.into())
    }
}
