use actix_web::{App as ActixApp, HttpServer, HttpResponse, web};
use std::error::Error;

pub async fn start_gui_server(port: u16) -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = format!("0.0.0.0:{}", port);
    println!("Starting GUI server at http://{}", addr);

    HttpServer::new(|| {
        ActixApp::new().route("/", web::get().to(gui_home))
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}

async fn gui_home() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Flusso GUI")
}
