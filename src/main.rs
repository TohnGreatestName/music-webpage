use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "test.html")]
struct MyTemplate {
    users: Vec<String>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        get(|| async {
            Html(
                MyTemplate {
                    users: vec!["abc".to_string(), "bca".to_string(), "cde".to_string()],
                }
                .render()
                .unwrap(),
            )
            .into_response()
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Listening on http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}
