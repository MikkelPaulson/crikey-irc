use irustc_bot::run;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let server_addr = env::args().nth(1).unwrap_or("127.0.0.1:6667".to_string());

    run(server_addr)
}
