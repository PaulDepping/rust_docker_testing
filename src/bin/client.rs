use std::{
    error::Error,
    io::{BufReader, Read},
    net::TcpStream,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let target_address = match std::env::var("CLIENT_CONNECT_URL") {
        Ok(s) => Ok(s),
        Err(err) => match err {
            std::env::VarError::NotPresent => Ok(String::from("localhost:30900")),
            err => Err(err),
        },
    }?;

    let connection = TcpStream::connect(&target_address)?;
    println!("Established Connection with {}", target_address);
    let mut connection = BufReader::with_capacity(1024 * 1024 * 64, connection);
    let mut total_throughput: usize = 0;

    let compare_array = {
        let mut r = [0u8; 1024 * 1024];
        let mut i = 0;
        for el in r.iter_mut() {
            *el = i;
            i = i.wrapping_add(1);
        }
        r
    };
    let mut read_array = [0; 1024 * 1024];

    let begin_time = Instant::now();

    loop {
        connection.read_exact(&mut read_array)?;
        assert_eq!(compare_array, read_array);
        total_throughput += compare_array.len();
        if total_throughput % (1024 * 1024 * 1024) == 0 {
            let current_time = begin_time.elapsed();
            let throughput_per_second =
                total_throughput as f64 / (1024.0 * 1024.0 * current_time.as_secs_f64());
            println!("Current Throughput: {} MB/s", throughput_per_second);
        }
    }
}
