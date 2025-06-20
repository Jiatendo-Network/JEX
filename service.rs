
pub trait ServiceProtocol {
    fn handle_packet(&mut self, packet: &dyn PacketInterface);
    fn endpoint(&self) -> &dyn EndpointInterface;
    fn set_endpoint(&mut self, endpoint: &dyn EndpointInterface);
}




