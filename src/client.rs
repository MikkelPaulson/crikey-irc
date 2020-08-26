use crate::connection;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Client {
    connection: Rc<RefCell<connection::Connection>>,
}

impl Client {
    pub fn new(connection: Rc<RefCell<connection::Connection>>) -> Client {
        Client { connection }
    }

    fn pong(&self, command: &connection::Command) {
        if let connection::Command::Ping { server1, .. } = command {
            self.connection
                .borrow_mut()
                .send_command(connection::Command::Pong {
                    server1: "Me".to_string(),
                    server2: Some(server1.to_string()),
                })
                .ok();
        }
    }
}
