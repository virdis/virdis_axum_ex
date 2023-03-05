use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Sledpath {
    pub path: String,
}
// TODO - Come up with a better name
#[derive(Debug, Deserialize)]
pub struct AUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Salt {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub auser: AUser,
    pub salt: Salt,
    pub sledpath: Sledpath,
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
    use rand::seq::SliceRandom;

    use super::*;

    #[test]
    fn test_settings_config() -> Result<(), ()> {
        let settings = Settings::new().expect("failed to create Settings");
        dbg!(settings);
        assert_eq!(1, 1);
        Ok(())
    }
}
