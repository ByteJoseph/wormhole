use axum::{
    routing::get,
    Router
};
use local_ip_address::local_ip;
use qrcode::QrCode;
use qrcode::render::unicode;
#[tokio::main]
async fn main() {

    //assign network ip address to variable name ip
    let ip = match local_ip(){
        Ok(ip) => { 
            ip 
        },
        Err(e) =>{ 
            eprintln!("{e}");
            return;
        }
    };
    //format network url
    let network_ip = format!("http://{ip}:3000");
    //checking the formated link
    println!("{network_ip}");
    let code = QrCode::new(network_ip).unwrap();
    let qr_image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();
    print!("{qr_image}");
    let app = Router::new().route("/",get(|| async {
        "Hello axum"
    }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

