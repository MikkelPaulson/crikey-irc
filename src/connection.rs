use std::io;
use std::io::prelude::*;
use std::net;
//use std::sync::mpsc;

pub struct Connection<'a> {
    reader: Box<dyn 'a + io::BufRead>,
    //writer: Box<dyn 'a + Write>,
}

impl Connection<'_> {
    pub fn new(stream: &net::TcpStream) -> Connection {
        //stream.set_nonblocking(true).unwrap();

        let reader = io::BufReader::new(stream);
        //let writer = io::BufWriter::new(stream);

        Connection {
            reader: Box::new(reader),
            //writer: Box::new(writer),
        }
    }

    pub fn poll(&mut self) -> Option<String> {
        let mut buffer = String::new();
        if let Ok(_) = self.reader.read_line(&mut buffer) {
            Some(buffer)
        } else {
            None
        }

        //match self.reader.read_line(&mut buffer) {
        //Ok(_) => Some(buffer),
        //Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => None,
        //Err(e) => panic!("IO error: {}", e)
        //}
    }

    //pub fn sendCommand(command: Command);

    //pub fn send_command_raw(&mut self, command: String) -> std::io::Result<()> {
    //self.writer.write(command.as_bytes())?;
    //Ok(())
    //}
}
