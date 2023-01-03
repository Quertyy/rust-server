use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();

        gestion_connexion(flux);
    }
}

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0;1024];

    flux.read(&mut tampon).unwrap();

    let contenu = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 {}\r\n\r\n{}",
        contenu.len(),
        contenu
    );

    flux.write(response.as_bytes()).unwrap();
    flux.flush().unwrap();
}