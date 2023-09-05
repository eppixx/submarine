use std::collections::HashMap;

/// Builder for [Auth]<br>
/// Example:
/// ```
/// use submarine::auth::AuthBuilder;
/// let auth = AuthBuilder::new("peter", "v0.16.1")
///     .client_name("my_music_app")
///     .hashed("change_me_password");
/// ```
#[derive(Default)]
pub struct AuthBuilder {
    user: String,
    version: String,
    client_name: Option<String>,
}

impl AuthBuilder {
    /// version should match the one closely matching at feature level
    pub fn new(user: &str, version: &str) -> Self {
        Self {
            user: String::from(user),
            version: String::from(version),
            ..Default::default()
        }
    }

    /// give the client a name; defaults to "submarine-lib"
    pub fn client_name(mut self, name: &str) -> Self {
        self.client_name = Some(String::from(name));
        self
    }

    /// hash plain password for storing and subsequently uses
    /// ; consumes self
    pub fn hashed(self, password: &str) -> Auth {
        let (salt, hash) = Auth::hash(password);
        Auth {
            user: self.user,
            version: self.version,
            client_name: self.client_name.unwrap_or(String::from("submarine-lib")),
            hash,
            salt,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Auth {
    pub user: String,
    pub version: String,
    pub client_name: String,
    pub hash: String,
    pub salt: String,
}

impl Auth {
    fn hash(password: &str) -> (String, String) {
        let salt = Self::create_salt(24);
        let hash = md5::compute(password.to_owned() + &salt);
        (format!("{:x}", hash), salt)
    }

    fn create_salt(size: usize) -> String {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};
        use std::iter;

        let mut rng = thread_rng();
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(size)
            .collect()
    }

    pub(crate) fn add_parameter(&self, paras: &mut HashMap<&str, String>) {
        paras.insert("u", self.user.clone());
        paras.insert("v", self.version.clone());
        paras.insert("c", self.client_name.clone());
        paras.insert("t", self.hash.clone());
        paras.insert("s", self.salt.clone());
        paras.insert("f", String::from("json"));
    }
}
