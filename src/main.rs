//adapting this over to using Axum from actix was achieved with help from DeepSeek.
mod quote;
mod templates;
use crate::templates::IndexTemplate;
extern crate mime;
use crate::quote::THE_QUOTE;
use crate::quote::Quote;
use axum::{self, response, routing};
use tokio::net;
use tower_http::services;


async fn get_quote() -> response::Html<String> {
    let quote = IndexTemplate::quote(&THE_QUOTE);
    response::Html(quote.to_string())
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let app = axum::Router::new()
        .route("/", routing::get(get_quote))
        .route_service(
            "/quote.css",
            services::ServeFile::new_with_mime("assets/static/quote.css", &mime::TEXT_CSS),
        );
    let listener = net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = serve().await {
        eprintln!("quote-server-askama: error: {}", err);
        std::process::exit(1);
    }
}
/*
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

#[derive(Deserialize)]
struct Quotes{
    quote: String,
    author: String,
}


// in the preview, the `source="…"` or `path="…"` argument is provided for you

struct HelloWorld<'a> {
    user: &'a str,
    first_visit: bool,
}
struct Quotes<'a>{
    quote: &'a str,
    author: &'a str,
}

fn main() {
    let quotes = Quotes { quote: "Carpe Diem" , author: "teacher"}; // instantiate your struct
    println!("{}", quotes.render().unwrap()); // then render it.
}

async fn retrieve_data(
    db: Arc<DatabaseConnection>,
) -> Result<Vec<Quotes>, DbErr> {
    // prep query for SQL
    let sql = "SELECT quote, author FROM quotes_table";
    let statement = Statement::from_string(DbBackend::Sqlite, sql.to_owned());

    let db = db;
    //execute query and map results to the quotes
    let data_retrieved = db
        .query_all(statement)
        .await?
        .into_iter()
        .map(|row| Quotes {
            quote: row.try_get::<String>("", "quote").unwrap(),
            author: row.try_get::<String>("", "author").unwrap(),
        })
        .collect();

    Ok(data_retrieved)
}

//receives input from the database and pushes it into a table format in html
/* 
fn format_data_as_html(data_retrieved: Vec<Quotes>) -> String {
    let mut to_display = String::from("<table border='1'><tr><th>Quote</th><th>Author</th></tr>");

    for data in data_retrieved {
        to_display.push_str(&format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            data.quote, data.author
        ));
    }
    
    to_display.push_str("</table>");

    to_display
}
*/

async fn add_data_handler(
    State(state): State<Arc<DatabaseConnection>>,
    Form(form): Form<Quotes>,
) -> Html<String> {
    if form.quote.is_empty() || form.author.is_empty() {
        //if either form is empty then an error message is returned
        return Html(
            "Please enter a quote in the first box. \
            Please type an author in the second box"
                .to_string(),
        );
    }

    // Access the shared database state

    let db = state.clone();
    // Example: Insert the quote and author  into the database
    let add_to_database = format!(
        "INSERT INTO quotes_table (quote, author) VALUES ('{}', '{}');",
        form.quote, form.author
    );

    db.execute(Statement::from_string(DbBackend::Sqlite, add_to_database))
        .await
        .expect("Failed to insert into quotes_table");

    //gets data from backend if successful and displays it on a redirect
    //as a table.
    let response: Html<String> = get_data(axum::extract::State(state)).await;

    response
}

//retrieves data from database
async fn get_data(State(state): State<Arc<DatabaseConnection>>) -> Html<String> {
    let db = state;
    let data = retrieve_data(db).await.expect("Failure to get data");
   // let data_post = format_data_as_html(data);
    let data_post = QuoteStruct { quote: "Carpe Diem" , author: "teacher"}; // instantiate your struct
   


    Html(data_post.render().unwrap());
}

async fn get_index() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Quote Server</title>
        </head>
        <body>
            <h1>QUOTE SERVER: <br \> Please enter a quote or word of wisdom in first box. Please enter an author in the second box <br \><br \>
            EXAMPLE: <br \>
            Quote: El barato cuesta caro (What is cheap is expensive) <br \>
            Author: Unknown, Spanish dicho <br/> 
            </h1>
            <form action="/add_to_database" method="post">
                <label for="entry">Quote:</label><br>
                <input type="text" id="quote" name="quote"/><br><br>
                <label for="entry">Author:</label><br>
                <input type="text" id="author" name="author"/><br><br>
                <button type="submit">Add to Quote Database</button>
            </form>            
        </body>
        </html>
        "#,
    )
}
 
#[tokio::main]
async fn main() -> io::Result<()> {
    // SQLite database file
    let db_url = "sqlite://entries_database.db?mode=rwc";

    // Use connection pooling from SeaORM
    let db: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Failed to connect to the entries_database");

    // Create the quotes table if it doesn't exist
    let create_quotes_table_sql = r#"
        CREATE TABLE IF NOT EXISTS quotes_table (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            quote TEXT NOT NULL,
            author TEXT NOT NULL
        );
    "#;

    //executes the SQL statement stmt that was created above
    db.execute(Statement::from_string(
        DbBackend::Sqlite,
        create_quotes_table_sql.to_string(),
    ))
    .await
    .expect("Failed to create quotes_table");

    println!("quotes_table table in entries_database is set up safely for async!");

    // Use Arc to share the database safely in async code
    let state = Arc::new(db);

    // Build the router with the shared state
    let app = Router::new()
        .route("/", get(get_index))
        .route("/add_to_database", post(add_data_handler))
        .with_state(state);
        //.with_state(state.into());

    // Serve the app
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serving on: http://127.0.0.1:8080");
    axum::serve(listener, app).await?;

    Ok(())
}
*/    
