//! Loads the credentials from environment variables

use std::env::var;

use backend::user::User;

/// Credentials to log in to the homeserver.
pub struct Credentials<T> {
    /// Homeserver URL
    ///
    /// Can be `http://localhost:<port>`
    pub homeserver: T,
    /// Homeserver password
    pub password:   T,
    /// Homeserver username
    pub username:   T,
}

impl Credentials<Option<String>> {
    /// Populates a [`Self`] from environment variables
    pub fn from_env() -> Self {
        Self {
            homeserver: var("HOMESERVER_URL").ok(),
            username:   var("USERNAME").ok(),
            password:   var("PASSWORD").ok(),
        }
    }

    /// Checks if all values are set
    pub const fn is_full(&self) -> bool {
        self.homeserver.is_some()
            && self.username.is_some()
            && self.password.is_some()
    }

    /// Fill the unknown fields with an empty string
    pub fn fill_with_empty(self) -> Credentials<String> {
        Credentials {
            homeserver: self.homeserver.unwrap_or_default(),
            username:   self.username.unwrap_or_default(),
            password:   self.password.unwrap_or_default(),
        }
    }
}

impl Credentials<String> {
    pub async fn login(self) -> color_eyre::Result<User> {
        let mut user = User::new(&self.homeserver).await?;
        user.login(self.username, &self.password).await?;
        Ok(user)
    }
}
