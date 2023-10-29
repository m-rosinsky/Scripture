//! # Lexer
//! 
//! `Lexer` is the crate responsible for processing raw input files
//! into tokens for the parser to handle.

use std::fs::File;
use std::io::{
    self,
    BufRead,
};

/// This struct defines a Lexer context, which is used to tokenize
/// raw input files into a table of tokens.
pub struct Lexer {
    pub table: Vec<Token>,
    line_num: u32,
    line_idx: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            table: Vec::new(),
            line_num: 1,
            line_idx: 0,
        }
    }

    /// This function tokenizes a raw input file.
    pub fn tokenize_file(&mut self, input_f: &str) -> Result<(), std::io::Error> {
        let file = File::open(input_f)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    // Tokenize individual line.
                    self.tokenize_line(line_content)?;
                    self.line_num += 1;
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}

impl Lexer {
    fn tokenize_line(&mut self, line: String) -> Result<(), std::io::Error> {
        self.table.push(
            Token::new(line.as_str(), self.line_num, self.line_idx + 1)
        );
        Ok(())
    }
}

/// This struct defines a token context, which contains
/// information about a given token, including the token string
/// itself.
#[derive(Debug)]
pub struct Token {
    /// The string representation of the token.
    token: String,
    /// The line number the token occurs on within the raw input file.
    line_num: u32,
    /// The column number within the raw input file.
    col_num: u32,
}

impl Token {
    fn new(token: &str, line_num: u32, col_num: u32) -> Self {
        Self {
            token: String::from(token),
            line_num,
            col_num
        }
    }
}