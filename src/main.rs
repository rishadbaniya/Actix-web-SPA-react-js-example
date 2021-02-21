use actix_web::{web, App, get, HttpServer, Responder,HttpResponse,http::StatusCode};
use actix_files;
#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hey")
}
async fn react_index() -> actix_files::NamedFile{
    actix_files::NamedFile::open("./static_content/index.html").unwrap()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new().service(actix_files::Files::new("/","./static_content").index_file("index.html"))
        .default_service(
            web::resource("")
            .route(web::get().to(react_index))
        )
    }).bind("localhost:8080")?.run().await
}