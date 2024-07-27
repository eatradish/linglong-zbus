use std::collections::HashMap;

use linglong_zbus::package_manager1::PackageManager1Proxy;
use zbus::{
    zvariant::{Array, OwnedValue, Value},
    Connection,
};
use zvariant::Dict;

#[tokio::main]
async fn main() {
    let conn = Connection::system().await.unwrap();
    let proxy = PackageManager1Proxy::new(&conn).await.unwrap();
    let mut map = HashMap::new();
    let binding = zbus::zvariant::Value::Str("youdao".into());
    map.insert("id", &binding);

    let res = proxy.search(map).await.unwrap();
    let code = res.get("code").unwrap();
    let v = i64::try_from(code).unwrap();
    println!("code: {}", v);

    let message = res.get("message").unwrap();
    let v = <&str>::try_from(message).unwrap();
    println!("message: {}", v);

    let packages = res.get("packages").unwrap();
    let packages = <Array>::try_from(packages.try_clone().unwrap()).unwrap();
    println!("packages:");
    for i in packages.into_iter() {
        let i = i.downcast_ref::<OwnedValue>().unwrap();
        let dict = Dict::try_from(i).unwrap();
        let id = dict
            .get::<Value, String>(&Value::Str("id".into()))
            .unwrap()
            .unwrap();
        let name = dict
            .get::<Value, String>(&Value::Str("name".into()))
            .unwrap()
            .unwrap();
        let desc = dict
            .get::<Value, String>(&Value::Str("description".into()))
            .unwrap()
            .unwrap();
        let version = dict
            .get::<Value, String>(&Value::Str("version".into()))
            .unwrap()
            .unwrap();
        let arch_key = Value::Str("arch".into());
        let archs = dict.get::<Value, Array>(&arch_key).unwrap().unwrap();
        let mut archs_value: Vec<String> = vec![];
        for a in archs.into_iter() {
            archs_value.push(String::try_from(a.downcast_ref::<String>().unwrap()).unwrap());
        }
        let archs_value = archs_value.join(", ");
        println!(
            "  {}\n    {}\n      {}: {}\n        {}",
            name, id, version, desc, archs_value
        );
    }
}
