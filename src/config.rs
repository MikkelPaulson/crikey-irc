use std::env;
use std::io;

pub fn get_filename(homedir: &str) -> String {
    let config_home: String = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        Err(_e) => format!("{}{}", homedir, "/.config"),
    };

    let config_file = config_home + "/crikeyrc";

    if !std::path::Path::new(&config_file).exists() {
        println!("Creating config file: {}", &config_file);
        std::fs::write(
            &config_file,
            "realname = Potato Johnson\nnick = spudly\nserver_addr = 127.0.0.1:6667\n",
        )
        .expect("Unable to create config file");
    }
    config_file
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub realname: String,
    pub nick: String,
    pub password: String,
    pub server_addr: String,
}

impl Data {
    pub fn load(path: &str) -> io::Result<Self> {
        let mut c = Self {
            realname: String::new(),
            nick: String::new(),
            password: String::new(),
            server_addr: String::new(),
        };
        let config_vec = configster::parse_file(path, ',')?;
        for i in &config_vec {
            match i.option.as_ref() {
                "realname" => c.realname = i.value.primary.clone(),
                "nick" => c.nick = i.value.primary.clone(),
                "password" => c.password = i.value.primary.clone(),
                "server_addr" => c.server_addr = i.value.primary.clone(),
                _ => println!("Invalid Option"),
            }
        }
        Ok(c)
    }
}

#[test]
fn test_config_getter() {
    let config_data = Data::load("./crikeyrc.example");
    assert_eq!(
        config_data.unwrap(),
        Data {
            realname: "Potato Johnson".to_string(),
            nick: "spudly".to_string(),
            password: String::from(""),
            server_addr: "127.0.0.1:6667".to_string(),
        }
    );
}
