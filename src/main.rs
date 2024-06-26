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
                if (resource.ends_with("/index.html")) {
                    path.push(".");
                }
                print!("{:?}", &path.display());
                stream.write_all(&std::fs::read(path).unwrap()).unwrap();
            }
            _ => todo!(),
        }
    }
}
