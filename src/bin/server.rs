use std::{
    error::Error,
    io::{BufWriter, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    thread,
};

fn main() -> Result<(), Box<dyn Error>> {
    let target_addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 30900);
    let listener = TcpListener::bind(target_addr)?;
    println!("Now listening on  {}", target_addr);

    loop {
        let (connection, source_addr) = listener.accept()?;
        thread::spawn(move || {
            let write_array = {
                let mut r = [0u8; 1024 * 32];
                let mut i = 0;
                for el in r.iter_mut() {
                    *el = i;
                    i = i.wrapping_add(1);
                }
                r
            };

            println!("Received Connection from {}", source_addr);
            let mut connection = BufWriter::with_capacity(1024 * 1024 * 8, connection);

            loop {
                if let Err(e) = connection.write_all(&write_array) {
                    match e.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            println!("Connection reached end.");
                            return;
                        }
                        _ => {
                            eprintln!("Received Unknown Error: {}", e);
                            return;
                        }
                    };
                };
            }
        });
    }
}
