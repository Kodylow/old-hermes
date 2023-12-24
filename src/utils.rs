use std::{fmt::Display, fs::read_to_string, str::FromStr};

use serde::{de, Deserialize, Deserializer};

use crate::models::nostr::Nip05WellKnown;

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

pub fn get_nostr_json() -> Nip05WellKnown {
    let nostr_str = read_to_string("nostr.json").expect("Could not read nostr.json");
    serde_json::from_str::<Nip05WellKnown>(&nostr_str).expect("Invalid nostr.json")
}
