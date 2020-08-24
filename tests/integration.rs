mod common;

// > NICK baz
// > USER pjohnson local remote :Potato Johnson
// < :irc.example.net 001 baz :Welcome to the Internet Relay Network baz!~pjohnson@ircbot_irustc-bot_run_33951ac1d023.ircbot_default
// < :irc.example.net 002 baz :Your host is irc.example.net, running version ngircd-23 (x86_64/alpine/linux-musl)
// < :irc.example.net 003 baz :This server has been started Fri Aug 21 2020 at 03:21:11 (UTC)
// < :irc.example.net 004 baz irc.example.net ngircd-23 abBcCFiIoqrRswx abehiIklmMnoOPqQrRstvVz
// < :irc.example.net 005 baz RFC2812 IRCD=ngIRCd CHARSET=UTF-8 CASEMAPPING=ascii PREFIX=(qaohv)~&@%+ CHANTYPES=#&+ CHANMODES=beI,k,l,imMnOPQRstVz CHANLIMIT=#&+:10 :are supported on this server
// < :irc.example.net 005 baz CHANNELLEN=50 NICKLEN=9 TOPICLEN=490 AWAYLEN=127 KICKLEN=400 MODES=5 MAXLIST=beI:50 EXCEPTS=e INVEX=I PENALTY :are supported on this server
// < :irc.example.net 251 baz :There are 1 users and 0 services on 1 servers
// < :irc.example.net 254 baz 1 :channels formed
// < :irc.example.net 255 baz :I have 1 users, 0 services and 0 servers
// < :irc.example.net 265 baz 1 1 :Current local users: 1, Max: 1
// < :irc.example.net 266 baz 1 1 :Current global users: 1, Max: 1
// < :irc.example.net 250 baz :Highest connection count: 1 (4 connections received)
// < :irc.example.net 422 baz :MOTD file is missing

#[test]
fn it_authenticates() {
    let addr = "127.0.0.1:16667";

    let (_client, mut server) = common::init(addr);

    assert_eq!("NICK baz", server.read_line().expect("Nothing to read."));
    assert_eq!(
        "USER pjohnson local remote :Potato Johnson",
        server.read_line().expect("Nothing to read.")
    );
    assert_eq!(None, server.read_line());
}
