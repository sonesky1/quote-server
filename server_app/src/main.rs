use actix_web::{web::Query, Error};
//adapting this over to using Axum from actix was achieved with help from DeepSeek. 
use axum::{
    extract::{Form, State}, response::Html, routing::{get, post}, Router
};
use serde::Deserialize;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement, DbBackend, DbErr};
use std::sync::Arc;
//use tokio::sync::Mutex;
use std::io;
use tokio::net::TcpListener;


#[derive(Deserialize)]
struct EnvironmentalActionParameters {
    action: String,
    topic: String,
}

#[derive(Deserialize)]
struct TheData {
    name: String,
}


/*Define a type for the shared state
#[derive(Clone)]
struct AppState {
    db: Arc<DatabaseConnection>,
}
*/

//adapted from DeepSeek.com AI
async fn retrieve_data(db: Arc<DatabaseConnection> ) -> Result<Vec<EnvironmentalActionParameters>, DbErr> {
  // prep query for SQL
  let sql = "SELECT action, topic FROM environmental_actions";
  let statement = Statement::from_string(DbBackend::Sqlite, sql.to_owned());
// Access the shared database state
  //let db = db.lock().await;
  let db = db;
  //execute query and map results to the EnvironmentalActionParameters
  let data_retrieved = db.query_all(statement)
  .await?
  .into_iter()
  .map(|row| {
        EnvironmentalActionParameters {
            action: row.try_get::<String>("", "action").unwrap(),
            topic: row.try_get::<String>("", "topic").unwrap(),

        }
})
.collect();

Ok(data_retrieved)

}

fn format_data_as_html(data_retrieved: Vec<EnvironmentalActionParameters>) -> String {
    let mut to_display   = String::from("<table border='1'><tr><th>Action</th><th>Topic</th></tr>");

    for data in data_retrieved {
        to_display.push_str(&format!(
            "<tr><td>{}</td><td>{}</td></tr>",
         data.action, data.topic
        ));
    }
    to_display.push_str(&format!("</table>"));
    to_display
}

async fn add_data_handler(
    State(state): State<Arc<DatabaseConnection>>,
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
    //let db = state.db.lock().await;
     let db = state.clone();
    // Example: Insert the action and topic into the database
    let add_to_database = format!(
        "INSERT INTO environmental_actions (action, topic) VALUES ('{}', '{}');",
        form.action, form.topic
    );

    db.execute(Statement::from_string(DbBackend::Sqlite, add_to_database))
        .await
        .expect("Failed to insert into environmental_actions table");

    let _response_0 = format!(
        "Good job. Your action of {} has been added to the environmental bucket under topic {}.",
        form.action,
        form.topic,
    );
    //let vector_from_database: Vec<EnvironmentalActionParameters> = retrieve_data(&db);
    //let _vector  = retrieve_data(&db).await;
    
    //let the_data_vector: Result<Vec<EnvironmentalActionParameters>, DbErr> = retrieve_data(&db).await;
     let response: Html<String> = get_data(axum::extract::State(state)).await;
    //Ok()
    //let response = format_data_as_html(the_data_vector);
   // //let data_response = get_data(db);
    //let response:String = "something".to_string();
    response
}
//my other one

async fn get_data(
    
    State(state): State<Arc<DatabaseConnection>>,
    //Form(query):Form<Query>,
) -> Html<String>{
    let db = state;
    let data = retrieve_data(db).await.expect("Failure to get data");
    let data_post = format_data_as_html(data);

    Html(data_post)
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

            <h2>Display Responses</h2>
            <form action="/get_data" method="post">
                <button type="submit">Display All Responses</button>
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
    let state = Arc::new(db);

    // Define the shared state
    //let state = db.clone();
     
    // Build the router with the shared state
    let app = Router::new()
        .route("/", get(get_index))
        .route("/add_to_database", post(add_data_handler))
        //.route("/query", post(get_data))
        .with_state(state.into());
       

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
