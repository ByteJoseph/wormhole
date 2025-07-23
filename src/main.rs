use axum::{Router, routing::get};
use local_ip_address::local_ip;
use qrcode::QrCode;
use qrcode::render::unicode;
use std::env;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return;
    }
    //get ip on LAN
    let ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };
    //format network url
    let network_ip = format!("http://{ip}:3000");
    // show qrcode
    let code = QrCode::new(&network_ip).unwrap();
    let qr_image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();
    println!("{qr_image}");

    //checking the formated link
    println!("URL Endpoint: {network_ip}");
    let app = Router::new().route("/", get(|| async { "Hello Test" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
