pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load<T: serde::de::DeserializeOwned>() -> Result<T, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(config::File::with_name("./config/default").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;
        s.try_deserialize()
    }

    pub fn file<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(config::File::with_name(path))
            .build()?;
        s.try_deserialize()
    }

    pub fn env<T: serde::de::DeserializeOwned>(prefix: &str) -> Result<T, config::ConfigError> {
        let s = config::Config::builder()
            .add_source(config::Environment::with_prefix(prefix))
            .build()?;
        s.try_deserialize()
    }
}
