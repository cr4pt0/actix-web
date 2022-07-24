
#[allow(unused_imports)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use placex_web_service::route::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(echo)
        .route("/hey", web::get().to(manual_hello))
        .service(web::scope("/app").route("/x", web::get().to(manual_hello) ))
        
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
