use crikey_irc::run;
use std::env;
use std::io::{self, ErrorKind};
mod config;

fn main() -> io::Result<()> {
    let homedir: String = match dirs::home_dir() {
        None => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "Unable to determine homedir",
            ))
        }
        Some(homedir) => homedir.to_str().unwrap().into(),
    };

    let config_file = config::get_filename(&homedir);
    let mut config_data = config::Data::new();
    config_data.get(&config_file);

    let server_addr = env::args().nth(1).unwrap_or(config_data.server_addr);
    let nick = env::args().nth(2).unwrap_or(config_data.nick);
    let username = env::args().nth(3).unwrap_or("pjohnson".to_string());
    let realname = env::args().nth(4).unwrap_or(config_data.realname);

    run(server_addr, nick, username, realname)
}
