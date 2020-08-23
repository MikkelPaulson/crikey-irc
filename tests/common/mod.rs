use std::io;
use std::io::prelude::*;
use std::net;
use std::process;
use std::sync::mpsc;
use std::thread;

pub fn new_server<A: net::ToSocketAddrs>(
    addr: A,
) -> mpsc::Receiver<(mpsc::Sender<String>, mpsc::Receiver<String>)> {
    let server = net::TcpListener::bind(addr).expect("Couldn't bind test server.");

    let (tx, rx) = mpsc::channel::<(mpsc::Sender<String>, mpsc::Receiver<String>)>();
    thread::spawn(move || {
        let (connection, _) = server.accept().expect("Couldn't accept client connection.");
        let mut writer = connection.try_clone().expect("Connection clone failed.");
        let mut reader = io::BufReader::new(connection);

        let (sender_tx, sender_rx) = mpsc::channel::<String>();
        thread::spawn(move || loop {
            // Blocks
            if let Ok(message) = sender_rx.recv() {
                writer
                    .write(message.as_bytes())
                    .expect("Unable to write message to TCP connection.");
            } else {
                break;
            }
        });

        let (receiver_tx, receiver_rx) = mpsc::channel::<String>();
        thread::spawn(move || loop {
            let mut buffer = String::new();

            // Blocks
            if reader.read_line(&mut buffer).expect("Server error") == 0 {
                break;
            } else {
                receiver_tx
                    .send(buffer)
                    .expect("Unable to send message to parent.");
            }
        });

        tx.send((sender_tx, receiver_rx)).expect("Unable to send IO channel handles to parent.")
    });

    rx
}

pub fn new_client<A: net::ToSocketAddrs>(addr: A) -> io::Result<process::Child> {
    let server_ip: String = addr.to_socket_addrs()?.next().unwrap().to_string();

    process::Command::new("target/debug/irustc_bot")
        .arg(&server_ip)
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .spawn()
}
