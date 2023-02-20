use sled::Tree;
use async_session::{Result, SessionStore, async_trait};

#[derive(Debug, Clone)]
pub struct SledSessionStore {
    store: Tree,
}

impl SledSessionStore {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let store = db.open_tree("sessions")?;
        Ok(SledSessionStore { store })
    }
}

#[async_trait]
impl SessionStore for SledSessionStore {
    fn load_session<'life0,'async_trait>(&'life0 self,cookie_value:String) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<async_session::Session> > > + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn store_session<'life0,'async_trait>(&'life0 self,session:async_session::Session) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result<Option<String> > > + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn destroy_session<'life0,'async_trait>(&'life0 self,session:async_session::Session) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result> + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }

    fn clear_store<'life0,'async_trait>(&'life0 self) ->  core::pin::Pin<Box<dyn core::future::Future<Output = Result> + core::marker::Send+'async_trait> >where 'life0:'async_trait,Self:'async_trait {
        todo!()
    }
}