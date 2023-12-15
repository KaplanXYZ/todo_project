use std::fs::OpenOptions;

use axum::Router;

mod endpoints;
#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let _ = OpenOptions::new().create(true).open("todos.json");

    // build our application with a route
    let app = Router::new().merge(endpoints::todos::router());
        // `GET /` goes to `root`
   

    // `POST /users` goes to `create_user`

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

