use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| 
        {
            App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd",web::post().to(post_gcd))
        }
    );

    println!("Serving on http://localhost:3000");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .expect("error running the server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
              <title>GCD calculator</title>
              <form action="/gcd" method="post">
              <input type="text" name="n"/>
              <input type="text" name="m"/>
              <button type="submit"> Compute GCD</button>
              </form>
            "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing the GCD with zero is boring. ");
    }

    let response = format!(
        "The numbers were {}, {} and the GCD is {}",
        form.m,
        form.n,
        gcd(form.m, form.n)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

fn gcd(m: u64, n: u64)-> u64{
    m*n
}
