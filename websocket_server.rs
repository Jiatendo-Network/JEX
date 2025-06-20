use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::collections::HashMap;

struct WebSocketServer {
    prudp_server: Arc<Mutex<PRUDPServer>>,
    upgrader: Arc<WebSocketUpgrader>,
}

struct WebSocketUpgrader {
    handler: WebSocketEventHandler,
}

struct WebSocketEventHandler {
    prudp_server: Arc<Mutex<PRUDPServer>>,
}

impl WebSocketEventHandler {
    fn on_open(&self, socket: &TcpStream) {
        let deadline = Instant::now() + Duration::from_secs(5);
        socket.set_read_timeout(Some(deadline));
    }

    fn on_close(&self, socket: &TcpStream, _error: std::io::Error) {
        self.prudp_server.lock().unwrap().endpoints.iter().for_each(|(stream_id, pep)| {
            pep.connections.iter().for_each(|(discriminator, pc)| {
                if pc.socket.remote_addr().unwrap() == socket.peer_addr().unwrap() {
                    self.prudp_server.lock().unwrap().cleanup_connection(pc);
                }
            });
        });
    }

    fn on_ping(&self, socket: &TcpStream, _payload: &[u8]) {
        let deadline = Instant::now() + Duration::from_secs(5);
        socket.set_read_timeout(Some(deadline));
        socket.write_all(b"Pong").unwrap();
    }

    fn on_pong(&self, _socket: &TcpStream, _payload: &[u8]) {}

    fn on_message(&self, socket: &TcpStream, message: &[u8]) {
        let mut packet_data = Vec::new();
        packet_data.extend_from_slice(message);
        if let Err(err) = self.prudp_server.lock().unwrap().handle_socket_message(&packet_data, socket.peer_addr().unwrap(), socket) {
            log::error!("Error: {}", err);
        }
    }
}

struct PRUDPServer {
    endpoints: HashMap<u8, PRUDPEndPoint>,
}

impl PRUDPServer {
    fn cleanup_connection(&mut self, connection: &PRUDPConnection) {
        // implementation
    }

    fn handle_socket_message(&mut self, packet_data: &[u8], remote_addr: std::net::IpAddr, socket: &TcpStream) -> std::io::Result<()> {
        // implementation
    }
}

