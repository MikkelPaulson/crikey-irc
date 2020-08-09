use std::io;
use std::sync::mpsc;
use std::thread;

pub struct Terminal {
    rx: mpsc::Receiver<String>,
}

impl Terminal {
    pub fn new(reader: io::Stdin) -> Terminal {
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            tx.send(buffer).unwrap();
        });

        Terminal { rx }
    }

    pub fn read(&self) -> Option<String> {
        match self.rx.try_recv() {
            Ok(input) => Some(input),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => panic!("Stdin channel disconnected"),
        }
    }
}
