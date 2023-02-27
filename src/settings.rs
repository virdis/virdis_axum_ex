use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metastore {
    key_space_name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct Blogstore {
    key_space_name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct Sessionstore {
    key_space_name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct Userstore {
    key_space_name: String,
    path: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    blogstore: Blogstore,
    metastore: Metastore,
    sessionstore: Sessionstore,
    userstore: Userstore,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
        .add_source(config::File::with_name("/home/virdis/Source/rust/virdis_me/src/config/default"))
        .build()?;
        settings.try_deserialize()
    }
}

#[cfg(test)]
mod test {
    use config::Config;

    use super::*;
    
    #[test]
    fn test_settings_config() -> Result<(), ()> {
        let settings = Settings::new().expect("failed to create Settings");
        dbg!(settings);
        assert_eq!(1,1);
        Ok(())
    }
}