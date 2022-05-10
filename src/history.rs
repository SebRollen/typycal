use std::time::{Duration, Instant};
use termion::event::Key;

#[derive(Default, Debug)]
pub struct History {
    pub timer: Option<Instant>,
    pub key_history: Vec<(Duration, Key)>,
    pub words: Vec<&'static str>,
}

impl History {
    pub fn record_key(&mut self, k: Key) {
        if let None = self.timer {
            self.timer = Some(Instant::now());
        }
        let t = self.timer.unwrap().elapsed();
        self.key_history.push((t, k));
    }
}
