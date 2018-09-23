use std::collections::BTreeMap;

pub fn example_config() -> Config {
    let mut imap_servers = BTreeMap::new();
    imap_servers.insert("example".to_string(), ImapServer {
        domain: "example.org".to_string(),
        port: 993,
        username: "test".to_string(),
        password: "mypassword".to_string(),
    });
    Config {
        imap_servers
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImapServer {
    pub domain: String,
    #[serde(default = "default_imap_port")]
    pub port: u16,
    pub username: String,
    pub password: String,
}

fn default_imap_port() -> u16 {
    993
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub imap_servers: BTreeMap<String, ImapServer>,
}
