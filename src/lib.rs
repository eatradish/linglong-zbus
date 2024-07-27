use std::collections::HashMap;

use package_manager1::PackageManager1Proxy;
use serde::{Deserialize, Serialize};
use zbus::zvariant::{Array, Dict, OwnedValue, Type, Value};

pub mod package_manager1;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
#[zvariant(signature = "(xsa(ssss))")]
pub struct SearchResult {
    code: i64,
    message: String,
    packages: Vec<Package>,
}

impl TryFrom<HashMap<String, OwnedValue>> for SearchResult {
    type Error = zbus::Error;

    fn try_from(value: HashMap<String, OwnedValue>) -> Result<Self, Self::Error> {
        let code = value.get("code").ok_or_else(|| zbus::Error::InvalidField)?;
        let code = i64::try_from(code)?;
        let message = value
            .get("message")
            .ok_or_else(|| zbus::Error::InvalidField)?;

        let message = <&str>::try_from(message)?.to_string();
        let packages = value
            .get("packages")
            .ok_or_else(|| zbus::Error::InvalidField)?;

        let packages = Array::try_from(packages.try_clone()?)?;
        let mut pkgs = vec![];
        for p in packages.into_iter() {
            let p = p.downcast_ref::<OwnedValue>()?;
            let p = Package::try_from(p)?;
            pkgs.push(p);
        }
        Ok(Self {
            code,
            message,
            packages: pkgs,
        })
    }
}

impl TryFrom<OwnedValue> for Package {
    type Error = zbus::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        let dict = Dict::try_from(value)?;
        let id = dict
            .get::<Value, String>(&Value::Str("id".into()))?
            .ok_or_else(|| zbus::Error::InvalidField)?;
        let name = dict
            .get::<Value, String>(&Value::Str("name".into()))?
            .ok_or_else(|| zbus::Error::InvalidField)?;
        let description = dict
            .get::<Value, String>(&Value::Str("description".into()))?
            .ok_or_else(|| zbus::Error::InvalidField)?;
        let version = dict
            .get::<Value, String>(&Value::Str("version".into()))?
            .ok_or_else(|| zbus::Error::InvalidField)?;
        let arch_key = Value::Str("arch".into());
        let archs = dict
            .get::<Value, Array>(&arch_key)?
            .ok_or_else(|| zbus::Error::InvalidField)?;
        let mut archs_value = vec![];
        for a in archs.into_iter() {
            let a = a.downcast_ref::<OwnedValue>()?;
            let a = String::try_from(a)?;
            archs_value.push(a);
        }

        Ok(Self {
            id,
            name,
            description,
            version,
            archs: archs_value,
        })
    }
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct Package {
    id: String,
    name: String,
    description: String,
    version: String,
    archs: Vec<String>,
}

pub struct LingLongClient;

pub async fn search(
    proxy: &PackageManager1Proxy<'_>,
    query: &str,
) -> Result<SearchResult, zbus::Error> {
    let mut map = HashMap::new();
    let binding = zbus::zvariant::Value::Str(query.into());
    map.insert("id", &binding);

    let res = proxy.search(map).await?;
    let res = SearchResult::try_from(res)?;

    Ok(res)
}
