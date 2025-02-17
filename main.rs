
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct MultiplyParameters{
     n: u64,
     m: u64,
}

async fn post_multiply(form: web::Form<MultiplyParameters>) -> HttpResponse {
  if form.n == 0 || form.m == 0 {
   return HttpResponse::BadRequest()
      .content_type("text/html")
      .body("computing the multiplication with just 0 doesn't do much!");

  }

    let response=
        format!("Multiplication of {} and {} \
               is <b>{}</b>\n",
              form.n, form.m, multiply(form.n, form.m));

            
            
     HttpResponse::Ok()
      .content_type("text/html")
      .body(response)    
}


#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
      App::new()
        .route("/", web::get().to(get_index))
        .route("/multiply", web::post().to(post_multiply))
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
           <title>Multiply</title>
           <form action="/multiply" method="post">
           <input type="text" name="n"/>
           <input type="text" name="m"/>
           <button type="submit">Multiply numbers</button>
           </form>
           "#,
    )
}


//multiply 2 numbers
fn multiply(n: u64, m: u64) -> u128
{

    let p:u128 = (m * n).into();
    p
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(6, 1), 6);
}
