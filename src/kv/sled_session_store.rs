use sled::{Tree, IVec};
use async_session::{Result, SessionStore, async_trait, Session, serde_json};

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
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
      
        match self.store.get(id)? {
            Some(data) => { 
                let data = std::str::from_utf8(&data[..])?;
                let json_data: Session = serde_json::from_str(data)?;
                Ok(Some(json_data))
             }
            None => {
                Ok(None)
            }
        }
    }    

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = session.id();
        let json_cookie: String = serde_json::to_string(&session)?;
        let _ = self.store.insert(id, IVec::from(&json_cookie[..]))?;
        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let id = session.id();
        self.store.remove(id);
        Ok(())
    }
    

    async fn clear_store(&self) -> Result {
        let mut iter = self.store.iter();
        iter.for_each(|result| {
            let (k, _) = result.unwrap();
            self.store.remove(k);
        });
        Ok(())
    }

}