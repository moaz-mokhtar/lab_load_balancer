use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Welcome to the Backend Server");
    initiate_logging();

    let server_address =
        std::env::var("BE_SERVER_HOST").expect("Missed 'BE_SERVER_HOST' environment variable");
    info!("Starting server at {}", server_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(web::scope("").service(web::resource("/").route(web::get().to(health))))
    })
    .bind(server_address)?
    .run()
    .await
}

pub fn initiate_logging() {
    std::env::set_var("RUST_LOG", "debug, actix_web=debug");

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    env_logger::init();
}

async fn health(req: HttpRequest) -> impl Responder {
    let request_address_message = format!(
        "Received request from {}",
        req.connection_info().peer_addr().unwrap_or_default()
    );
    let request_method_message =
        format!("{} {} {:?}", req.method(), req.uri().path(), req.version());
    let request_host_message = format!("Host: {:?}", req.headers().get("Host").unwrap());
    let request_user_agent_message =
        format!("User-Agent: {:?}", req.headers().get("User-Agent").unwrap());
    let request_accept_message = format!("Accept: {:?}", req.headers().get("Accept").unwrap());

    let message: String = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        request_address_message,
        request_method_message,
        request_host_message,
        request_user_agent_message,
        request_accept_message
    );
    println!("{}", message);

    // let status: String = "Ok".to_string();
    // let response = MessageResponse { status, message };
    HttpResponse::Ok().body(message)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageResponse {
    pub status: String,
    pub message: String,
}
