use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer};

use std::collections::HashMap;

use crate::{
    error::AppError,
    types::nostr::{Nip05WellKnown, Nip05WellKnownParams},
};
use axum::http::StatusCode;

pub async fn get_pubkey_and_relays(
    nostr_json: &Nip05WellKnown,
    params: &Nip05WellKnownParams,
) -> Result<Nip05WellKnown, AppError> {
    let pubkey = nostr_json.names.get(&params.name).ok_or_else(|| AppError {
        error: anyhow::anyhow!("Username not found"),
        status: StatusCode::NOT_FOUND,
    })?;

    let relays = nostr_json.relays.get(pubkey).ok_or_else(|| AppError {
        error: anyhow::anyhow!("Relays not found"),
        status: StatusCode::NOT_FOUND,
    })?;

    let mut names = HashMap::new();
    names.insert(params.name.clone(), pubkey.clone());

    let mut relays_map = HashMap::new();
    relays_map.insert(pubkey.clone(), relays.clone());

    Ok(Nip05WellKnown {
        names,
        relays: relays_map,
    })
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
