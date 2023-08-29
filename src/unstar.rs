use crate::star::StarTarget;
use crate::{Client, SubsonicError};

impl Client {
    /// Quelle: http://www.subsonic.org/pages/api.jsp#unstar
    pub async fn unstar(&self, id: &str, target: StarTarget) -> Result<(), SubsonicError> {
        let mut paras = std::collections::HashMap::new();
        let key = target.to_string();
        paras.insert(key.as_str(), String::from(id));

        let _ = self.request("unstar", Some(paras), None).await?;
        Ok(())
    }
}
