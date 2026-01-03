use sqlx::Pool;
use sqlx::postgres::Postgres;

#[tokio::main]
async fn main() {
    let pool = Pool::<Postgres>::connect("postgresql://luka@localhost:5432/MyBookStore")
        .await
        .unwrap();

    // sqlx::query("INSERT INTO books (title, author, isbn, price, stock_quantity) VALUES ('The Rust Book', 'Luka Engine', '9781718503106', 39.99, 15)")
    // .execute(&pool)
    // .await
    // .unwrap();
    //
    let status: BookData = sqlx::query_as("SELECT title FROM books")
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("{:?}", status.title);
}

#[derive(sqlx::FromRow, Debug)]
struct BookData {
    title: String,
}
