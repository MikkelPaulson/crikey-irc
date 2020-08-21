use irustc_bot::run;
use std::io;

fn main() -> io::Result<()> {
    run("irc-server:6667")
}
