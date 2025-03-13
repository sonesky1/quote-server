//adapting this over to using Axum from actix was achieved with help from DeepSeek. 
use axum::{
    extract::{Form, State},
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement, DbBackend};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct EnvironmentalActionParameters {
    action: String,
    topic: String,
}

// Define a type for the shared state
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<DatabaseConnection>>,
}

async fn post_response(
    State(state): State<AppState>,
    Form(form): Form<EnvironmentalActionParameters>,
) -> Html<String> {
    if form.action.is_empty() && form.topic.is_empty() {
        return Html(
            "Please enter what you did as your Environmental Action in the first box. \
            Please type a topic you believe it could belong to in the second box. \n\
            Action: Rode bike to work    Topic: Transportation".to_string(),
        );
    }

    // Access the shared database state
    let db = state.db.lock().await;

    // Example: Insert the action and topic into the database
    let insert_sql = format!(
        "INSERT INTO environmental_actions (action, topic) VALUES ('{}', '{}');",
        form.action, form.topic
    );

    db.execute(Statement::from_string(DbBackend::Sqlite, insert_sql))
        .await
        .expect("Failed to insert into environmental_actions table");

    let response = format!(
        "Good job. Your action of {} has been added to the environmental bucket under topic {}.",
        form.action,
        form.topic,
    );

    Html(response)
}

async fn get_index() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Add your Action</title>
        </head>
        <body>
            <h1>DROP IN THE BUCKET:  Please enter any action you took today to help 
            the environment in the first box.
            Please enter a potential category you believe that action could belong
            to in the second box.
            Here are some examples: 
            Action: Rode bike to work                  Topic: Transportation </h1>
            <form action="/add_to_database" method="post">
                <label for="action">Action you did to help the environment:</label><br>
                <input type="text" id="action" name="action"/><br><br>
                
                <label for="topic">Action Category:</label><br>
                <input type="text" id="topic" name="topic"/><br><br>
                
                <button type="submit">Add to Bucket</button>
            </form>
            <br>
            <form action="/display_responses" method="get">
                <button type="submit">Display Responses</button>
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

    // Create the environmental_actions table if it doesn't exist
    let create_environmental_actions_table_sql = r#"
        CREATE TABLE IF NOT EXISTS environmental_actions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            action TEXT NOT NULL,
            topic TEXT NOT NULL
        );
    "#;

    db.execute(Statement::from_string(DbBackend::Sqlite, create_environmental_actions_table_sql.to_string()))
        .await
        .expect("Failed to create environmental_actions table");

    println!("environmental_actions table in entries_database is set up safely for async!");

    // Use Arc + Mutex to share the database safely in async code
    let shared_db = Arc::new(Mutex::new(db));

    // Define the shared state
    let state = AppState { db: shared_db };

    // Build the router with the shared state
    let app = Router::new()
        .route("/", get(get_index))
        .route("/add_to_database", post(post_response))
        .with_state(state);

    // Serve the app
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Serving on: http://127.0.0.1:8080");
    axum::serve(listener, app).await?;

    Ok(())
}
    //let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //println!("Serving on http://{}...", addr);

    /*axum::serve(&app)
        .serve(app.into_make_service())
        .await
        .unwrap();
    */

/*
fn _separate_str_by_word{


}

fn _remove_punctuation{



}

fn _remove_common_non_verbs{


}

fn _create_dictionary_of_common_not_helfpul_words{
//a, the, we, you, them, (this will help in only storing actions by other probably more "important" words for keeping track of action points)

}

fn _ability_for_someone_to_comment{
  //dream for later

}

fn _show_random_entries(u8){
//program will pick random entries from list

}

*/
