use actix_files as actix_fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use audiocloud_lib::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[get("/gen_lib")]
async fn gen_lib(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("Created library")
}
#[get("/")]
async fn ping_pong() -> impl Responder {
    println!("Req /");
    HttpResponse::Ok().body("Server is running")
}

#[get("/packs")]
async fn get_packs(data: web::Data<AppState>) -> impl Responder {
    let res = get_packs_metadata(&data.lib);
    HttpResponse::Ok().json(res)
}

#[post("/search")]
async fn search(data: web::Data<AppState>, query: web::Json<SearchParams>) -> impl Responder {
    let params = query.into_inner();
    let res = search_lib(&data.lib, &params);
    HttpResponse::Ok().json(res)
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerSettings {
    lib_path: String,
    lib_content_dir: String,
}
impl ServerSettings {
    fn load_from_file(path: &str) -> Self {
        let filecontent = fs::read_to_string(path).expect("Couldn't read file");
        let settings: Self = serde_json::from_str(&filecontent).expect("Couldnt parse file");
        settings
    }
    fn save_to_file(&self, path: &str) {
        let content = serde_json::to_string_pretty(self).expect("Couldnt serialize");
        let _ = fs::write(path, content);
    }
}

struct AppState {
    lib: SampleLibrary,
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = ServerSettings::load_from_file("config.json");
    /*let pack = load_pack(
        "Testlib/Battery Selection",
        "Battery4 Selection",
        "Example library using a few Battery4 drums",
    );
    let pack2 = load_pack(
        "Testlib/Platinum Loops",
        "Platinum Loops",
        "Example using loops",
    );
    let testlib_minimal = SampleLibrary {
        name: "Testlib_minimal.json".to_string(),
        packs: vec![pack, pack2],
    };
    save_lib_json(&testlib_minimal, "");
    exit(2);*/
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                lib: load_lib_json(&settings.lib_path),
            }))
            .service(ping_pong)
            .service(search)
            .service(gen_lib)
            .service(get_packs)
            .service(
                actix_fs::Files::new(
                    &("samples/".to_string() + &settings.lib_content_dir),
                    &settings.lib_content_dir,
                )
                .show_files_listing(),
            )
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
