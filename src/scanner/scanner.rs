use crate::helper::helper::Error;
use std::collections::HashMap;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u128,
    keywords: HashMap<&'static str, TokenType>,
    errors: Vec<Error>,
}

#[derive(Clone, PartialEq, Debug)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(PartialEq, Debug)]
enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u128,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        return Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
            errors: Vec::new(),
        };
    }

    pub fn scan_tokens(&mut self) -> Vec<Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });

        return self.errors.clone();
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),

            '!' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            '=' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            '<' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            '>' => {
                let matches_eq = self.matches('=');
                self.add_token(if matches_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }

            '/' => {
                let matches_commnet = self.matches('/');
                if matches_commnet {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            ' ' | '\r' | 't' => (),
            '\n' => self.line += 1,

            '"' => self.string(),
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.errors
                        .push(Error::new(self.line, "Unexpected Character".to_string()));
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let result = self.source[self.current];
        self.current += 1;
        return result;
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_literal(token, None);
    }

    fn add_token_literal(&mut self, token: TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type: token,
            lexeme: text.iter().cloned().collect(),
            literal,
            line: self.line,
        });
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source[self.current];
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        return self.source[self.current + 1];
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        self.advance();

        let string = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_literal(TokenType::String, Some(Literal::String(string)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        let number = value.parse::<f64>().unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(number)));
    }

    fn is_alpha(&self, c: char) -> bool {
        return c.is_alphabetic() || c == '_';
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let value = String::from_iter(&self.source[self.start..self.current]);

        if let Some(keyword) = self.keywords.get::<str>(&*value) {
            self.add_token_literal(keyword.clone(), Some(Literal::Identifier(value)))
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_tokens_with_error_test() {
        let source = "var $test = 1234".to_string();

        let mut scanner = Scanner::new(&source);
        let errors = scanner.scan_tokens();

        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn scan_tokens_test() {
        let source = "
        var test = 1234;
        if (test <= 10) 
        {
            test = test + 10;
        }
        "
        .to_string();

        let expected = [
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::SemiColon,
            TokenType::If,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::LessEqual,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::LeftBrace, 
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::Plus,
            TokenType::Number,
            TokenType::SemiColon,
            TokenType::RightBrace,
            TokenType::EOF,
        ];
        let mut scanner = Scanner::new(&source);

        assert!(!scanner.is_at_end());

        let errors = scanner.scan_tokens();

        assert_eq!(errors.len(), 0);
        // 19 tokens in source string + 1 for EOF
        assert_eq!(scanner.tokens.len(), expected.len());
        // 2 extra lines for empty first/last line in string
        assert_eq!(scanner.line, 7);

        let scanner_tokens_mapped: Vec<TokenType> = scanner
            .tokens
            .iter()
            .map(|t| t.token_type.clone())
            .collect();

        assert_eq!(scanner_tokens_mapped, expected);

        assert!(scanner.is_at_end());
    }

    #[test]
    fn peek_and_advance_test() {
        let source = "var test = 1234".to_string();
        let mut scanner = Scanner::new(&source);
        let source_bytes = source.as_bytes();

        for (index, char) in source.chars().enumerate() {
            if index < source.len() - 1 {
                let b: u8 = source_bytes[index + 1];
                assert_eq!(scanner.peek_next(), b as char);
            }

            assert_eq!(scanner.peek(), char);
            scanner.advance();
        }
    }

    #[test]
    fn create_string_test() {
        let source = "\"Hello \n World\"".to_string();
        let mut scanner = Scanner::new(&source);

        //drop the \" first as that's how scan token would handle it
        scanner.advance();
        scanner.string();

        assert_eq!(scanner.tokens.len(), 1);

        let string = &scanner.tokens[0];
        assert_eq!(string.token_type, TokenType::String);
        assert_eq!(
            string.literal,
            Some(Literal::String("Hello \n World".to_string()))
        );
        assert_eq!(string.line, 2);
    }

    #[test]
    fn create_double_test() {
        let source = "11.234".to_string();
        let mut scanner = Scanner::new(&source);
        scanner.number();

        assert_eq!(scanner.tokens.len(), 1);

        let number = &scanner.tokens[0];
        assert_eq!(number.token_type, TokenType::Number);
        assert_eq!(number.literal, Some(Literal::Number(11.234)));
        assert_eq!(number.line, 1);
    }

    #[test]
    fn create_number_test() {
        let source = "10".to_string();
        let mut scanner = Scanner::new(&source);
        scanner.number();

        assert_eq!(scanner.tokens.len(), 1);

        let number = &scanner.tokens[0];
        assert_eq!(number.token_type, TokenType::Number);
        assert_eq!(number.literal, Some(Literal::Number(10.0)));
        assert_eq!(number.line, 1);
    }

    #[test]
    fn create_keyword_test() {
        let source = "var".to_string();
        let mut scanner = Scanner::new(&source);
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);

        let keyword = &scanner.tokens[0];
        assert_eq!(keyword.token_type, TokenType::Var);
        assert_eq!(
            keyword.literal,
            Some(Literal::Identifier("var".to_string()))
        );
        assert_eq!(keyword.line, 1);
    }

    #[test]
    fn create_identifier_test() {
        let source = "hello".to_string();
        let mut scanner = Scanner::new(&source);
        scanner.identifier();

        assert_eq!(scanner.tokens.len(), 1);

        let identifier = &scanner.tokens[0];
        assert_eq!(identifier.token_type, TokenType::Identifier);
        assert_eq!(identifier.literal, None);
        assert_eq!(identifier.line, 1);
    }

    #[test]
    fn is_at_end_test() {
        let source = "Hello world".to_string();
        let mut scanner = Scanner::new(&source);

        scanner.current = source.len();
        assert!(scanner.is_at_end());
    }

    #[test]
    fn is_not_at_end() {
        let source = "Hello world".to_string();
        let mut scanner = Scanner::new(&source);
        assert!(!scanner.is_at_end());

        scanner.current = source.len() - 1;
        assert!(!scanner.is_at_end());
    }
}
