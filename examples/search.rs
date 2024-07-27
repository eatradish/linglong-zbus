use linglong_zbus::{package_manager1::PackageManager1Proxy, search};
use zbus::Connection;

#[tokio::main]
async fn main() {
    let conn = Connection::system().await.unwrap();
    let proxy = PackageManager1Proxy::new(&conn).await.unwrap();
    let res = search(&proxy, "youdao").await.unwrap();
    dbg!(res);
}
