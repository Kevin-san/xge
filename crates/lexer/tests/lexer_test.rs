use lexer::{Lexer, Token};

#[test]
fn test_basic_tokens() {
    let source = "let x: i32 = 42;";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Let));
    assert_eq!(lexer.next(), Some(Token::Ident("x".to_string())));
    assert_eq!(lexer.next(), Some(Token::Colon));
    assert_eq!(lexer.next(), Some(Token::Ident("i32".to_string())));
    assert_eq!(lexer.next(), Some(Token::Eq));
    assert_eq!(lexer.next(), Some(Token::Integer(42)));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Eof));
}

#[test]
fn test_string_literals() {
    let source = r#"let s: String = "hello world";"#;
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Let));
    assert_eq!(lexer.next(), Some(Token::Ident("s".to_string())));
    assert_eq!(lexer.next(), Some(Token::Colon));
    assert_eq!(lexer.next(), Some(Token::Ident("String".to_string())));
    assert_eq!(lexer.next(), Some(Token::Eq));
    assert_eq!(lexer.next(), Some(Token::StringLiteral("hello world".to_string())));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Eof));
}

#[test]
fn test_char_literals() {
    let source = "let c: char = 'a';";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Let));
    assert_eq!(lexer.next(), Some(Token::Ident("c".to_string())));
    assert_eq!(lexer.next(), Some(Token::Colon));
    assert_eq!(lexer.next(), Some(Token::Ident("char".to_string())));
    assert_eq!(lexer.next(), Some(Token::Eq));
    assert_eq!(lexer.next(), Some(Token::CharLiteral('a')));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Eof));
}

#[test]
fn test_float_literals() {
    let source = "let f: f64 = 3.14;";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Let));
    assert_eq!(lexer.next(), Some(Token::Ident("f".to_string())));
    assert_eq!(lexer.next(), Some(Token::Colon));
    assert_eq!(lexer.next(), Some(Token::Ident("f64".to_string())));
    assert_eq!(lexer.next(), Some(Token::Eq));
    assert_eq!(lexer.next(), Some(Token::Float(3.14)));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Eof));
}

#[test]
fn test_keywords() {
    let source = "fn main() -> i32 { return 0; }";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Fn));
    assert_eq!(lexer.next(), Some(Token::Ident("main".to_string())));
    assert_eq!(lexer.next(), Some(Token::LParen));
    assert_eq!(lexer.next(), Some(Token::RParen));
    assert_eq!(lexer.next(), Some(Token::Arrow));
    assert_eq!(lexer.next(), Some(Token::Ident("i32".to_string())));
    assert_eq!(lexer.next(), Some(Token::LBrace));
    assert_eq!(lexer.next(), Some(Token::Return));
    assert_eq!(lexer.next(), Some(Token::Integer(0)));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::RBrace));
    assert_eq!(lexer.next(), Some(Token::Eof));
}

#[test]
fn test_indentation() {
    let source = "fn main() {\n    let x = 1;\n    if x > 0 {\n        println(\"positive\");\n    }\n}";
    let mut lexer = Lexer::new(source);
    
    assert_eq!(lexer.next(), Some(Token::Fn));
    assert_eq!(lexer.next(), Some(Token::Ident("main".to_string())));
    assert_eq!(lexer.next(), Some(Token::LParen));
    assert_eq!(lexer.next(), Some(Token::RParen));
    assert_eq!(lexer.next(), Some(Token::LBrace));
    assert_eq!(lexer.next(), Some(Token::Newline));
    assert_eq!(lexer.next(), Some(Token::Indent));
    assert_eq!(lexer.next(), Some(Token::Let));
    assert_eq!(lexer.next(), Some(Token::Ident("x".to_string())));
    assert_eq!(lexer.next(), Some(Token::Eq));
    assert_eq!(lexer.next(), Some(Token::Integer(1)));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Newline));
    assert_eq!(lexer.next(), Some(Token::If));
    assert_eq!(lexer.next(), Some(Token::Ident("x".to_string())));
    assert_eq!(lexer.next(), Some(Token::Gt));
    assert_eq!(lexer.next(), Some(Token::Integer(0)));
    assert_eq!(lexer.next(), Some(Token::LBrace));
    assert_eq!(lexer.next(), Some(Token::Newline));
    assert_eq!(lexer.next(), Some(Token::Indent));
    assert_eq!(lexer.next(), Some(Token::Ident("println".to_string())));
    assert_eq!(lexer.next(), Some(Token::LParen));
    assert_eq!(lexer.next(), Some(Token::StringLiteral("positive".to_string())));
    assert_eq!(lexer.next(), Some(Token::RParen));
    assert_eq!(lexer.next(), Some(Token::Semicolon));
    assert_eq!(lexer.next(), Some(Token::Newline));
    assert_eq!(lexer.next(), Some(Token::Dedent));
    assert_eq!(lexer.next(), Some(Token::RBrace));
    assert_eq!(lexer.next(), Some(Token::Newline));
    assert_eq!(lexer.next(), Some(Token::Dedent));
    assert_eq!(lexer.next(), Some(Token::RBrace));
    assert_eq!(lexer.next(), Some(Token::Eof));
}
