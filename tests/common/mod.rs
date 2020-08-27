use std::io;
use std::io::prelude::*;
use std::net;
use std::process;
use std::sync::mpsc;
use std::thread;
use std::time;

pub fn init<A: net::ToSocketAddrs>(addr: A) -> (Client, Server) {
    let mut server = Server::new(&addr);
    let client = Client::new(&addr);
    server.accept_connection();

    (client, server)
}

pub struct Client {
    child: process::Child,
}

impl Client {
    pub fn new<A: net::ToSocketAddrs>(addr: &A) -> Client {
        let server_ip: String = addr.to_socket_addrs().unwrap().next().unwrap().to_string();

        let child = process::Command::new("target/debug/irustc_bot")
            .arg(&server_ip)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .spawn()
            .expect("Unable to spawn client process.");

        Client { child }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.child.kill().ok();
    }
}

pub struct Server {
    listener: Option<mpsc::Receiver<(mpsc::Sender<String>, mpsc::Receiver<String>)>>,
    sender: Option<mpsc::Sender<String>>,
    receiver: Option<mpsc::Receiver<String>>,
}

impl Server {
    pub fn new<A: net::ToSocketAddrs>(addr: &A) -> Server {
        let server = net::TcpListener::bind(addr).expect("Couldn't bind test server.");

        let (tx, rx) = mpsc::channel::<(mpsc::Sender<String>, mpsc::Receiver<String>)>();
        thread::spawn(move || {
            let (connection, _) = server.accept().expect("Couldn't accept client connection.");
            let mut writer = connection.try_clone().expect("Connection clone failed.");
            let reader = io::BufReader::new(connection);

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
            thread::spawn(move || {
                for input in reader.lines() {
                    if let Ok(line) = input {
                        if line.len() > 0 {
                            receiver_tx
                                .send(line)
                                .expect("Unable to send message to parent.");
                        }
                    } else {
                        break;
                    }
                }
            });

            tx.send((sender_tx, receiver_rx))
                .expect("Unable to send IO channel handles to parent.")
        });

        Server {
            listener: Some(rx),
            sender: None,
            receiver: None,
        }
    }

    pub fn accept_connection(&mut self) {
        let listener = self
            .listener
            .as_ref()
            .expect("Connection already established.");

        let (sender, receiver) = listener
            .recv_timeout(time::Duration::from_millis(10))
            .expect("Timed out waiting for connection.");

        self.listener = None;
        self.receiver = Some(receiver);
        self.sender = Some(sender);
    }

    pub fn read_line(&mut self) -> Option<String> {
        match self
            .receiver
            .as_ref()
            .expect("Connection not yet established.")
            .recv_timeout(time::Duration::from_millis(100))
        {
            Ok(line) => Some(line),
            Err(_) => None,
        }
    }

    pub fn truncate(&mut self) {
        while let Some(_) = self.read_line() { }
    }

    pub fn write_line(&self, message: &str) {
        let mut message = message.to_string();
        message.push_str("\r\n");
        self.sender
            .as_ref()
            .expect("Connection not yet established.")
            .send(message)
            .expect("Unable to send message.");
    }
}
