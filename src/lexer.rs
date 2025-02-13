use regex::Regex;
use std::str::FromStr;

//
// Tokens
//

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(i32),
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    BitwiseComplement,
    Subtraction,
    Addition,
    Multiplication,
    Division,
    Remainder
}

//
// Keywords
//

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(Keyword::Int),
            "void" => Ok(Keyword::Void),
            "return" => Ok(Keyword::Return),
            _ => Err(()),
        }
    }
}

//
// Tokenization logic
//

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut remaining = input.trim_start();

    while !remaining.is_empty() {
        match tokenize_next(remaining) {
            Ok((token, rest)) => {
                tokens.push(token);
                remaining = rest.trim_start();
            }
            Err(e) => return Err(e),
        }
    }

    Ok(tokens)
}

// Order matters - if a token matches two expressions, we should use the longest match
fn tokenize_next(input: &str) -> Result<(Token, &str), String> {
    let token_patterns: Vec<(&str, fn(&str) -> Result<Token, String>)> = vec![
        (r"^[a-zA-Z_]\w*\b", tokenize_identifier_or_keyword),
        (r"^[0-9]+\b", tokenize_constant),
        (r"^--", |_| Err("Decrement token not implemented yet".to_string())),
        (r"^\(", |_| Ok(Token::OpenParen)),
        (r"^\)", |_| Ok(Token::CloseParen)),
        (r"^\{", |_| Ok(Token::OpenBrace)),
        (r"^\}", |_| Ok(Token::CloseBrace)),
        (r"^;", |_| Ok(Token::Semicolon)),
        (r"^~", |_| Ok(Token::BitwiseComplement)),
        (r"^-", |_| Ok(Token::Subtraction)),
        (r"^\+", |_| Ok(Token::Addition)),
        (r"^\*", |_| Ok(Token::Multiplication)),
        (r"^/", |_| Ok(Token::Division)),
        (r"^%", |_| Ok(Token::Remainder)),
    ];

    for (pattern, tokenizer) in token_patterns.iter() {
        if let Some(cap) = Regex::new(pattern).map_err(|e| e.to_string())?.captures(input) {
            let matched = cap.get(0).unwrap().as_str();
            let token = tokenizer(matched)?;
            return Ok((token, &input[matched.len()..]));
        }
    }

    Err(format!("Unexpected token: {}", input))
}

fn tokenize_identifier_or_keyword(s: &str) -> Result<Token, String> {
    match Keyword::from_str(s) {
        Ok(keyword) => Ok(Token::Keyword(keyword)),
        Err(_) => Ok(Token::Identifier(s.to_string())),
    }
}

fn tokenize_constant(s: &str) -> Result<Token, String> {
    s.parse::<i32>()
        .map(Token::Constant)
        .map_err(|_| format!("Invalid constant: {}", s))
}