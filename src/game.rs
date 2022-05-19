use crate::history::History;
use crate::input::Input;
use crate::statistics::Statistics;
use crate::style::subtle;
use itertools::peek_nth;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::io::{stdin, stdout, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, event::Key, input::TermRead};

#[derive(Clone)]
pub enum GameMode {
    Words(usize),
}

impl Default for GameMode {
    fn default() -> Self {
        Self::Words(10)
    }
}

fn initialize_words(mode: &GameMode) -> Vec<&'static str> {
    let word_data = include_str!("../words.txt");
    let words = word_data.split(',');
    match mode {
        GameMode::Words(n) => words.choose_multiple(&mut thread_rng(), *n),
    }
}

pub struct Game {
    mode: GameMode,
    words: Vec<&'static str>,
    history: History,
    stdout: RawTerminal<Stdout>,
    num_peek: u16,
}

impl Game {
    pub fn new(mode: GameMode) -> Self {
        let words = initialize_words(&mode);
        Self {
            mode,
            words,
            history: History::default(),
            stdout: stdout().into_raw_mode().unwrap(),
            num_peek: 3,
        }
    }

    pub fn words(n: usize) -> Self {
        Self::new(GameMode::Words(n))
    }

    pub fn num_peek(mut self, n: u16) -> Self {
        self.num_peek = n;
        self
    }

    fn initialize_screen(&mut self) -> std::io::Result<()> {
        let (width, height) = termion::terminal_size()?;
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(width / 2, height / 2),
            cursor::Hide
        )?;
        self.stdout.flush()
    }

    fn clear_current_word(&mut self) -> std::io::Result<()> {
        let (width, height) = termion::terminal_size()?;
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(width / 2, height / 2),
            clear::CurrentLine,
        )?;
        self.stdout.flush()
    }

    fn clear_next_word(&mut self, i: u16) -> std::io::Result<()> {
        let (width, height) = termion::terminal_size()?;
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(width / 2, height / 2 + i + 1),
            clear::CurrentLine,
        )?;
        self.stdout.flush()
    }

    fn finalize(mut self) -> std::io::Result<()> {
        let statistics = Statistics::calculate(self.history, self.words);
        if let Some(statistics) = statistics {
            write!(self.stdout, "{}", statistics)?;
        }
        write!(self.stdout, "{}", cursor::Show)?;
        self.stdout.flush()
    }

    pub fn play(mut self) -> std::io::Result<()> {
        let mut stdin = stdin().keys();
        let mut on_last_word = false;
        let (width, height) = termion::terminal_size()?;
        self.initialize_screen()?;
        let w = self.words.clone();
        let mut words = peek_nth(w.iter());
        let num_peek = self.num_peek;
        'words: while let Some(current_word) = words.next() {
            self.history.words.push(current_word.clone());
            let mut input = Input::new(current_word);
            self.clear_current_word()?;
            write!(self.stdout, "{}", input)?;
            for i in 0..num_peek {
                self.clear_next_word(i)?;
                let next_word = words.peek_nth(i.into());
                if let Some(next) = next_word {
                    write!(
                        self.stdout,
                        "{}{}",
                        cursor::Goto(width / 2, height / 2 + 1 + i as u16),
                        subtle(next)
                    )?
                } else if i == 0 {
                    on_last_word = true;
                }
            }
            self.stdout.flush()?;
            loop {
                write!(self.stdout, "{}", cursor::Goto(width / 2, height / 2))?;
                self.stdout.flush()?;
                let c = stdin.next().unwrap()?;
                self.history.record_key(c);
                match c {
                    Key::Char('\t') => return Game::new(self.mode.clone()).play(),
                    Key::Char(' ') => break,
                    Key::Char(c) => {
                        let correct = input.update(c);
                        if correct && on_last_word && input.is_complete() {
                            break;
                        }
                    }
                    Key::Backspace => {
                        self.clear_current_word()?;
                        input.delete_one();
                    }
                    Key::Ctrl('c') => {
                        break 'words;
                    }
                    _ => {}
                }
                write!(self.stdout, "{}", input)?;
                self.stdout.flush()?;
            }
            self.clear_current_word()?;
        }
        self.finalize()?;
        Ok(())
    }
}
