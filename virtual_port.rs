#[derive(Debug, Clone, Copy)]
enum VirtualPort {
    // Add your variants here
}

impl VirtualPort {
    fn set_stream_type(&mut self, stream_type: constants::StreamType) {
        *self = VirtualPort((*self as u8 & 0x0F) | ((stream_type as u8) << 4));
    }

    fn stream_type(&self) -> constants::StreamType {
        constants::StreamType((*self as u8 >> 4))
    }

    fn set_stream_id(&mut self, stream_id: u8) {
        *self = VirtualPort(((*self as u8 & 0xF0) | (stream_id & 0x0F)) as u8 as VirtualPort);
    }

    fn stream_id(&self) -> u8 {
        *self as u8 & 0xF
    }
}

