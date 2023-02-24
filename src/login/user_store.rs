use std::{marker::PhantomData};

use axum_login::{UserStore, AuthUser};
use sled::{Tree, IVec};
use super::auth_user::*; 
use async_session::async_trait;
use std::io::{ Error, ErrorKind };


#[derive(Clone, Debug)]
pub struct SledUserStore<User, Role = ()> {
    inner: Tree,
    _user_type: PhantomData<User>,
    _role_type: PhantomData<Role>,
}

impl<User, Role> SledUserStore<User, Role> {
    fn new(inner: Tree) -> Self {
        Self { 
            inner,
            _user_type: Default::default(),
            _role_type: Default::default(), 
        }
    }
}

#[async_trait]
impl <User, Role> UserStore<Role> for SledUserStore<User, Role> 
where 
    Role: PartialOrd + PartialEq + Clone + Send + Sync + 'static,
    User: AuthUser<Role> + From<IVec> + Clone + Send + Sync + 'static

{
    type User = User;

    async fn load_user(&self, user_id: &str) -> Result<Option<Self::User>, eyre::Report> {
        let opt_user = self.inner.get(user_id).expect("failed to find data for user_id");
        let user: Option<User> = opt_user.map(|u| { u.into() });
        match user {
            Some(u) => Ok(Some(u)),
            None => {
                Err(eyre::eyre!("Could not find user by user_id: {:?}", user_id))
            }
        }
    }
}
