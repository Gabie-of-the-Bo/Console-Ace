use std::time::{Duration, Instant};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Timer {
    start_time: Option<Instant>,
    duration: Duration
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Timer { start_time: None, duration }
    }

    pub fn new_started(duration: Duration) -> Self {
        Timer { start_time: Some(Instant::now()), duration }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn exhaust(&mut self) {
        self.start_time = None;
    }

    pub fn elapsed(&self) -> Option<Duration> {
        if self.start_time.is_none() {
            None
        
        } else {
            Some(Instant::now().duration_since(self.start_time.unwrap()))
        }
    }

    pub fn done(&self) -> bool {
        self.start_time.is_none() || Instant::now().duration_since(self.start_time.unwrap()) >= self.duration
    }
}