use std::{marker::PhantomData, rc::Rc, sync::Arc};

use async_session::{async_trait, serde_json};
use axum_login::{AuthUser, UserStore};
use sled::{IVec, Tree};

use super::auth_user::BlogAuthor;
use crate::settings::{AUser, Salt};

#[derive(Clone, Debug)]
pub struct SledUserStore<User, Role = ()> {
    inner: Arc<Tree>,
    _user_type: PhantomData<User>,
    _role_type: PhantomData<Role>,
}

impl<User, Role> SledUserStore<User, Role> {
    pub fn new(inner: Arc<Tree>) -> Self {
        Self {
            inner: inner,
            _user_type: Default::default(),
            _role_type: Default::default(),
        }
    }

    pub fn setup(&self, auser: AUser, salt: Salt) {
        let hashed_password = argon2::hash_encoded(
            auser.password.as_ref(),
            salt.value.as_ref(),
            &argon2::Config::default(),
        )
        .unwrap();
        let user: BlogAuthor = BlogAuthor {
            username: auser.username,
            password_hash: hashed_password,
        };
        let json_user = serde_json::to_string(&user).expect("failed to convert User to String");
        self.inner
            .insert(user.get_id(), IVec::from(json_user.as_bytes()));
    }
}

#[async_trait]
impl<User, Role> UserStore<Role> for SledUserStore<User, Role>
where
    Role: PartialOrd + PartialEq + Clone + Send + Sync + 'static,
    User: AuthUser<Role> + From<IVec> + Clone + Send + Sync + 'static,
{
    type User = User;

    async fn load_user(&self, user_id: &str) -> Result<Option<Self::User>, eyre::Report> {
        let opt_user = self
            .inner
            .get(user_id)
            .expect("failed to find data for user_id");
        let user: Option<User> = opt_user.map(|u| u.into());
        match user {
            Some(u) => Ok(Some(u)),
            None => Err(eyre::eyre!("Could not find user by user_id: {:?}", user_id)),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use assert_fs::TempDir;
    use async_session::serde_json;
    use async_std::task;

    type TResult = Result<(), ()>;

    async fn user_store() -> (SledUserStore<BlogAuthor, ()>, TempDir) {
        let temp = TempDir::new().unwrap();
        let db = sled::open(temp.path()).expect("failed to create Db from path");
        let store = db
            .open_tree("users")
            .expect("failed to create Tree from db");
        let store: SledUserStore<BlogAuthor> = SledUserStore::new(Arc::new(store));
        (store, temp)
    }

    #[async_std::test]
    async fn test_load_user() -> TResult {
        let (user_store, temp) = user_store().await;
        let u = BlogAuthor {
            username: "username".to_string(),
            password_hash: "password".to_string(),
        };
        let id = u.get_id();
        let id = id.as_ref();
        let json_user = serde_json::to_string(&u).expect("failed to convert User to string");
        let _ = user_store
            .inner
            .insert(id, IVec::from(json_user.as_bytes()));
        let u_db = user_store
            .load_user(&id)
            .await
            .expect("failed to find user by id");
        let u_db = u_db.unwrap();
        assert_eq!(u_db.username, u.username);
        assert_eq!(u_db.password_hash, u.password_hash);
        temp.close().unwrap();
        Ok(())
    }
}
