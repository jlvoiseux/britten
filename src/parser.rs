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
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

//
// Pretty-print
//

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

fn write_indented(f: &mut fmt::Formatter<'_>, level: usize, s: &str) -> fmt::Result {
    writeln!(f, "{}{}", indent(level), s)
}

trait PrettyPrint {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result;
}

impl PrettyPrint for Program {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Program::Program(func_def) => {
                writeln!(f, "Program(")?;
                func_def.pretty_print(f, level + 1)?;
                writeln!(f, "{}", indent(level))?;
                write!(f, "{})", indent(level))
            }
        }
    }
}

impl PrettyPrint for Identifier {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, _level: usize) -> fmt::Result {
        match self {
            Identifier::Identifier(name) => write!(f, "{}", name),
        }
    }
}

impl PrettyPrint for FunctionDefinition {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            FunctionDefinition::Function { name, body } => {
                writeln!(f, "{}Function(", indent(level))?;
                write_indented(f, level + 1, &format!("name=\"{}\",", name))?;
                write!(f, "{}body=", indent(level + 1))?;
                body.pretty_print(f, level + 1)?;
                writeln!(f)?;
                write!(f, "{})", indent(level))
            }
        }
    }
}

impl PrettyPrint for Statement {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Statement::Return(expr) => {
                writeln!(f, "Return(")?;
                expr.pretty_print(f, level+1)?;
                writeln!(f)?;
                write!(f, "{})", indent(level))
            }
        }
    }
}

impl PrettyPrint for Expression {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        match self {
            Expression::Constant(value) => write!(f, "{}Constant({})", indent(level), value),
            Expression::Unary(op, expr) => write!(f, "{}Unary({}({}))", indent(level), op, expr),
        }
    }
}

impl PrettyPrint for UnaryOperator {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, _level: usize) -> fmt::Result {
        match self {
            UnaryOperator::Complement => write!(f, "Complement"),
            UnaryOperator::Negate => write!(f, "Negate"),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for FunctionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {self.pretty_print(f, 0)}
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
    let expr = parse_expression(tokens)?;
    expect(tokens, &Token::Semicolon)?;
    Ok(Statement::Return(expr))
}

fn parse_expression(tokens: &mut TokenIterator) -> Result<Expression, String> {
    match tokens.next() {
        Some(Token::Constant(value)) => Ok(Expression::Constant(value)),
        Some(Token::BitwiseComplement) => parse_unary_operation(UnaryOperator::Complement, tokens),
        Some(Token::Negation) => parse_unary_operation(UnaryOperator::Negate, tokens),
        Some(Token::OpenParen) => {
            let expr = parse_expression(tokens)?;
            expect(tokens, &Token::CloseParen)?;
            Ok(expr)
        },
        _ => Err("Expected valid expression".to_string()),
    }
}

fn parse_unary_operation(operator: UnaryOperator, tokens: &mut TokenIterator) -> Result<Expression, String> {
    parse_expression(tokens).map(|expr| Expression::Unary(operator, Box::new(expr)))
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