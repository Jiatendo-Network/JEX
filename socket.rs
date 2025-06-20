struct SocketConnection {
    server: Option<&PRUDPServer>,
    address: std::net::SocketAddr,
    web_socket_connection: Option<&gws::Conn>,
}

impl SocketConnection {
    fn new(server: &PRUDPServer, address: std::net::SocketAddr, web_socket_connection: &gws::Conn) -> Self {
        SocketConnection {
            server: Some(server),
            address,
            web_socket_connection: Some(web_socket_connection),
        }
    }
}


