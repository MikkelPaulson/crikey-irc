use crikey_irc::run;
use std::env;
use std::io;
mod config;

fn main() -> io::Result<()> {
    let mut config_data = config::Data::new();
    config_data.get();
    let server_addr = env::args().nth(1).unwrap_or(config_data.server_addr);
    let nick = env::args().nth(2).unwrap_or(config_data.nick);
    let username = env::args().nth(3).unwrap_or("pjohnson".to_string());
    let realname = env::args().nth(4).unwrap_or(config_data.realname);

    run(server_addr, nick, username, realname)
}
