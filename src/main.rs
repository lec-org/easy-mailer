mod client;
mod config;
mod email;
mod routes;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use config::Config;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "client" {
        if let Err(e) = client::send_email_request().await {
            eprintln!("发送邮件请求时出错: {}", e);
        }
    } else {
        let config = Config::new().expect("Failed to load configuration");

        println!("正在启动， 127.0.0.1:8080");
        HttpServer::new(move || {
            let cors = Cors::permissive();
            App::new()
                .wrap(cors)
                .app_data(actix_web::web::Data::new(config.clone()))
                .configure(routes::configure)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    }

    Ok(())
}
