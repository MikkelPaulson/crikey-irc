mod common;

#[test]
fn it_authenticates() {
    let (_client, mut server) = common::init("127.0.0.1:16667");

    assert_eq!("NICK spudly", server.read_line().expect("Nothing to read."));
    assert_eq!(
        "USER pjohnson 0 * :Potato Johnson",
        server.read_line().expect("Nothing to read.")
    );
    assert_eq!(None, server.read_line());
}

#[test]
fn it_responds_to_ping() {
    let (_client, mut server) = common::connect("127.0.0.1:16668");

    server.write_line("PING :irc.example.com");

    assert_eq!(
        "PONG spudly irc.example.com",
        server.read_line().expect("Nothing to read.")
    );
}
