use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;
//use rbatis;
lazy_static! {
  // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，无需担心线程竞争
  static ref RB:Rbatis=Rbatis::new();
}
//这里使用async_std的main方法，你可以选择actix,tokio等等其他runtime运行时的main方法或者 spawn

async fn link_db() {
    //启用日志输出，你也可以使用其他日志框架，这个不限定的

    //初始化连接池
    RB.link("mysql://root:123456@localhost:3306/test")
        .await
        .unwrap();
}
