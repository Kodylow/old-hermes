use std::fs::read_to_string;

use tracing::info;

pub async fn handle_readme() -> String {
    let readme = read_to_string("README.md").expect("Could not read README.md");
    info!("Serving README.md");
    readme
}

pub async fn register() -> String {
    "register stub".to_string()
}

pub async fn well_known(username: String) -> String {
    format!("well_known stub for {}", username)
}

pub async fn lnurlp_callback(username: String) -> String {
    format!("lnurlp_callback stub for {}", username)
}

pub async fn lnurlp_verify(username: String) -> String {
    format!("lnurlp_verify stub for {}", username)
}
