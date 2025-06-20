
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use std::context::Context;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

type TimeoutManager = struct {
    ctx: Context,
    cancel: Context.CancelFunc,
    packets: Mutex<HashMap<uint16, PRUDPPacketInterface>>,
    stream_settings: StreamSettings,
}

impl TimeoutManager {
    fn new() -> Self {
        let (tx, rx) = channel();
        let (ctx, cancel) = Context::with_cancel(tx);
        let packets = Mutex::new(HashMap::new());
        let stream_settings = StreamSettings::new();
        TimeoutManager {
            ctx,
            cancel,
            packets,
            stream_settings,
        }
    }

    fn schedule_packet_timeout(&self, packet: PRUDPPacketInterface) {
        let endpoint = packet.sender().endpoint().clone();
        let rto = endpoint.compute_retransmit_timeout(packet);
        let ctx = self.ctx.clone();
        let cancel = self.cancel.clone();
        let timeout = Timeout::new(rto);
        packet.set_timeout(timeout);
        self.packets.insert(packet.sequence_id(), packet);
        thread::spawn(move || self.start(packet));
    }

    fn acknowledge_packet(&self, sequence_id: uint16) {
        self.packets.run_and_delete(sequence_id, |_, packet| {
            if packet.send_count() >= self.stream_settings.rtt_retransmit {
                let rttm = packet.sent_at().elapsed();
                packet.sender().rtt.adjust(rttm);
            }
        });
    }

    fn start(&self, packet: PRUDPPacketInterface) {
        let timeout = packet.get_timeout();
        let _ = timeout.ctx.recv();
        let connection = packet.sender().clone();
        if connection.connection_state!= StateConnected {
            return;
        }
        if self.packets.contains_key(&packet.sequence_id()) {
            let endpoint = packet.sender().endpoint().clone();
            if packet.send_count() < self.stream_settings.max_packet_retransmissions {
                packet.increment_send_count();
                packet.set_sent_at(Instant::now());
                let rto = endpoint.compute_retransmit_timeout(packet);
                let ctx = self.ctx.clone();
                let cancel = self.cancel.clone();
                let timeout = packet.get_timeout();
                timeout.timeout = rto;
                timeout.ctx = ctx;
                timeout.cancel = cancel;
               

