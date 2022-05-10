use crate::style::{bad, good};
use std::fmt::{self, Write};

pub struct Input<'a> {
    word: &'a str,
    cursor: usize,
    input: Vec<char>,
    pub correct: bool,
}

impl<'a> Input<'a> {
    pub fn new(word: &'a str) -> Self {
        let input = Vec::with_capacity(word.len());
        Input {
            word,
            cursor: 0,
            input,
            correct: true,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cursor == self.word.len()
    }

    pub fn update(&mut self, input: char) -> bool {
        let correct = match self.word.chars().nth(self.cursor) {
            None => false,
            Some(c) => c == input,
        };
        self.cursor += 1;
        self.input.push(input);
        self.correct &= correct;
        correct
    }

    pub fn delete_one(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.input.pop();
    }
}

impl fmt::Display for Input<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.word.chars().enumerate() {
            if i >= self.cursor {
                f.write_char(c)?;
            } else {
                if self.input[i] == c {
                    f.write_str(&good(c))?;
                } else {
                    f.write_str(&bad(c))?;
                }
            }
        }
        if self.cursor > self.word.len() {
            for i in self.word.len()..self.cursor {
                f.write_str(&bad(self.input[i]))?;
            }
        }

        Ok(())
    }
}
