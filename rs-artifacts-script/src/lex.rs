#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Keywords
    Move, // move
    To,   // to

    // Literals
    Identifier(String), // foo
    String(String),     // "foo"
    Number(String),     // 123

    // Symbols
    Comma,      // ,
    Semicolon,  // ;
    OpenParen,  // (
    CloseParen, // )

    // Special
    Error(LexerError),
}

#[derive(Debug, Clone)]
pub struct Lexer<T: Iterator<Item = char>> {
    input: std::iter::Peekable<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerError {
    msg: String,
}
type Result<T> = std::result::Result<T, LexerError>;

impl<T: Iterator<Item = char>> Lexer<T> {
    pub fn new(input: T) -> Self {
        Lexer {
            input: input.peekable(),
        }
    }

    /// Consumes all whitespace characters from the input.
    fn eat_whitespace(&mut self) {
        while let Some(c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn try_read_next_ident(&mut self) -> Result<Token> {
        let mut buffer = String::new();
        while let Some(c) = self.input.peek() {
            if c.is_alphabetic() || c.is_digit(10) || *c == '_' {
                buffer.push(*c);
                self.input.next();
            } else {
                break;
            }
        }

        if buffer.is_empty() {
            Err(LexerError {
                msg: "Expected identifier, got nothing".to_string(),
            })
        } else {
            // TODO: Handle keywords (phf?)
            Ok(Token::Identifier(buffer))
        }
    }

    fn try_read_next_num(&mut self) -> Result<Token> {
        let mut buffer = String::new();
        while let Some(c) = self.input.peek() {
            if c.is_digit(10) {
                buffer.push(*c);
                self.input.next();
            } else {
                break;
            }
        }

        if buffer.is_empty() {
            Err(LexerError {
                msg: "Expected number, got nothing".to_string(),
            })
        } else {
            Ok(Token::Number(buffer))
        }
    }

    fn try_read_next_str(&mut self) -> Result<Token> {
        // Ensure that the next character is a quote to begin the string.
        let c = self.input.next();
        if c != Some('"') {
            return Err(LexerError {
                msg: format!("Expected quote to begin string, got {:?}", c),
            });
        }

        // Read until the end of the input or the next quote.
        let mut buffer = String::new();
        let mut closed = false;
        while let Some(c) = self.input.peek() {
            if *c == '"' {
                self.input.next();
                closed = true;
                break;
            } else {
                buffer.push(*c);
                self.input.next();
            }
        }

        if !closed {
            Err(LexerError {
                msg: "Expected closing quote for string, got nothing".to_string(),
            })
        } else {
            Ok(Token::String(buffer))
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        match self.input.peek() {
            Some(c) => {
                // Lex keywords and identifiers.
                if c.is_alphabetic() {
                    let token = self
                        .try_read_next_ident()
                        .unwrap_or_else(|err| Token::Error(err));
                    Some(token)
                }
                // Lex numbers
                else if c.is_digit(10) {
                    let token = self
                        .try_read_next_num()
                        .unwrap_or_else(|err| Token::Error(err));
                    Some(token)
                }
                // Lex strings
                else if *c == '"' {
                    let token = self
                        .try_read_next_str()
                        .unwrap_or_else(|err| Token::Error(err));
                    Some(token)
                } else {
                    None
                }

                // Lex symbols
            }
            _ => None,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_eats_whitespace() {
        let input = " \t\nfoo";
        let mut lexer = Lexer::new(input.chars());

        lexer.eat_whitespace();
        assert_eq!(lexer.input.next(), Some('f'));
        assert_eq!(lexer.input.next(), Some('o'));
        assert_eq!(lexer.input.next(), Some('o'));
        assert_eq!(lexer.input.next(), None);
    }

    #[test]
    fn it_lexes_identifiers() {
        let input = "foo bar";
        let mut lexer = Lexer::new(input.chars());

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Identifier("foo".to_string())));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Identifier("bar".to_string())));

        let actual = lexer.next();
        assert_eq!(actual, None);
    }

    #[test]
    fn it_lexes_strings() {
        let input = "\"foo bar\" \"baz\"";
        let mut lexer = Lexer::new(input.chars());

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::String("foo bar".to_string())));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::String("baz".to_string())));

        let actual = lexer.next();
        assert_eq!(actual, None);
    }

    fn ensure_result<T>(result: Result<T>) -> T {
        match result {
            Ok(value) => value,
            Err(err) => panic!("Unexpected error: {:?}", err),
        }
    }
}
