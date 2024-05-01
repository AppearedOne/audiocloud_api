use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use audiocloud_lib::*;
use serde::Deserialize;

#[get("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
#[get("/gen_lib")]
async fn gen_lib(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Created library")
}
#[get("/")]
async fn ping_pong() -> impl Responder {
    println!("Req /");
    HttpResponse::Ok().body("Server is running")
}
#[post("/search")]
async fn search(data: web::Data<AppState>, query: web::Json<SearchParams>) -> impl Responder {
    let params = query.into_inner();
    let res = search_lib(&data.lib, &params);
    HttpResponse::Ok().json(res)
}

struct AppState {
    lib: SampleLibrary,
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                lib: load_lib_json("Testlib.json"),
            }))
            .service(ping_pong)
            .service(echo)
            .service(search)
            .service(gen_lib)
    })
    .bind(("127.0.0.1", 4040))?
    .run()
    .await
}
