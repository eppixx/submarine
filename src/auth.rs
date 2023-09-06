use crate::Parameter;

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
    salt: Option<String>,
    client_name: Option<String>,
}

impl AuthBuilder {
    /// version should match the one closely matching at feature level
    pub fn new(user: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            version: version.into(),
            ..Default::default()
        }
    }

    /// give the client a name; defaults to "submarine-lib"
    pub fn client_name(mut self, name: &str) -> Self {
        self.client_name = Some(String::from(name));
        self
    }

    /// specifiy salt instead of generated
    /// mostly useful for tests so that the outcome is determinante
    pub(crate) fn _salt(mut self, salt: impl Into<String>) -> Self {
        self.salt = Some(salt.into());
        self
    }

    /// hash plain password for storing and subsequently uses
    /// ; consumes self
    pub fn hashed(self, password: &str) -> Auth {
        let (hash, salt) = if let Some(salt) = self.salt {
            (Auth::hash_with_salt(password, &salt), salt)
        } else {
            Auth::hash(password)
        };

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

    fn hash_with_salt(password: impl Into<String>, salt: &str) -> String {
        let hash = md5::compute(password.into() + salt);
        format!("{:x}", hash)
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

    pub(crate) fn add_parameter(&self, paras: &mut Parameter) {
        paras.push("u", &self.user);
        paras.push("v", &self.version);
        paras.push("c", &self.client_name);
        paras.push("t", &self.hash);
        paras.push("s", &self.salt);
        paras.push("f", "json");
    }
}
