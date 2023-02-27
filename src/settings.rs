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
struct User {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    user: User,
    blogstore: Blogstore,
    metastore: Metastore,
    sessionstore: Sessionstore,
    userstore: Userstore,
}

impl Settings {
    fn new() -> Result<Self, ConfigError> {
        // TODO: Add file location for production settings
        let production_settings: Result<Config, ConfigError> = Config::builder()
            .add_source(config::File::with_name(""))
            .build();

        let default_settings: Result<Config, ConfigError> = Config::builder()
            .add_source(config::File::with_name("src/config/default"))
            .build();

        production_settings
            .or_else(|_| {
                println!("Loading Default Settings ......");
                default_settings
            })
            .and_then(|c: Config| c.try_deserialize())
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
        assert_eq!(1, 1);
        Ok(())
    }
}
