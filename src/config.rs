use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
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
    pub PIPELINE_ID: i64,
    pub FUNNEL: i64,
    // -- AmoCRM
    pub AMO_CITY_ACCOUNT: String,
    pub AMO_CITY_TOKEN: String,
    // -- Profitbase
    pub PROF_CITY_ACCOUNT: String,
    pub PROF_CITY_API_KEY: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        let mode = "standalone";
        if mode == "standalone_" {
            Ok(Config {
                PIPELINE_ID: 10192498,
                FUNNEL: 80709866,
                AMO_CITY_ACCOUNT: "dnscity".to_string(),
                AMO_CITY_TOKEN: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImp0aSI6IjI0MzAxNmJjODA1NDg3N2IyNmY2NTNmMjg1NDBiMjA3YjdhYzZjYTUzZDA4MWZkZmFmNjc0ZThhNmMzOGQ5MDM3ZjI5ZDI3OTA0NTU2NjExIn0.eyJhdWQiOiI4MjVjMjI3OC1lZTRiLTRlMzMtOTIzYS04ZmMyMmQyMWViNzYiLCJqdGkiOiIyNDMwMTZiYzgwNTQ4NzdiMjZmNjUzZjI4NTQwYjIwN2I3YWM2Y2E1M2QwODFmZGZhZjY3NGU4YTZjMzhkOTAzN2YyOWQyNzkwNDU1NjYxMSIsImlhdCI6MTc0Mjc5NDQzNiwibmJmIjoxNzQyNzk0NDM2LCJleHAiOjE5MDA1NDA4MDAsInN1YiI6IjIxNzAzMTgiLCJncmFudF90eXBlIjoiIiwiYWNjb3VudF9pZCI6MzE0MTM0MjIsImJhc2VfZG9tYWluIjoiYW1vY3JtLnJ1IiwidmVyc2lvbiI6Miwic2NvcGVzIjpbImNybSIsImZpbGVzIiwiZmlsZXNfZGVsZXRlIiwibm90aWZpY2F0aW9ucyIsInB1c2hfbm90aWZpY2F0aW9ucyJdLCJoYXNoX3V1aWQiOiI3YTBmY2Q5Ny1kMDM0LTQ1NTYtODhmOS00ZGVmZWY2NzFlOGUiLCJhcGlfZG9tYWluIjoiYXBpLWIuYW1vY3JtLnJ1In0.IiWJeBqWcyHmFdJT8S87FnqJjU8xlI3njVJUe4RFHnER7Tx4wSuAnKxz19ZIkVHp8S4ssQusziSXdnNllCuR0rLOOtBnP8E_9lZbPAjdMueQ72KyaUhkDESD25C4zpqcTQNMDG7vMgk1fsfa4jQB9KT3wGM2XvP5fPd1A9Dv6QonD7lDCgU5YRkUktZ32yKOVN4eJXWcTxuhkC-EsMoGe5z-edimmTwigO4OL-vRqXPVIcE-v1YEFbdPrX_PSUiWeaFGy33O3G2rfTvUPAjtrDxTBzMoTuV5MS03J5zlY4_mzCA9Mf4zQnoLq8PpuLp3dLaLel1GiDNpVPV3t5ZsIQ".to_string(),
                PROF_CITY_ACCOUNT: "pb18549".to_string(),
                PROF_CITY_API_KEY: "app-658c0ccb38bbf".to_string(),
            })
        } else {
            dotenv().expect("dotenv init failed");
            Ok(Config {
                PIPELINE_ID: get_env_as_parse("PIPELINE_ID")?,
                FUNNEL: get_env_as_parse("FUNNEL")?,
                AMO_CITY_ACCOUNT: get_env("AMO_CITY_ACCOUNT")?,
                AMO_CITY_TOKEN: get_env("AMO_CITY_TOKEN")?,
                PROF_CITY_ACCOUNT: get_env("PROF_CITY_ACCOUNT")?,
                PROF_CITY_API_KEY: get_env("PROF_CITY_API_KEY")?,
            })
        }
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn get_env_as_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}
