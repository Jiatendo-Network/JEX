struct Timeout {
    timeout: std::time::Duration,
    ctx: std::sync::Context,
    cancel: std::sync::CancelFunc,
}

impl Timeout {
    // SetRTO sets the timeout field on this instance
    fn set_rto(&mut self, timeout: std::time::Duration) {
        self.timeout = timeout;
    }

    // GetRTO gets the timeout field of this instance
    fn rto(&self) -> std::time::Duration {
        self.timeout
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout {
            timeout: std::time::Duration::from_millis(0),
           ..Default::default()
        }
    }
}

