use std::env;
use std::sync::OnceLock;

use anyhow::anyhow;
use url::Url;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load().unwrap_or_else(|ex| panic!("ERROR WHILE LOADING CONF: {ex:?}"))
    })
}

trait ConfigLoader {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized;
}

#[allow(non_snake_case)]
pub struct Config {
    pub OAUTH: OauthConfig,
    pub SERVER: ServerConfig,
}

#[allow(non_snake_case)]
pub struct OauthConfig {
    pub AUDIENCE: Url,
    pub SERVER: Url,
    pub TOKEN: Url,
    pub API: Url,
    pub USER_INFO: Url,

    pub CLIENT_ID: String,
    pub CLIENT_SECRET: String,
    pub REDIRECT_URI: String,
}

impl ConfigLoader for Config {
    fn load() -> anyhow::Result<Config> {
        Ok(Config {
            OAUTH: OauthConfig::load()?,
            SERVER: ServerConfig::load()?,
        })
    }
}

impl ConfigLoader for OauthConfig {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let audience = Url::parse(&get_env("AUTH0_AUDIENCE")?)?;
        let server = Url::parse(&get_env("AUTH0_SERVER")?)?;
        Ok(OauthConfig {
            TOKEN: server.join("oauth/token")?,
            USER_INFO: audience.join("userinfo")?,
            API: audience.join("api/v2/")?,

            AUDIENCE: audience,
            SERVER: server,

            CLIENT_ID: get_env("AUTH0_CLIENT_ID")?,
            CLIENT_SECRET: get_env("AUTH0_CLIENT_SECRET")?,
            REDIRECT_URI: get_env("AUTH0_REDIRECT_URI")?,
        })
    }
}

#[allow(non_snake_case)]
pub struct ServerConfig {
    pub PORT: u16,
}

impl ConfigLoader for ServerConfig {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let port: u16 = get_env("SERVER_PORT")?
            .parse()
            .map_err(|_| anyhow!("Can not parse port to u16"))?;
        Ok(ServerConfig { PORT: port })
    }
}

fn get_env(name: &'static str) -> anyhow::Result<String> {
    Ok(env::var(name)?)
}
