use async_session::serde_json;
use axum_login::{secrecy::SecretVec, AuthUser};
use serde::{Deserialize, Serialize};
use sled::IVec;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Role {
    User,
    Admin,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub (crate) username: String,
    pub (crate) password_hash: String,
}

impl AuthUser for User {
    fn get_id(&self) -> String {
        let hash = twox_hash::xxh3::hash64(self.username.as_ref());
        format!("{}", hash)
    }

    fn get_password_hash(&self) -> axum_login::secrecy::SecretVec<u8> {
        SecretVec::new(self.password_hash.clone().into())
        }
}

impl From<IVec> for User {
    fn from(value: IVec) -> Self {
        let str = std::str::from_utf8(value.as_ref()).expect("failed to convert IVec to User");
        let user: User = serde_json::from_str(str).expect("failed to convert str to CustomUser");
        user
    }
}
