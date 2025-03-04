use axum::{
    routing::get,
    Router,
};

use adapter::handler::vocabulary;

// async fn sample() -> String {
//     "Hello, World!".to_string()
// }

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/sample", get(vocabulary::sample_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}