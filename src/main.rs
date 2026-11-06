
use std::net::TcpListener;
use std::io::{Write, Read};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2525")
        .expect("Could not bind to address");

    println!("Mailforge listening on 127.0.0.1:2525");

    loop {
        let (mut stream, addr) = listener.accept()
            .expect("Failed to accept connection");

        println!("Connection established from {addr}");

        stream.write_all(
            b"220 localhost ESMTP MailForge\r\n"
        ).expect("Failed to send greeting!");

        let mut read_buffer = [0u8; 1024];
        let mut command_buffer = Vec::new();

        loop {
            let bytes_read = stream.read(&mut read_buffer)
                .expect("Failed to read from stream");

            println!("Bytes received: {bytes_read}");
            
            if bytes_read == 0 {
                println!("Connection closed by client");
                break;
            }

            command_buffer.extend_from_slice(
                &read_buffer[..bytes_read]
            );

            while let Some(position) = command_buffer
                .windows(2)
                .position(|window| window == b"\r\n") {
                    let command = String::from_utf8_lossy(&command_buffer[..position])
                    .into_owned();

                    println!("Received command: {command}");

                    command_buffer.drain(..position + 2);
                }
        }
    }
}
