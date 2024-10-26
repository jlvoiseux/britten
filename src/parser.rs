use std::fmt;
use std::iter::Peekable;
use std::vec::IntoIter;

use crate::lexer::{Token, Keyword};

//
// C AST nodes
//

#[derive(Debug, Clone)]
pub enum Identifier {
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Program {
    Program(FunctionDefinition),
}

#[derive(Debug, Clone)]
pub enum FunctionDefinition {
    Function {
        name: Identifier,
        body: Statement,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
}

//
// Parser
//

type TokenIterator = Peekable<IntoIter<Token>>;

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut tokens = tokens.into_iter().peekable();
    let program = parse_program(&mut tokens)?;

    if tokens.peek().is_some() {
        return Err("Unexpected content after main function".to_string());
    }

    Ok(program)
}

fn parse_program(tokens: &mut TokenIterator) -> Result<Program, String> {
    let function = parse_function(tokens)?;
    Ok(Program::Program(function))
}

fn parse_function(tokens: &mut TokenIterator) -> Result<FunctionDefinition, String> {
    expect(tokens, &Token::Keyword(Keyword::Int))?;
    let name = parse_identifier(tokens)?;
    expect(tokens, &Token::OpenParen)?;
    expect(tokens, &Token::Keyword(Keyword::Void))?;
    expect(tokens, &Token::CloseParen)?;
    expect(tokens, &Token::OpenBrace)?;
    let body = parse_statement(tokens)?;
    expect(tokens, &Token::CloseBrace)?;
    Ok(FunctionDefinition::Function { name, body })
}

fn parse_statement(tokens: &mut TokenIterator) -> Result<Statement, String> {
    expect(tokens, &Token::Keyword(Keyword::Return))?;
    let expr = parse_expression(tokens, 0)?;
    expect(tokens, &Token::Semicolon)?;
    Ok(Statement::Return(expr))
}

fn parse_factor(tokens: &mut TokenIterator) -> Result<Expression, String> {
    match tokens.peek() {
        Some(Token::Constant(_)) => {
            if let Some(Token::Constant(value)) = tokens.next() {
                Ok(Expression::Constant(value))
            } else {
                Err("Expected constant".to_string())
            }
        },
        Some(Token::BitwiseComplement) => {
            tokens.next();
            parse_unary_operation(UnaryOperator::Complement, tokens)
        },
        Some(Token::Subtraction) => {
            tokens.next();
            parse_unary_operation(UnaryOperator::Negate, tokens)
        },
        Some(Token::OpenParen) => {
            tokens.next();
            let expr = parse_expression(tokens, 0)?;
            expect(tokens, &Token::CloseParen)?;
            Ok(expr)
        },
        _ => Err("Expected valid factor".to_string()),
    }
}

fn parse_expression(tokens: &mut TokenIterator, min_prec: i32) -> Result<Expression, String> {
    let mut left = parse_factor(tokens)?;

    while let Some(token) = tokens.peek() {
        if let Some(op) = token_to_binary_operator(token) {
            if get_precedence(&op) < min_prec {
                break;
            }
            tokens.next();
            let right = parse_expression(tokens, get_precedence(&op) + 1)?;
            left = Expression::Binary(op, Box::new(left), Box::new(right));
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_unary_operation(operator: UnaryOperator, tokens: &mut TokenIterator) -> Result<Expression, String> {
    parse_factor(tokens).map(|expr| Expression::Unary(operator, Box::new(expr)))
}

fn parse_identifier(tokens: &mut TokenIterator) -> Result<Identifier, String> {
    if let Some(Token::Identifier(name)) = tokens.next() {
        Ok(Identifier::Identifier(name))
    } else {
        Err("Expected identifier".to_string())
    }
}

fn expect(tokens: &mut TokenIterator, expected: &Token) -> Result<(), String> {
    if let Some(token) = tokens.next() {
        if &token == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, token))
        }
    } else {
        Err(format!("Expected {:?}, found EOF", expected))
    }
}

fn get_precedence(op: &BinaryOperator) -> i32 {
    match op {
        BinaryOperator::Multiply |
        BinaryOperator::Divide |
        BinaryOperator::Remainder => 50,
        BinaryOperator::Add |
        BinaryOperator::Subtract => 45,
    }
}

fn token_to_binary_operator(token: &Token) -> Option<BinaryOperator> {
    match token {
        Token::Multiplication => Some(BinaryOperator::Multiply),
        Token::Division => Some(BinaryOperator::Divide),
        Token::Remainder => Some(BinaryOperator::Remainder),
        Token::Addition => Some(BinaryOperator::Add),
        Token::Subtraction => Some(BinaryOperator::Subtract),
        _ => None,
    }
}

//
// Debug print
//

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Program::Program(func_def) => write!(f, "Program(\n  {}\n)", func_def.to_string().replace("\n", "\n  "))
        }
    }
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionDefinition::Function { name, body } => {
                write!(f, "Function(\n  name=\"{}\",\n  body={}\n)", name, body.to_string().replace("\n", "\n  "))
            }
        }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Identifier(name) => write!(f, "{}", name)
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Return(expr) => write!(f, "Return(\n  {}\n)", expr.to_string().replace("\n", "\n  "))
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(value) => write!(f, "Constant({})", value),
            Expression::Unary(op, expr) => {
                write!(f, "Unary(\n  {},\n  {}\n)", op, expr.to_string().replace("\n", "\n  "))
            },
            Expression::Binary(op, left, right) => {
                write!(f, "Binary(\n  {},\n  {},\n  {}\n)",
                       op,
                       left.to_string().replace("\n", "\n  "),
                       right.to_string().replace("\n", "\n  ")
                )
            }
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Complement => write!(f, "Complement"),
            UnaryOperator::Negate => write!(f, "Negate")
        }
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Multiply => write!(f, "Multiply"),
            BinaryOperator::Divide => write!(f, "Divide"),
            BinaryOperator::Remainder => write!(f, "Remainder"),
            BinaryOperator::Add => write!(f, "Add"),
            BinaryOperator::Subtract => write!(f, "Subtract"),
        }
    }
}