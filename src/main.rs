use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:50000").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr = std::io::BufReader::new(&mut stream);
        let mut l = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() {
                        break;
                    }
                }
                let mut path = std::path::PathBuf::new();
                path.push(resource.trim_start_matches('/'));
                if (resource.ends_with("/")) {
                    path.push("index.html");
                }
                print!("{:?}", &path);
                let content = std::fs::read(&path)
                    .unwrap_or_else(|_| b"HTTP/1.1 404 NOT FOUND\r\n\r\n".to_vec());
                if content.starts_with(b"HTTP/1.1 404") {
                    stream.write_all(&content).unwrap();
                } else {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    stream.write_all(&content).unwrap();
                }
            }
            _ => todo!(),
        }
    }
}
