use configster;

#[derive(Debug, PartialEq)]
pub struct Data {
    pub realname: String,
    pub nick: String,
    pub password: String,
    pub server_addr: String,
}

impl Data {
    pub fn new() -> Self {
        Self {
            realname: String::new(),
            nick: String::new(),
            password: String::new(),
            server_addr: String::new(),
        }
    }

    pub fn get(&mut self) {
        let config_vec =
            configster::parse_file("./crikeyrc", ',').expect("Error reading config file");
        for i in &config_vec {
            match i.option.as_ref() {
                "realname" => self.realname = i.value.primary.clone(),
                "nick" => self.nick = i.value.primary.clone(),
                "password" => self.password = i.value.primary.clone(),
                "server_addr" => self.server_addr = i.value.primary.clone(),
                _ => println!("Invalid Option"),
            }
        }
    }
}

#[test]
fn test_config_getter() {
    let mut config_data = Data::new();
    config_data.get();
    assert_eq!(
        config_data,
        Data {
            realname: "Potato Johnson".to_string(),
            nick: "spudly".to_string(),
            password: String::from(""),
            server_addr: "127.0.0.1:6667".to_string(),
        }
    );
}
