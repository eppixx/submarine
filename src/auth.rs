use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Auth {
    pub user: String,
    pub version: String,
    pub client_name: String,
    pub hash: String,
    pub salt: String,
}

#[derive(Default)]
pub struct AuthBuilder {
    user: String,
    version : String,
    client_name: Option<String>,
}

impl AuthBuilder {
    /// version should match the one closely matching at feature level
    pub fn new(user: &str, version: &str) -> AuthBuilder {
        AuthBuilder {
            user: String::from(user),
            version: String::from(version),
            ..Default::default()
        }
    }

    pub fn client_name(mut self, name: &str) -> AuthBuilder {
        self.client_name = Some(String::from(name));
        self
    }

    pub fn hashed(self, password: &str) -> Auth {
        let (salt, hash) = Auth::hash(password);
        Auth {
            user: self.user,
            version: self.version,
            client_name: self.client_name.unwrap_or(String::from("submarine")),
            hash,
            salt,
        }
    }
}

impl Auth {
    /// create a new auth object
    pub fn with_hash(user: &str, hash: String, salt: String) -> Self {
        Self {
            user: String::from(user),
            version: String::from("0.16.0"),
            client_name: String::from("audio-ui"), //TODO change
            hash,
            salt,
        }
    }

    /// hash plain password for storing and subsequently uses
    pub fn hash(password: &str) -> (String, String) {
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
}

impl From<Auth> for HashMap<&str, String> {
    fn from(auth: Auth) -> Self {
        let mut params = HashMap::new();
        params.insert("u", auth.user.clone());
        params.insert("v", auth.version.clone());
        params.insert("c", auth.client_name.clone());
        params.insert("t", auth.hash.clone());
        params.insert("s", auth.salt);
        params.insert("f", "json".into());
        params
    }
}
