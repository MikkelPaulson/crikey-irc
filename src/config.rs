use configster;

pub struct Data {
    pub realname: String,
    pub nick: String,
    pub password: String,
    pub server_addr: String,
    pub server_port: u32,
}

impl Data {
    pub fn new() -> Self {
        Self {
            realname: String::new(),
            nick: String::new(),
            password: String::new(),
            server_addr: String::new(),
            server_port: 0,
        }
    }
}

pub fn get() -> Data {
    let config_vec = configster::parse_file("./crikeyrc", ',').expect("Error reading config file");
    let mut config_data = Data::new();
    for i in &config_vec {
        match i.option.as_ref() {
            "realname" => config_data.realname = i.value.primary.clone(),
            "nick" => config_data.nick = i.value.primary.clone(),
            "password" => config_data.password = i.value.primary.clone(),
            "server_addr" => {
                config_data.server_addr = i.value.primary.clone();
                match i.value.attributes.get(0).is_some() {
                    true => {
                        config_data.server_port =
                            i.value.attributes[0].parse().expect("Invalid port number")
                    }
                    false => println!("No port option given for server"),
                }
            }
            _ => println!("Invalid Option"),
        }
    }
    config_data
}

#[test]
fn test_config_getter() {
    let config_data = get();
    assert_eq!(config_data.realname, "Potato Johnson".to_string());
    assert_eq!(config_data.nick, "spudly".to_string());
    assert_eq!(config_data.server_addr, "127.0.0.1".to_string());
    assert_eq!(config_data.server_port, 6667);
}
