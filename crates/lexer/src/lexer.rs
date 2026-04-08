
use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    indent_stack: Vec<usize>,
    pending: Vec<Token>,
    position: usize,
    _source: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            chars: source.chars().peekable(),
            indent_stack: vec![0],
            pending: Vec::new(),
            position: 0,
            _source: source,
        };
        
        lexer.skip_whitespace();
        lexer
    }

    fn tokenize(&mut self) {
        while let Some(&c) = self.chars.peek() {
            match c {
                c if c.is_whitespace() && c != '\n' => self.skip_whitespace(),
                '#' => self.skip_comment(),
                '\n' => self.handle_newline(),
                '"' => self.read_string(),
                '\'' => self.read_char(),
                c if c.is_ascii_digit() => self.read_number(),
                c if c.is_alphabetic() || c == '_' => self.read_ident_or_keyword(),
                _ => self.read_symbol(),
            }
        }

        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.pending.push(Token::Dedent);
        }
        self.pending.push(Token::Eof);
        self.pending.reverse();
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() && c != '\n' {
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c != '\n' {
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn handle_newline(&mut self) {
        self.chars.next();
        self.position += 1;
        
        let mut indent = 0;
        while let Some(&c) = self.chars.peek() {
            if c == ' ' || c == '\t' {
                self.chars.next();
                self.position += 1;
                indent += 1;
            } else {
                break;
            }
        }

        let current = *self.indent_stack.last().unwrap();
        self.pending.push(Token::Newline);
        if indent > current {
            self.indent_stack.push(indent);
            self.pending.push(Token::Indent);
        } else {
            while indent < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.pending.push(Token::Dedent);
            }
        }
    }

    fn read_string(&mut self) {
        self.chars.next();
        self.position += 1;
        
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c == '"' {
                self.chars.next();
                self.position += 1;
                break;
            }
            if c == '\\' {
                self.chars.next();
                self.position += 1;
                if let Some(escaped) = self.chars.next() {
                    s.push(escaped);
                    self.position += 1;
                }
            } else {
                s.push(c);
                self.chars.next();
                self.position += 1;
            }
        }
        self.pending.push(Token::StringLiteral(s));
    }

    fn read_char(&mut self) {
        self.chars.next();
        self.position += 1;
        
        if let Some(c) = self.chars.next() {
            self.position += 1;
            if self.chars.peek() == Some(&'\'') {
                self.chars.next();
                self.position += 1;
            }
            self.pending.push(Token::CharLiteral(c));
        }
    }

    fn read_number(&mut self) {
        let mut s = String::new();
        let mut is_float = false;

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.chars.next();
                self.position += 1;
            } else if c == '.' && !is_float {
                is_float = true;
                s.push(c);
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }

        if is_float {
            if let Ok(f) = s.parse() {
                self.pending.push(Token::Float(f));
            }
        } else {
            if let Ok(i) = s.parse() {
                self.pending.push(Token::Integer(i));
            }
        }
    }

    fn read_ident_or_keyword(&mut self) {
        let mut s = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.chars.next();
                self.position += 1;
            } else {
                break;
            }
        }

        match s.as_str() {
            "let" => self.pending.push(Token::Let),
            "if" => self.pending.push(Token::If),
            "else" => self.pending.push(Token::Else),
            "for" => self.pending.push(Token::For),
            "while" => self.pending.push(Token::While),
            "fn" => self.pending.push(Token::Fn),
            "class" => self.pending.push(Token::Class),
            "return" => self.pending.push(Token::Return),
            "async" => self.pending.push(Token::Async),
            "await" => self.pending.push(Token::Await),
            "try" => self.pending.push(Token::Try),
            "except" => self.pending.push(Token::Except),
            "true" => self.pending.push(Token::True),
            "false" => self.pending.push(Token::False),
            "none" => self.pending.push(Token::None),
            "and" => self.pending.push(Token::And),
            "or" => self.pending.push(Token::Or),
            "not" => self.pending.push(Token::Not),
            "in" => self.pending.push(Token::In),
            _ => self.pending.push(Token::Ident(s)),
        }
    }

    fn read_symbol(&mut self) {
        let c = self.chars.next().unwrap();
        self.position += 1;
        
        match c {
            '+' => self.pending.push(Token::Plus),
            '-' => {
                if let Some('>') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::Arrow);
                } else {
                    self.pending.push(Token::Minus);
                }
            }
            '*' => self.pending.push(Token::Star),
            '/' => self.pending.push(Token::Slash),
            '%' => self.pending.push(Token::Percent),
            '=' => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::EqEq);
                } else if let Some('>') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::FatArrow);
                } else {
                    self.pending.push(Token::Eq);
                }
            }
            '!' => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::NotEq);
                }
            }
            '<' => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::LtEq);
                } else {
                    self.pending.push(Token::Lt);
                }
            }
            '>' => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    self.position += 1;
                    self.pending.push(Token::GtEq);
                } else {
                    self.pending.push(Token::Gt);
                }
            }
            '(' => self.pending.push(Token::LParen),
            ')' => self.pending.push(Token::RParen),
            '[' => self.pending.push(Token::LBracket),
            ']' => self.pending.push(Token::RBracket),
            '{' => self.pending.push(Token::LBrace),
            '}' => self.pending.push(Token::RBrace),
            ',' => self.pending.push(Token::Comma),
            ':' => self.pending.push(Token::Colon),
            ';' => self.pending.push(Token::Semicolon),
            '.' => self.pending.push(Token::Dot),
            _ => self.pending.push(Token::Error),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.pending.pop() {
                Some(Token::Whitespace) => continue,
                token => return token,
            }
        }
    }
}

