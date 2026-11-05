use std::net::TcpListener;
use std::io::Write;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:2525")
        .expect("Could not bind to address");

    println!("Mailforge listening on 127.0.0.1:2525");

    loop {
        let (mut stream, addr) = listener.accept().expect("Failed to accept connection");

        println!("Connection established from {addr}");

        stream.write_all("220 localhost ESMTP MailForge\r\n".as_bytes()).expect("Failed to send greetings!");
    }
}