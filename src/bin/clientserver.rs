use axum;
use axum::{Router, routing::get};
use sqlx::Pool;
use sqlx::postgres::Postgres;
use tokio::net;

#[derive(sqlx::FromRow, Debug)]
struct BookData {
    title: String,
}

async fn client_request_books() -> String {
    println!("Quick test");

    let pool = Pool::<Postgres>::connect("postgresql://luka@localhost:5432/MyBookStore")
        .await
        .unwrap();

    let status: BookData = sqlx::query_as("SELECT title FROM books")
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{}", status.title);
    status.title
}

fn main_client_router() -> Router {
    Router::new().route("/clientrequestbooks", get(client_request_books))
}

#[tokio::main]
async fn main() {
    let app = main_client_router();

    let listener = net::TcpListener::bind("0.0.0.0:3069").await.unwrap();

    let server_addy = listener.local_addr().unwrap();

    println!("{}", server_addy);

    axum::serve(listener, app).await.unwrap();
}
