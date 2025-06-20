struct StreamSettings {
    extra_retransmit_timeout_trigger: u32,
    max_packet_retransmissions: u32,
    keep_alive_timeout: u32,
    checksum_base: u32,
    fault_detection_enabled: bool,
    initial_rtt: u32,
    syn_initial_rtt: u32,
    encryption_algorithm: Box<dyn EncryptionAlgorithm>,
    extra_retransmit_timeout_multiplier: f64,
    window_size: u32,
    compression_algorithm: Box<dyn CompressionAlgorithm>,
    rtt_retransmit: u32,
    retransmit_timeout_multiplier: f64,
    max_silence_time: u32,
}

impl StreamSettings {
    fn new() -> Self {
        StreamSettings {
            extra_retransmit_timeout_trigger: 0x32,
            max_packet_retransmissions: 0x14,
            keep_alive_timeout: 1000,
            checksum_base: 0,
            fault_detection_enabled: true,
            initial_rtt: 0x2EE,
            syn_initial_rtt: 0xFA,
            encryption_algorithm: Box::new(RC4Encryption::new()),
            extra_retransmit_timeout_multiplier: 1.0,
            window_size: 8,
            compression_algorithm: Box::new(DummyCompression::new()),
            rtt_retransmit: 2,
            retransmit_timeout_multiplier: 1.25,
            max_silence_time: 10000,
        }
    }

    fn copy(&self) -> Box<Self> {
        Box::new(StreamSettings {
            extra_retransmit_timeout_trigger: self.extra_retransmit_timeout_trigger,
            max_packet_retransmissions: self.max_packet_retransmissions,
            keep_alive_timeout: self.keep_alive_timeout,
            checksum_base: self.checksum_base,
            fault_detection_enabled: self.fault_detection_enabled,
            initial_rtt: self.initial_rtt,
            syn_initial_rtt: self.syn_initial_rtt,
            encryption_algorithm: self.encryption_algorithm.clone(),
            extra_retransmit_timeout_multiplier: self.extra_retransmit_timeout_multiplier,
            window_size: self.window_size,
            compression_algorithm: self.compression_algorithm.clone(),
            rtt_retransmit: self.rtt_retransmit,
            retransmit_timeout_multiplier: self.retransmit_timeout_multiplier,
            max_silence_time: self.max_silence_time,
        })
    }
}

trait EncryptionAlgorithm {
    fn new() -> Self;
}

struct RC4Encryption;

impl EncryptionAlgorithm
