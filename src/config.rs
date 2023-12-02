use fedimint_client::derivable_secret::DerivableSecret;
use fedimint_client::secret::{PlainRootSecretStrategy, RootSecretStrategy};
use fedimint_core::api::InviteCode;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;

pub struct Config {
    pub invite_code: InviteCode,
    pub root_secret: DerivableSecret,
    pub db_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let db_path = env::var("DB_PATH").expect("DB_PATH must be set");
        let db_path = PathBuf::from_str(&db_path).expect("Invalid db path");

        let invite_code =
            env::var("FEDERATION_INVITE_CODE").expect("FEDERATION_INVITE_CODE must be set");
        let invite_code = InviteCode::from_str(&invite_code).expect("Invalid invite code");

        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        let root_secret = create_root_secret(secret);

        info!("Loaded config");

        Ok(Self {
            invite_code,
            root_secret,
            db_path,
        })
    }
}

fn create_root_secret(secret: String) -> DerivableSecret {
    let secret_bytes = secret.as_bytes();
    assert_eq!(secret_bytes.len(), 64, "SECRET_KEY must be 64 bytes long");
    let mut secret_array = [0; 64];
    secret_array.copy_from_slice(secret_bytes);
    PlainRootSecretStrategy::to_root_secret(&secret_array)
}
