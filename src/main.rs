use irustc_bot::run;
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let server_addr = env::args().nth(1).unwrap_or("127.0.0.1:6667".to_string());
    let nick = env::args().nth(2).unwrap_or("spudly".to_string());
    let username = env::args().nth(3).unwrap_or("pjohnson".to_string());
    let realname = env::args().nth(4).unwrap_or("Potato Johnson".to_string());

    run(server_addr, nick, username, realname)
}
