use figlet_rs::FIGfont;
use local_ip_address::local_ip;
use qrcode::QrCode;
use qrcode::render::unicode;
use rand::{Rng, rng};
// use std::env;
use clap::Parser;
use colored::*;
use std::fs;
use std::path::Path;
use tiny_http::{Header, Response, Server, StatusCode};

#[derive(Parser)]
#[command(name = "wormhole")]
#[command(version = "1.0")]
#[command(about = "Share files over LAN with QR codes. No internet required")]
struct Cli {
    filename: Option<String>,
}
fn main() {
    let port: u32 = rng().random_range(1001..=6000);
    #[allow(unused_variables)]
    //Title banner
    let standard_font = FIGfont::standard().unwrap();
    let banner = standard_font.convert("WORM HOLE").unwrap();
    println!("{}", banner.to_string().blue());
    println!(
        "{}",
        "News: Upcoming release will support file transfer without a physical modem or router."
            .blue()
            .bold()
    );
    let args = Cli::parse();
    let ip = match local_ip() {
        Ok(ip) => ip,
        Err(e) => {
            println!("There is no Local Area network");
            eprintln!("{e}");
            //for next release
            "192.168.1.1".parse().unwrap()
        }
    };
    //format network url
    let network_ip = format!("http://{ip}:{port}");
    // show qrcode
    let code = QrCode::new(&network_ip).unwrap();
    let qr_image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Dark)
        .light_color(unicode::Dense1x2::Light)
        .build();
    let f_path = match &args.filename {
        Some(path) => path,
        None => {
            eprintln!(
                "{}{}",
                "Usage: ".red().bold(),
                "wormhole <filename>".green()
            );
            return;
        }
    };
    if !Path::new(f_path).is_file() {
        eprintln!("Error: {} is not a file.", f_path);
        return;
    }
    let f_name = Path::new(f_path)
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let server = Server::http(format!("0.0.0.0:{port}")).unwrap();
    println!("Scan this QR code using phone");
    println!("{qr_image}");

    //checking the formated link
    println!("URL Endpoint: {network_ip}");
    // serve file
    for request in server.incoming_requests() {
        println!(
            "Connected to: {:?}",
            request.remote_addr().expect("REASON").ip()
        );

        let file_content = match fs::read(f_path) {
            Ok(data) => data,
            Err(err) => {
                let err_response = Response::from_string(format!("Error reading file: {err}"))
                    .with_status_code(StatusCode(500));
                let _ = request.respond(err_response);
                continue;
            }
        };

        let response = Response::from_data(file_content)
            .with_status_code(200)
            .with_header(
                Header::from_bytes(&b"Content-Type"[..], &b"application/octet-stream"[..]).unwrap(),
            )
            .with_header(
                Header::from_bytes(
                    &b"Content-Disposition"[..],
                    format!("attachment; filename=\"{}\"", f_name).as_bytes(),
                )
                .unwrap(),
            );

        let _ = request.respond(response);
    }
}
