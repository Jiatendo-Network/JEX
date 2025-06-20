
const alpha: f64 = 1.0 / 8.0;
const beta: f64 = 1.0 / 4.0;
const k: f64 = 4.0;

struct RTT {
    mutex: std::sync::Mutex,
    last_rtt: f64,
    average: f64,
    variance: f64,
    initialized: bool,
}

impl RTT {
    fn new() -> Self {
        RTT {
            mutex: std::sync::Mutex::new(),
            last_rtt: 0.0,
            average: 0.0,
            variance: 0.0,
            initialized: false,
        }
    }

    fn set_rtt(&mut self, next: f64) {
        self.mutex.lock();
        if self.initialized {
            self.variance = (1.0 - beta) * self.variance + beta * (self.variance - next).abs();
            self.average = (1.0 - alpha) * self.average + alpha * next;
        } else {
            self.last_rtt = next;
            self.variance = next / 2.0;
            self.average = next + k * self.variance;
            self.initialized = true;
        }
        self.mutex.unlock();
    }

    fn get_smoothed_avg(&self) -> f64 {
        self.average / 16.0
    }

    fn get_smoothed_dev(&self) -> f64 {
        self.variance / 8.0
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn average(&self) -> time::Duration {
        time::Duration::from_nanos((self.average * 1_000_000_000.0) as u64)
    }
}




