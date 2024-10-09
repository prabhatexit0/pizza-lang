use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Topping,
    Recipe,
    Slice,
    Extra,
    Oven,
    Serve,
    Cheese,

    // Identifiers and Literals
    Identifier,
    StringLiteral,
    NumberLiteral,

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    DoubleEquals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,

    // Other
    Eof,
    Illegal,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

struct Cursor {
    loc_x: usize,
    loc_y: usize,
    index: usize,
}

pub struct Lexer {
    source: String,
    cursor: Cursor,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let current_char = source.chars().nth(0);

        Lexer {
            source,
            cursor: Cursor { loc_x: 0, loc_y: 1, index: 0 },
            current_char,
        }
    }

    fn advance(&mut self) {
        self.cursor.index += 1;

        if let Some(current_char) = self.source.chars().nth(self.cursor.index) {
            self.current_char = Some(current_char);
            self.cursor.loc_x += 1;

            // Check if we've moved to a new line
            if current_char == '\n' {
                self.cursor.loc_y += 1;
                self.cursor.loc_x = 0;
            }
        } else {
            self.current_char = None;
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.current_char {
            Some(c) => match c {
                'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(),
                '0'..='9' => self.read_number(),
                '"' => self.read_string(),
                '+' => self.create_token(TokenKind::Plus, "+"),
                '-' => self.create_token(TokenKind::Minus, "-"),
                '*' => self.create_token(TokenKind::Asterisk, "*"),
                '/' => self.create_token(TokenKind::Slash, "/"),
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.create_token(TokenKind::DoubleEquals, "==")
                    } else {
                        self.create_token(TokenKind::Equals, "=")
                    }
                },
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.create_token(TokenKind::NotEquals, "!=")
                    } else {
                        self.create_token(TokenKind::Illegal, "!")
                    }
                },
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.create_token(TokenKind::GreaterThanOrEqual, ">=")
                    } else {
                        self.create_token(TokenKind::GreaterThan, ">")
                    }
                },
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.create_token(TokenKind::LessThanOrEqual, "<=")
                    } else {
                        self.create_token(TokenKind::LessThan, "<")
                    }
                },
                '(' => self.create_token(TokenKind::LeftParen, "("),
                ')' => self.create_token(TokenKind::RightParen, ")"),
                '{' => self.create_token(TokenKind::LeftBrace, "{"),
                '}' => self.create_token(TokenKind::RightBrace, "}"),
                ',' => self.create_token(TokenKind::Comma, ","),
                _ => self.create_token(TokenKind::Illegal, &c.to_string()),
            },
            None => return Some(self.create_token(TokenKind::Eof, "")),
        };

        self.advance();
        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start_position = self.cursor.index;

        while let Some(c) = self.current_char {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }

        let identifier = &self.source.as_str()[start_position..self.cursor.index];
        match identifier {
            "topping" => self.create_token(TokenKind::Topping, identifier),
            "recipe" => self.create_token(TokenKind::Recipe, identifier),
            "slice" => self.create_token(TokenKind::Slice, identifier),
            "extra" => self.create_token(TokenKind::Extra, identifier),
            "oven" => self.create_token(TokenKind::Oven, identifier),
            "serve" => self.create_token(TokenKind::Serve, identifier),
            "cheese" => self.create_token(TokenKind::Cheese, identifier),
            _ => self.create_token(TokenKind::Identifier, identifier),
        }
    }

    fn read_number(&mut self) -> Token {
        let start_position = self.cursor.index;
        while let Some(c) = self.current_char {
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            self.advance();
        }
        let number = &self.source.as_str()[start_position..self.cursor.index];
        self.create_token(TokenKind::NumberLiteral, number)
    }

    fn read_string(&mut self) -> Token {
        let start_position = self.cursor.loc_x;
        self.advance(); // Skip opening quote
        while let Some(c) = self.current_char {
            if c == '"' {
                break;
            }
            self.advance();
        }
        let string = &self.source.as_str()[start_position..self.cursor.loc_x - 1];
        self.create_token(TokenKind::StringLiteral, string)
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.cursor.index+1)
    }

    fn create_token(&self, kind: TokenKind, lexeme: &str) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            literal: None,
            line: self.cursor.loc_y,
        }
    }

    pub fn get_all_tokens(&mut self) -> Result<Vec<Token>, std::io::Error> {
        let mut tokens = Vec::new();
        let mut file = File::create("tokens.txt")?;
        while let Some(token) = self.next_token() {
            writeln!(file, "Token: {:?}", token)?;

            if token.kind == TokenKind::Eof {
                println!("Eof token");
                break;
            }

            tokens.push(token);
        }
        Ok(tokens)
    }
}
