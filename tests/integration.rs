mod common;

// > NICK spudly
// > USER pjohnson local remote :Potato Johnson
// < :irc.example.net 001 spudly :Welcome to the Internet Relay Network baz!~pjohnson@ircbot_irustc-bot_run_33951ac1d023.ircbot_default
// < :irc.example.net 002 spudly :Your host is irc.example.net, running version ngircd-23 (x86_64/alpine/linux-musl)
// < :irc.example.net 003 spudly :This server has been started Fri Aug 21 2020 at 03:21:11 (UTC)
// < :irc.example.net 004 spudly irc.example.net ngircd-23 abBcCFiIoqrRswx abehiIklmMnoOPqQrRstvVz
// < :irc.example.net 005 spudly RFC2812 IRCD=ngIRCd CHARSET=UTF-8 CASEMAPPING=ascii PREFIX=(qaohv)~&@%+ CHANTYPES=#&+ CHANMODES=beI,k,l,imMnOPQRstVz CHANLIMIT=#&+:10 :are supported on this server
// < :irc.example.net 005 spudly CHANNELLEN=50 NICKLEN=9 TOPICLEN=490 AWAYLEN=127 KICKLEN=400 MODES=5 MAXLIST=beI:50 EXCEPTS=e INVEX=I PENALTY :are supported on this server
// < :irc.example.net 251 spudly :There are 1 users and 0 services on 1 servers
// < :irc.example.net 254 spudly 1 :channels formed
// < :irc.example.net 255 spudly :I have 1 users, 0 services and 0 servers
// < :irc.example.net 265 spudly 1 1 :Current local users: 1, Max: 1
// < :irc.example.net 266 spudly 1 1 :Current global users: 1, Max: 1
// < :irc.example.net 250 spudly :Highest connection count: 1 (4 connections received)
// < :irc.example.net 422 spudly :MOTD file is missing

#[test]
fn it_authenticates() {
    let addr = "127.0.0.1:16667";

    let (_client, mut server) = common::init(addr);

    assert_eq!("NICK spudly", server.read_line().expect("Nothing to read."));
    assert_eq!(
        "USER pjohnson 0 * :Potato Johnson",
        server.read_line().expect("Nothing to read.")
    );
    assert_eq!(None, server.read_line());
}

#[test]
fn it_responds_to_ping() {
    let addr = "127.0.0.1:16667";

    let (_client, mut server) = common::init(addr);
    server.truncate();
    server.write_line("PING :irc.example.com");

    assert_eq!(
        "PONG spudly irc.example.com",
        server.read_line().expect("Nothing to read.")
    );
}