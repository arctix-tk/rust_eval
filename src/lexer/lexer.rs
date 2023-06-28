use std::fmt::Display;

use anyhow::Result;

/// represents different types of tokens recognized by the lexer
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    One,
    Zero,
    LPar,
    RPar,
    Mult(u8), // stores precedence of Operation
    Add(u8),
    Or(u8),
    True,
    False,
    Eof, // end of file
}

impl Display for Token {
    /// Pretty printing
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::One => write!(f, "1"),
            Token::Zero => write!(f, "0"),
            Token::LPar => write!(f, "("),
            Token::RPar => write!(f, ")"),
            Token::Mult(_) => write!(f, "*"),
            Token::Add(_) => write!(f, "+"),
            Token::Or(_) => write!(f, "||"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Eof => write!(f, "Eof"),
        };
    }
}

/// Lexer struct that tokenizes the input string
pub struct Lexer {
    /// holds state of the lexer: current position + character in the input stream
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

/// Lexer struct that tokenizes the input string
impl Lexer {
    /// creates a new lexer instance, initializes lexer state
    pub fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lex.read_char();
        return lex;
    }

    /// retrieves the next token from the input string until EOF
    pub fn next_token(&mut self) -> Result<Token> {

        // skip any whitespace characters
        self.skip_whitespace();

        // assign input string with token type
        let tok = match self.ch {
            b'(' => Token::LPar,
            b')' => Token::RPar,
            b'*' => Token::Mult(2),
            b'+' => Token::Add(1),
            b'0' => Token::Zero,
            b'1' => Token::One,
            b'a'..=b'z' | b'A'..=b'Z' | b'|' | b'_' => {
                let ident = self.read_ident();
                // match found identifier with existing token types
                return Ok(match ident.as_str() {
                    "false" => Token::False,
                    "true" => Token::True,
                    "||" => Token::Or(0),
                    _ => unreachable!("Unallowed character"),
                });
            }
            0 => Token::Eof, // end of file
            _ => unreachable!("Unallowed character"),
        };

        self.read_char();
        return Ok(tok);
    }

    /// skips any whitespace characters of input string
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    /// reads an identifier from the input stream
    fn read_ident(&mut self) -> String {
        let pos = self.position;
        // position advaces until stop 
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.ch == b'|' {
            self.read_char();
        }
        // returns string of found identifier
        return String::from_utf8_lossy(&self.input[pos..self.position]).to_string();
    }

    /// reads the next character from the input stream & updates the lexer's state accordingly
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::{Lexer, Token};

    #[test]
    fn get_next_token() -> Result<()> {
        let input = "(0 * 1) + true * false || true";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            Token::LPar,
            Token::Zero,
            Token::Mult(2),
            Token::One,
            Token::RPar,
            Token::Add(1),
            Token::True,
            Token::Mult(2),
            Token::False,
            Token::Or(0),
            Token::True,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lexer.next_token()?;
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }

        return Ok(());
    }
}
