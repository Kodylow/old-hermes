use std::path::PathBuf;
use std::str::FromStr;

use fedimint_client::derivable_secret::DerivableSecret;
use fedimint_client::secret::{PlainRootSecretStrategy, RootSecretStrategy};
use fedimint_core::api::InviteCode;
use tracing::info;

pub struct Config {
    pub invite_code: InviteCode,
    pub root_secret: DerivableSecret,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();
        info!("Loaded environment variables");

        let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        info!("Loaded SECRET_KEY");

        let root_secret = create_root_secret(secret);
        info!("Created root secret");

        Ok(Self {
            invite_code,
            root_secret,
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
