use serde::{Deserialize, Serialize};
use zbus::zvariant::{OwnedValue, Type};

pub mod package_manager1;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct SearchResult {
    code: i64,
    message: String,
    packages: OwnedValue,
}
