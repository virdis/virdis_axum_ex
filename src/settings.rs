use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sledpath {
    pub path: String,
}
#[derive(Debug, Deserialize)]
pub struct Metastore {
    pub keyspace: String,
}

#[derive(Debug, Deserialize)]
pub struct Bodystore {
    pub keyspace: String,
}

#[derive(Debug, Deserialize)]
pub struct Sessionstore {
    pub keyspace: String,
}

#[derive(Debug, Deserialize)]
pub struct Userstore {
    pub keyspace: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub user: User,
    pub sledpath: Sledpath,
    pub bodystore: Bodystore,
    pub metastore: Metastore,
    pub sessionstore: Sessionstore,
    pub userstore: Userstore,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
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
