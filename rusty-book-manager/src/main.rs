use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

// こういった関数をaxumでは"ハンドラ"という
// 関数の引数でリクエストを受け取り、関数内で処理を実行、レスポンスを返す、という作業を行う
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_world));
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // 上記で指定したアドレスでバインドしたリスナーを立ち上げる
    let listener = TcpListener::bind(addr).await.unwrap();
    // 起動する際にルーターをaxumのサービスに登録
    axum::serve(listener, app).await.unwrap();
}
