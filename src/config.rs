use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use fedimint_client::derivable_secret::DerivableSecret;
use fedimint_client::secret::{PlainRootSecretStrategy, RootSecretStrategy};
use fedimint_core::api::InviteCode;
use nostr::hashes::hex::FromHex;
use nostr::key::FromSkStr;
use nostr::Keys;
use tracing::info;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config =
        Config::from_env().expect("Failed to load config from environment");
}

pub struct Config {
    pub domain: String,
    pub port: u16,
    pub invite_code: InviteCode,
    pub root_secret: DerivableSecret,
    pub fm_db_path: PathBuf,
    pub pg_db: String,
    pub nostr_sk: Keys,
    pub default_relay: String,
    pub xmpp_username: String,
    pub xmpp_password: String,
    pub xmpp_chat_server: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let domain = env::var("DOMAIN").unwrap_or("localhost".to_string());

        let port = env::var("PORT").unwrap_or("3000".to_string());
        let port = u16::from_str(&port).expect("Invalid port");

        let fm_db_path = env::var("FM_DB_PATH").expect("FM_DB_PATH must be set");
        let fm_db_path = PathBuf::from_str(&fm_db_path).expect("Invalid fm db path");

        let invite_code =
            env::var("FEDERATION_INVITE_CODE").expect("FEDERATION_INVITE_CODE must be set");
        let invite_code = InviteCode::from_str(&invite_code).expect("Invalid invite code");

        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let root_secret = create_root_secret(secret);

        let pg_db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let nostr_sk = env::var("NOSTR_SK").expect("NOSTR_SK must be set");
        let nostr_sk = Keys::from_sk_str(&nostr_sk).expect("Invalid NOSTR_SK");

        let default_relay =
            env::var("DEFAULT_NOSTR_RELAY").expect("DEFAULT_NOSTR_RELAY must be set");

        let xmpp_username = env::var("XMPP_USERNAME").expect("XMPP_USERNAME must be set");
        let xmpp_password = env::var("XMPP_PASSWORD").expect("XMPP_PASSWORD must be set");
        let xmpp_chat_server = env::var("XMPP_CHAT_SERVER").expect("XMPP_CHAT_SERVER must be set");

        info!("Loaded config");

        Ok(Self {
            domain,
            port,
            invite_code,
            root_secret,
            fm_db_path,
            pg_db,
            nostr_sk,
            default_relay,
            xmpp_username,
            xmpp_password,
            xmpp_chat_server,
        })
    }
}

fn create_root_secret(secret: String) -> DerivableSecret {
    let secret_bytes: [u8; 64] = FromHex::from_hex(&secret).expect("Invalid hex string");
    PlainRootSecretStrategy::to_root_secret(&secret_bytes)
}
