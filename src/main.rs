use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use tera::Tera;

use std::env;
use std::io::Result;

mod handlers;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load port from Render (or default for local dev)
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", bind_addr);

    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(tera.clone()))
            .service(handlers::index)
            // Serve static files
            .service(Files::new("/static", "static"))
    })
    .bind(bind_addr)?
    .run()
    .await
}
