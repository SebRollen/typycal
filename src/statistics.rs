use crate::history::History;
use crate::input::Input;
use std::fmt;
use std::time::Duration;
use termion::{clear, cursor, event::Key};

pub struct Statistics {
    wpm: f64,
    raw_wpm: f64,
    accuracy: f64,
    time: Duration,
    correct_words: usize,
    incorrect_words: usize,
}

impl Statistics {
    pub fn calculate(history: History, words: Vec<&'static str>) -> Option<Statistics> {
        let mut correct_words = 0;
        let mut incorrect_words = 0;
        let mut correct_chars = 0;
        let mut incorrect_chars = 0;
        let mut inputs = words.into_iter().map(Input::new).peekable();
        let mut current_input = inputs.next().unwrap();
        let mut final_time = Duration::ZERO;
        for (t, k) in history.key_history {
            final_time = t;
            match k {
                Key::Char(' ') => {
                    if current_input.is_complete() && current_input.correct {
                        correct_words += 1;
                    } else {
                        incorrect_words += 1;
                    }
                    match inputs.next() {
                        Some(n) => current_input = n,
                        None => break,
                    }
                }
                Key::Char(c) => {
                    let correct = current_input.update(c);
                    if correct {
                        correct_chars += 1;
                    } else {
                        incorrect_chars += 1;
                    }
                    if current_input.is_complete()
                        && current_input.correct
                        && inputs.peek().is_none()
                    {
                        correct_words += 1
                    }
                }
                Key::Backspace => {
                    current_input.delete_one();
                }
                Key::Ctrl('c') => break,
                _ => unreachable!("Unexpected key in history"),
            }
        }
        let wpm = correct_chars as f64 / final_time.as_millis() as f64 * 12000.0;
        let raw_wpm =
            (correct_chars + incorrect_chars) as f64 / final_time.as_millis() as f64 * 12000.0;
        let accuracy = correct_chars as f64 / (correct_chars + incorrect_chars) as f64;
        Some(Statistics {
            wpm,
            raw_wpm,
            accuracy,
            time: final_time,
            correct_words,
            incorrect_words,
        })
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", cursor::Goto(1, 1), clear::CurrentLine)?;
        write!(
            f,
            "T: {}s, C: {}, I: {}",
            self.time.as_millis() as f64 / 1000.0,
            self.correct_words,
            self.incorrect_words,
        )?;
        write!(f, "{}{}", cursor::Goto(1, 2), clear::CurrentLine)?;
        write!(
            f,
            "WPM: {:.1}, Raw: {:.1}, Accuracy {:.1}",
            self.wpm,
            self.raw_wpm,
            self.accuracy * 100.0
        )
    }
}
