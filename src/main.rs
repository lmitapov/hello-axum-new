use axum::{
    routing::get,
    Router,
    extract::Query,
    response::Html,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Name {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Query(params): Query<Name>) -> Html<String> {
    let html_content = format!("<h1>Hello, {}!</h1>", params.name);
    Html(html_content)
}
