use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| {
            panic!("FATAL - WHILE LOADING Config -cause: {:?}", err);
        })
    })
}

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct Config {
    // -- AmoCRM
    pub AMO_CITY_ACCOUNT: String,
    pub AMO_CITY_TOKEN: String,
    pub AMO_FORMAT_ACCOUNT: String,
    pub AMO_FORMAT_TOKEN: String,
    // -- Profitbase
    pub PROF_CITY_ACCOUNT: String,
    pub PROF_CITY_API_KEY: String,
    pub PROF_FORMAT_ACCOUNT: String,
    pub PROF_FORMAT_API_KEY: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        dotenv().expect("dotenv init failed");
        Ok(Config {
            AMO_CITY_ACCOUNT: get_env("AMO_CITY_ACCOUNT")?,
            AMO_CITY_TOKEN: get_env("AMO_CITY_TOKEN")?,
            AMO_FORMAT_ACCOUNT: get_env("AMO_FORMAT_ACCOUNT")?,
            AMO_FORMAT_TOKEN: get_env("AMO_FORMAT_TOKEN")?,
            PROF_CITY_ACCOUNT: get_env("PROF_CITY_ACCOUNT")?,
            PROF_CITY_API_KEY: get_env("PROF_CITY_API_KEY")?,
            PROF_FORMAT_ACCOUNT: get_env("PROF_FORMAT_ACCOUNT")?,
            PROF_FORMAT_API_KEY: get_env("PROF_FORMAT_API_KEY")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
