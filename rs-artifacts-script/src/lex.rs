#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Keywords
    Move, // move
    To,   // to
    Load, // load

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

static KEYWORDS: phf::Map<&'static str, Token> = phf::phf_map! {
    "move" => Token::Move,
    "to" => Token::To,
    "load" => Token::Load,
};

fn try_get_keyword(s: impl AsRef<str>) -> Option<Token> {
    KEYWORDS.get(s.as_ref()).cloned()
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

fn is_ident_start_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c.is_ascii_digit() || c == '_'
}

fn is_num_char(c: char) -> bool {
    c.is_ascii_digit()
}

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

    /// Assuming the next characters in the input are an identifier, reads the next identifier.
    /// If the identifier is a valid keyword, it returns the keyword token.
    fn try_read_next_ident(&mut self) -> Result<Token> {
        let mut buffer = String::new();
        while let Some(&c) = self.input.peek() {
            if is_ident_char(c) {
                buffer.push(c);
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
            Ok(try_get_keyword(&buffer).unwrap_or(Token::Identifier(buffer)))
        }
    }

    /// Assuming the next characters in the input are a number, reads the next number.
    fn try_read_next_num(&mut self) -> Result<Token> {
        let mut buffer = String::new();
        while let Some(&c) = self.input.peek() {
            if is_num_char(c) {
                buffer.push(c);
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

    /// Assuming the next character in the input buffer is a quote, reads the next string.
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

    fn try_read_next_symbol(&mut self) -> Result<Token> {
        let c = self.input.next().ok_or_else(|| LexerError {
            msg: "Expected symbol, got nothing".to_string(),
        })?;

        match c {
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semicolon),
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            _ => Err(LexerError {
                msg: format!("Unexpected symbol: {:?}", c),
            }),
        }
    }
}

impl<T: Iterator<Item = char>> Iterator for Lexer<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();
        match self.input.peek() {
            // Lex keywords and identifiers.
            Some(&c) if is_ident_start_char(c) => {
                let token = self
                    .try_read_next_ident()
                    .unwrap_or_else(Token::Error);
                Some(token)
            }
            // Lex numbers
            Some(&c) if is_num_char(c) => {
                let token = self
                    .try_read_next_num()
                    .unwrap_or_else(Token::Error);
                Some(token)
            }
            // Lex strings
            Some(&'"') => {
                let token = self
                    .try_read_next_str()
                    .unwrap_or_else(Token::Error);
                Some(token)
            }, 
            // Lex symbols and operators
            Some(_) => {
                let token = self
                    .try_read_next_symbol()
                    .unwrap_or_else(Token::Error);
                Some(token)
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
    fn it_lexes_keywords() {
        let input = "move to load";
        let mut lexer = Lexer::new(input.chars());

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Move));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::To));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Load));

        let actual = lexer.next();
        assert_eq!(actual, None);
    }

    #[test]
    fn it_lexes_numbers() {
        let input = "123 456";
        let mut lexer = Lexer::new(input.chars());

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Number("123".to_string())));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Number("456".to_string())));

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

    #[test]
    fn it_lexes_symbols() {
        let input = ", ; ( )";
        let mut lexer = Lexer::new(input.chars());

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Comma));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::Semicolon));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::OpenParen));

        let actual = lexer.next();
        assert_eq!(actual, Some(Token::CloseParen));

        let actual = lexer.next();
        assert_eq!(actual, None);
    }
}
