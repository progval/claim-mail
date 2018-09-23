use std::net::TcpStream;
use native_tls::{TlsConnector, TlsStream};
use imap::{client::Client, Capabilities, ZeroCopy};

pub struct Connection {
    imap_socket: Client<TlsStream<TcpStream>>,
    capabilities: ZeroCopy<Capabilities>,
}

impl Connection {
    pub fn new(domain: &str, port: u16, username: &str, password: &str) -> Connection {
        let socket_addr = (domain, port);
        let ssl_connector = TlsConnector::builder().unwrap().build().unwrap();
        let mut imap_socket = Client::secure_connect(socket_addr, domain, &ssl_connector).unwrap();
        imap_socket.login(username, password).unwrap();
        let capabilities = imap_socket.capabilities().unwrap();

        Connection {
            imap_socket,
            capabilities,
        }
    }
}

