use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;


#[derive(serde::Deserialize)]
struct EnvironmentalActionParameters {
    action: String,
    topic: String,
}

async fn post_response(form: web::Form<EnvironmentalActionParameters<>>) -> HttpResponse {
    if form.action =="".to_string() && form.topic =="".to_string() {
        return HttpResponse::BadRequest()
      .content_type("text/html")
      .body("Please enter what you did as your Environmental Action in the first box. Please type a topic you believe it could belong to in the second box. \n
            Action: Rode bike to work    Topic: Transportation");
    }

    let response = format!(
        "Good job. Your action of {} has been added to the environmental bucket under topic {}. /n{}",
        form.action, form.topic, add_to_database(&form.action,&form.topic));
    



    HttpResponse::Ok().content_type("text/html").body(response)
}



#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/add_to_database", web::post().to(post_response))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
           <title>Add your Action</title>
           <form action="/add_to_database" method="post">
           <input type="text" name="action"/>
           <input type="text" name="topic"/>
           <button type="submit">Add to Bucket</button>
           </form>
           "#,
    )
}



fn add_to_database<'a>(action: &'a str, topic: &'a str) -> String {
    
        return ".../n  nothing more is implemented yet. Code writer needs to work on the database function!".to_string()
    
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
