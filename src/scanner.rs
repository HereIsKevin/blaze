use std::mem;

use crate::error::SyntaxError;
use crate::kind::Kind;
use crate::token::Token;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<SyntaxError>,
    start: usize,
    current: usize,
    line: usize,
    parens: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),
            errors: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            parens: 0,
        }
    }

    pub fn scan(&mut self) -> (Vec<Token>, Vec<SyntaxError>) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_semicolon();
        self.tokens.push(Token {
            kind: Kind::EOF,
            lexeme: String::new(),
            line: self.line,
        });

        let tokens = mem::take(&mut self.tokens);
        let errors = mem::take(&mut self.errors);

        (tokens, errors)
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => {
                self.add_token(Kind::LeftParen);
                self.parens += 1;
            }
            ')' => {
                self.add_token(Kind::RightParen);
                self.parens -= 1;
            }
            '{' => self.add_token(Kind::LeftBrace),
            '}' => self.add_token(Kind::RightBrace),
            ',' => self.add_token(Kind::Comma),
            '+' => self.add_token(Kind::Plus),
            '-' => self.add_token(Kind::Minus),
            '*' => self.add_token(Kind::Star),
            '/' if self.compare('/') => self.scan_comment(),
            '/' => self.add_token(Kind::Slash),
            '?' => self.add_token(Kind::Question),
            ':' => self.add_token(Kind::Colon),
            ';' => self.add_token(Kind::Semicolon),
            '!' if self.compare('=') => self.add_token(Kind::BangEqual),
            '!' => self.add_token(Kind::Bang),
            '=' if self.compare('=') => self.add_token(Kind::EqualEqual),
            '=' => self.add_token(Kind::Equal),
            '<' if self.compare('=') => self.add_token(Kind::LessEqual),
            '<' => self.add_token(Kind::Less),
            '>' if self.compare('=') => self.add_token(Kind::GreaterEqual),
            '>' => self.add_token(Kind::Greater),
            '&' if self.compare('&') => self.add_token(Kind::AmpAmp),
            '|' if self.compare('|') => self.add_token(Kind::BarBar),
            '\n' => self.scan_newline(),
            ' ' | '\r' | '\t' => (),
            '"' => self.scan_string(),
            '0'..='9' => self.scan_number(),
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier(),
            _ => self.add_error("Unexpected character."),
        }
    }

    fn scan_comment(&mut self) {
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn scan_newline(&mut self) {
        if self.parens <= 0 {
            self.add_semicolon();
        }

        self.line += 1
    }

    fn scan_string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            self.add_error("Unterminated string.");
        } else {
            self.advance();
            self.add_token(Kind::String);
        }
    }

    fn scan_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(Kind::Number);
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        let kind = match text.as_str() {
            "if" => Kind::If,
            "else" => Kind::Else,
            "fn" => Kind::Fn,
            "return" => Kind::Return,
            "false" => Kind::False,
            "true" => Kind::True,
            "loop" => Kind::Loop,
            "break" => Kind::Break,
            "continue" => Kind::Continue,
            "let" => Kind::Let,
            "type" => Kind::Type,
            _ => Kind::Identifier,
        };

        self.add_token(kind);
    }

    fn compare(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count()
    }

    fn add_token(&mut self, kind: Kind) {
        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        self.tokens.push(Token {
            kind,
            lexeme: text,
            line: self.line,
        });
    }

    fn add_error(&mut self, message: &str) {
        self.errors.push(SyntaxError {
            line: self.line,
            location: String::new(),
            message: message.to_string(),
        });
    }

    fn add_semicolon(&mut self) {
        if let Some(token) = self.tokens.last() {
            if !matches!(
                token.kind,
                Kind::LeftBrace | Kind::RightBrace | Kind::Semicolon
            ) {
                self.tokens.push(Token {
                    kind: Kind::Semicolon,
                    lexeme: ";".to_string(),
                    line: self.line,
                });
            }
        }
    }
}
