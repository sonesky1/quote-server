//adapting this over to using Axum from actix was achieved with help from DeepSeek. 

use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct EnvironmentalActionParameters {
    action: String,
    topic: String,
}

async fn post_response(Form(form): Form<EnvironmentalActionParameters>) -> Html<String> {
    if form.action.is_empty() && form.topic.is_empty() {
        return Html(
            "Please enter what you did as your Environmental Action in the first box. \
            Please type a topic you believe it could belong to in the second box. \n\
            Action: Rode bike to work    Topic: Transportation".to_string(),
        );
    }

    let response = format!(
        "Good job. Your action of {} has been added to the environmental bucket under topic {}. \n{}",
        form.action,
        form.topic,
        add_to_database(&form.action, &form.topic)
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
            <h1>DROP IN THE BUCKET</h1>
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

fn add_to_database(action: &str, topic: &str) -> String {
    "...\n  nothing more is implemented yet. Code writer needs to work on the database function!".to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_index))
        .route("/add_to_database", post(post_response));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serving on http://{}...", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
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
