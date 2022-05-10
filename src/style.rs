use std::fmt::Display;
use termion::{color, style};

pub fn good<T: Display>(c: T) -> String {
    format!("{}{}{}", color::Fg(color::Green), c, style::Reset)
}

pub fn bad<T: Display>(c: T) -> String {
    format!("{}{}{}", color::Fg(color::Red), c, style::Reset)
}

pub fn subtle<T: Display>(c: T) -> String {
    format!("{}{}{}", color::Fg(color::LightBlack), c, style::Reset)
}
