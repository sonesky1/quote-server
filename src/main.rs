//adapting this over to using Axum from actix was achieved with help from DeepSeek.
use axum::{
    extract::{Form, State},
    response::Html,
    routing::{get, post},
    Router,
};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};
use serde::Deserialize;
use std::io;
use std::sync::Arc;
use tokio::net::TcpListener;

use askama::Template;

#[derive(Template, Debug)]
#[template(path = "index.html")]
struct HomepageTemplate {
    quote: String,
    author: String,
    stylesheet: String,
}

#[derive(Deserialize)]
struct Quotes {
    quote: String,
    author: String,
}

async fn retrieve_data(db: Arc<DatabaseConnection>) -> Result<Vec<Quotes>, DbErr> {
    // prep query for SQL
    let sql = "SELECT * FROM quote_table ORDER BY RANDOM() LIMIT 1";
    let statement = Statement::from_string(DbBackend::Sqlite, sql.to_owned());

    let db = db;
    //execute query and map results to the quotes
    let rows = db.query_all(statement).await?;

    let quotes = rows
        .into_iter()
        .map(|row| Quotes {
            quote: row.try_get("", "quote").unwrap_or_default(),
            author: row.try_get("", "author").unwrap_or_default(),
        })
        .collect();

    Ok(quotes)
}

async fn add_data_handler(
    State(state): State<Arc<DatabaseConnection>>,
    Form(form): Form<Quotes>,
) -> Html<String> {
    if form.quote.is_empty() || form.author.is_empty() {
        let response: Html<String> = get_data(axum::extract::State(state)).await;

        return response;
    }

    // Access the shared database state

    let db = state.clone();
    // Example: Insert the quote and author  into the database
    let add_to_database = format!(
        "INSERT INTO quote_table (quote, author) VALUES ('{}', '{}');",
        form.quote, form.author
    );

    db.execute(Statement::from_string(DbBackend::Sqlite, add_to_database))
        .await
        .expect("Failed to insert into quote_table");

    //gets data from backend if successful and displays it on a redirect
    //as a table.
    let response: Html<String> = get_data(axum::extract::State(state)).await;

    response
}

async fn get_data(State(state): State<Arc<DatabaseConnection>>) -> Html<String> {
    let db = state;
    let quotes = retrieve_data(db).await.expect("Failure to get data");
    // Default hardcoded values
    let mut random_quote = "Slow but steady wins the race".to_string();
    let mut random_author = "Anonymous Turtle".to_string();
    if !quotes.is_empty() {
        let first_quote: &Quotes = &quotes[0];
        random_quote = first_quote.quote.clone();
        random_author = first_quote.author.clone();
    }

    let template = HomepageTemplate {
        quote: random_quote,
        author: random_author,
        stylesheet: "/assets/static/quote.css".to_string(),
    };
    axum::response::Html(template.render().expect("Failed to render template"))
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // You'll need to derive Debug for your struct:

    // SQLite database file
    let db_url = "sqlite://entries_database.db?mode=rwc";

    // Use connection pooling from SeaORM
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Failed to connect to the entries_database");

    // Create the environmental_actions table if it doesn't exist
    let create_quote_table_sql = r#"
        CREATE TABLE IF NOT EXISTS quote_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            quote TEXT NOT NULL,
            author TEXT NOT NULL
        );
    "#;

    //executes the SQL statement stmt that was created above
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        create_quote_table_sql.to_string(),
    ))
    .await
    .expect("Failed to create quote_table");

    println!("quote_table in entries_database is set up safely for async!");

    // Use Arc to share the database safely in async code
    let state = Arc::new(db);

    // Build the router with the shared state
    let app = Router::new()
        .route("/", get(get_data))
        .route("/add_to_database", post(add_data_handler))
        .with_state(state);

    // Serve the app
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serving on: http://127.0.0.1:8080");
    axum::serve(listener, app).await?;

    Ok(())
}
