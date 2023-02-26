use async_session::{async_trait, serde_json, Result, Session, SessionStore};
use sled::{IVec, Tree};

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
            None => Ok(None),
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use assert_fs::TempDir;
    use async_std::task;
    use axum_sessions::async_session::chrono::{Days, Utc};

    async fn test_store() -> (SledSessionStore, TempDir) {
        let temp = TempDir::new().unwrap();
        let mut path = temp.path().as_os_str().to_str().unwrap();
        let store = SledSessionStore::new(path).unwrap();
        (store, temp)
    }

    #[async_std::test]
    async fn create_new_session() -> Result {
        let (store, dir) = test_store().await;
        let mut session = Session::new();
        let duration = Utc::now();
        let duration_add = duration.checked_add_days(Days::new(2)).unwrap();
        session.set_expiry(duration_add);
        let mut book_reviews = HashMap::new();

        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        book_reviews.insert(
            "Grimms' Fairy Tales".to_string(),
            "Masterpiece.".to_string(),
        );
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );
        book_reviews.insert(
            "The Adventures of Sherlock Holmes".to_string(),
            "Eye lyked it alot.".to_string(),
        );

        session.insert("book_reviews", book_reviews)?;
        let cloned = session.clone();
        let cookie_value = store.store_session(session).await?.unwrap();

        let loaded_session = store.load_session(cookie_value).await?.unwrap();
        assert_eq!(cloned.id(), loaded_session.id());

        dir.close().unwrap();
        Ok(())
    }

    #[async_std::test]
    async fn destroy_session() -> Result {
        let (store, dir) = test_store().await;
        let mut session = Session::new();
        let id = session.id();
        session.insert("book_reviews_destroy", "lost all books ........")?;
        let cookie_value = store.store_session(session).await?.unwrap();
        let mut iter = store.store.iter();

        let no_present = store.load_session(cookie_value.clone()).await?.unwrap();
        let result = store.destroy_session(no_present).await?;
        assert_eq!((), result);
        iter.for_each(|res| {
            let (key, _) = res.unwrap();
            let s = std::str::from_utf8(&key[..]).unwrap();
            println!("Iterating over keys: {:?}", s)
        });
        dir.close().unwrap();
        Ok(())
    }
}
