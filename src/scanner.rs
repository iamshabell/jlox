use std::string::String;
use std::collections::HashMap;
use once_cell::sync::Lazy;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and",    And);
    m.insert("class",  Class);
    m.insert("else",   Else);
    m.insert("false",  False);
    m.insert("for",    For);
    m.insert("fun",    Fun);
    m.insert("if",     If);
    m.insert("nil",    Nil);
    m.insert("or",     Or);
    m.insert("print",  Print);
    m.insert("return", Print);
    m.insert("super",  Super);
    m.insert("this",   This);
    m.insert("true",   True);
    m.insert("var",    Var);
    m.insert("while",  While);
    m
});

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if !errors.is_empty() {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push('\n');
            }
            return Err(joined);
        }

        Ok(self.tokens.clone())
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.in_advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = if self.char_match('=') {
                    BangEqual
                } else {
                    Bang
                };

                self.add_token(token);
            }
            '=' => {
                let token = if self.char_match('=') {
                    EqualEqual
                } else {
                    Equal
                };

                self.add_token(token);
            }
            '<' => {
                let token = if self.char_match('=') {
                    LessEqual
                } else {
                    Less
                };

                self.add_token(token);
            }
            '>' => {
                let token = if self.char_match('=') {
                    GreaterEqual
                } else {
                    Greater
                };

                self.add_token(token);
            }
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.in_advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string()?,

            char => {
                if is_digit(char) {
                    let _ = self.number();
                } else if is_alpha(char){
		    self.identifier();
		} else {
                    return Err(format!(
                        "Unexpected characater at line {}: {}",
                        self.line, c
                    ));
                }
            }
        }

        Ok(())
    }

    fn identifier(&mut self) {
	while is_alpha_numeric(self.peek()) {
	    self.in_advance();
	}

	let text = &self.source[self.start..self.current];
	let token_type = KEYWORDS.get(text).copied().unwrap_or(Identifier);

	self.add_token(token_type)
    }

    fn number(&mut self) -> Result<(), String> {
        while is_digit(self.peek()) {
            self.in_advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.in_advance();

            while is_digit(self.peek()) {
                self.in_advance();
            }
        }

	let substring = &self.source[self.start..self.current];
	let value = substring.parse::<f64>();
	match value {
	    Ok(value) => self.add_token_alt(Number, Some(FloatValue(value))),
	    Err(_) => return Err(format!("Could not parse number: {}", substring )),
	}

	Ok(())
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.in_advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        let value = &self.source[self.start + 1..self.current];

        self.add_token_alt(StringLit, Some(StringValue(value.to_string())));

        Ok(())
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn in_advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;

        c as char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_alt(token_type, None);
    }

    fn add_token_alt(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let mut text = "".to_string();
        let _ = self.source[self.start..self.current]
            .chars()
            .map(|ch| text.push(ch));

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line_number: self.line,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // Multiple chars
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    StringLit,
    Number,

    // Keywords
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

    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    IdentifierValue(String),
}

use LiteralValue::*;
use TokenType::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LiteralValue>,
        line_number: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn _to_string(&self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

fn is_digit(c: char) -> bool {
    c as u8 >= b'0' && c as u8 <= b'9'
}

fn is_alpha(c: char) -> bool {
    let char = c as u8;
    char >= b'a' && char <= b'z' || char >= b'A' && char <= b'Z' || char == b'_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_char_tokens() {
        let source = "((  )) {}";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, LeftBrace);
        assert_eq!(scanner.tokens[5].token_type, RightBrace);
        assert_eq!(scanner.tokens[6].token_type, Eof);
    }

    #[test]
    fn test_operators_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handle_string_literals() {
        let source = r#""YAY""#;
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "YAY"),
            _ => panic!("Unrecognized literal"),
        }
    }

    #[test]
    fn handle_terminated_string_literals() {
        let source = r#""YAY"#;
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        match result {
            Err(_) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn handle_multiline_string_literals() {
        let source = "\"YAY\ndef\"";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, StringLit);

        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "YAY\ndef"),
            _ => panic!("Unrecognized literal"),
        }
    }

    #[test]
    fn handle_number_literals() {
        let source = "123.456\n321.0\n5";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 4);        
	for i in 0..3 {
	   assert_eq!(scanner.tokens[i].token_type, Number);
	}
        match scanner.tokens[0].literal {
            Some(FloatValue(val)) => assert_eq!(val, 123.456),
            _ => panic!("Unrecognized literal"),
        }
        match scanner.tokens[1].literal {
	    Some(FloatValue(val)) => assert_eq!(val, 321.0),
            _ => panic!("Unrecognized literal"),
        }
        match scanner.tokens[2].literal {
	    Some(FloatValue(val)) => assert_eq!(val, 5.0),
            _ => panic!("Unrecognized literal"),
        }

    }


    #[test]
    fn handle_identifiers() {
        let source = "init_var = 28;";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5);
	assert_eq!(scanner.tokens[0].token_type, Identifier);
	assert_eq!(scanner.tokens[1].token_type, Equal);
	assert_eq!(scanner.tokens[2].token_type, Number);
	assert_eq!(scanner.tokens[3].token_type, Semicolon);
	assert_eq!(scanner.tokens[4].token_type, Eof);

    }
    #[test]
    fn handle_keywords() {
        let source = "var var_name = 8; \nwhile true { print 3 };";
        let mut scanner = Scanner::new(source);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 13);
	assert_eq!(scanner.tokens[0].token_type, Var);
	assert_eq!(scanner.tokens[1].token_type, Identifier);
	assert_eq!(scanner.tokens[2].token_type, Equal);
	assert_eq!(scanner.tokens[3].token_type, Number);
	assert_eq!(scanner.tokens[4].token_type, Semicolon);
	assert_eq!(scanner.tokens[5].token_type, While);
	assert_eq!(scanner.tokens[6].token_type, True);
	assert_eq!(scanner.tokens[7].token_type, LeftBrace);
	assert_eq!(scanner.tokens[8].token_type, Print);
	assert_eq!(scanner.tokens[9].token_type, Number);
	assert_eq!(scanner.tokens[10].token_type, RightBrace);
	assert_eq!(scanner.tokens[11].token_type, Semicolon);
	assert_eq!(scanner.tokens[12].token_type, Eof);
    }
}
