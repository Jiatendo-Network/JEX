
struct SlidingWindow {
    sequence_id_counter: Box<Counter<u16>>,
    stream_settings: Box<StreamSettings>,
    timeout_manager: Box<TimeoutManager>,
}

impl SlidingWindow {
    fn set_cipher_key(&mut self, key: &[u8]) {
        self.stream_settings.encryption_algorithm.set_key(key);
    }

    fn next_outgoing_sequence_id(&mut self) -> u16 {
        self.sequence_id_counter.next()
    }

    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.stream_settings.encryption_algorithm.decrypt(data)
    }

    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.stream_settings.encryption_algorithm.encrypt(data)
    }
}

fn new_sliding_window() -> Box<SlidingWindow> {
    Box::new(SlidingWindow {
        sequence_id_counter: Box::new(Counter::new(0)),
        timeout_manager: Box::new(TimeoutManager::new()),
    })
}




