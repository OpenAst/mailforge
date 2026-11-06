
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
            println!("Buffer: {:?}", command_buffer);

            while let Some(position) = command_buffer
                .windows(2)
                .position(|window| window == b"\r\n") {
                    let command = String::from_utf8_lossy(&command_buffer[..position])
                    .into_owned();

                    if command.starts_with("EHLO") {
                        stream.write_all(
                            b"250 localhost\r\n"
                        )
                        .expect("Failed to send EHLO response");
                    } else if command.starts_with("MAIL FROM:") {
                        stream.write_all(
                            "250 OK\r\n".as_bytes()
                        )
                        .expect("Failed to send MAIL FROM response");
                    } else if command.starts_with("RCPT TO:") {
                        stream.write_all(
                            "250 OK\r\n".as_bytes()
                        )
                        .expect("Failed to send RCPT TO response");
                    } else if command == "DATA" {
                        stream.write_all(
                            b"354 End data with <CR><LF>.<CR><LF>\r\n"
                        )
                        .expect("Failed to send DATA response");
                    } else if command == "QUIT" {
                        stream.write_all(
                            b"221 Bye\r\n"
                        )
                        .expect("Failed to send QUIT response");
                        break;
                    } else {
                        stream.write_all(
                            b"500 Unrecognized command\r\n"
                        )
                        .expect("Failed to send unrecognized command response");
                    }
                    println!("Received command: {command}");

                    command_buffer.drain(..position + 2);
                }
        }
    }
}
