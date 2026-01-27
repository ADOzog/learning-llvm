use std::boxed;
use std::iter;

use crate::lexer::{self, *};

#[derive(Debug, PartialEq)]
pub enum ExprAST {
    Number(f64),
    Variable(String),
    Binary(char, Box<ExprAST>, Box<ExprAST>),
    Call(String, Vec<ExprAST>),
    If {
        condition: Box<ExprAST>,
        then: Box<ExprAST>,
        else_: Box<ExprAST>,
    },
    // Ill do 'for' later
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrototypeAST(pub String, pub Vec<String>);

#[derive(Debug, PartialEq)]
pub struct FunctionAST(pub PrototypeAST, pub ExprAST);

// I am just doing this to stick with the tutorial but if one wants to make a good language
// errors should be expressed better
type ParseResult<T> = Result<T, String>;

fn parse(input: &str) -> ParseResult<ExprAST> {
    let tokens: Vec<Token> = lexer::gettok(input);
    match parse_rec(tokens.into_iter(), None) {
        Some(s) => Ok(s),
        None => Err("IDK".into()),
    }
}
// This might not need to be an option
fn parse_rec(input: impl Iterator<Item = Token>, cur_ast: Option<ExprAST>) -> Option<ExprAST> {
    let next_token: Token = match input.next() {
        None => return cur_ast,
        Some(t) => t,
    };
    match next_token {
        Token::Eof => cur_ast,
        Token::Def => FunctionAST(proto_parse(till_body), parse_rec(till_end, cur_ast)),
        Token::Extern => {}
        Token::Identifier(ident) => {}
        Token::Number(num) => {}
        Token::Char(c) => {}
        Token::If => {}
        Token::Then => {}
        Token::Else => {}
        Token::For => {
            todo!()
        }
        Token::In => {
            todo!()
        }
    };
}
fn parse_def(input: &str) -> ParseResult<FunctionAST> {
    todo!()
}

// I am writing the test before I write the actual functions
// In test driven development is it better to test the more or less abstracted functions
#[cfg(test)]
//use super::
#[test]
fn parse_number() {
    assert_eq!(parse("12.34"), Ok(ExprAST::Number(12.34f64)));
}

#[test]
fn parse_variable() {
    assert_eq!(parse("foo"), Ok(ExprAST::Variable("foo".into())));
}

#[test]
fn parse_if() {
    let cond = Box::new(ExprAST::Number(1f64));
    let next = Box::new(ExprAST::Number(2f64));
    let last = Box::new(ExprAST::Number(3f64));

    assert_eq!(
        parse("if 1 then 2 else 3"),
        Ok(ExprAST::If {
            condition: cond,
            then: next,
            else_: last
        })
    );

    let cond = Box::new(ExprAST::Call("foo".into(), vec![]));
    let next = Box::new(ExprAST::Call("bar".into(), vec![ExprAST::Number(2f64)]));
    let last = Box::new(ExprAST::Call(
        "toast".into(),
        vec![ExprAST::Number(3.14f64)],
    ));

    assert_eq!(
        parse("if foo() then bar(2) else toast(3.14)"),
        Ok(ExprAST::If {
            condition: cond,
            then: next,
            else_: last
        })
    );
}

#[test]
fn parse_binary_add_over_sub() {
    let binexpr_ab = ExprAST::Binary(
        '+',
        Box::new(ExprAST::Variable("a".into())),
        Box::new(ExprAST::Variable("b".into())),
    );

    let binexpr_abc = ExprAST::Binary(
        '-',
        Box::new(binexpr_ab),
        Box::new(ExprAST::Variable("c".into())),
    );
    assert_eq!(parse("a + b - c"), Ok(binexpr_abc))
}

#[test]
fn parse_binary_mul_over_add() {
    let binexpr_ab = ExprAST::Binary(
        '*',
        Box::new(ExprAST::Variable("b".into())),
        Box::new(ExprAST::Variable("c".into())),
    );

    let binexpr_abc = ExprAST::Binary(
        '+',
        Box::new(binexpr_ab),
        Box::new(ExprAST::Variable("a".into())),
    );
    assert_eq!(parse("a + b * c"), Ok(binexpr_abc))
}

// what does this do
#[test]
fn parse_function() {
    let fun_body = ExprAST::Binary(
        '+',
        Box::new(ExprAST::Variable("a".into())),
        Box::new(ExprAST::Variable("b".into())),
    );

    assert_eq!(
        parse_def("def add(a,b) a + b"),
        Ok(FunctionAST(
            PrototypeAST("add".into(), vec!["a".into(), "b".into()]),
            fun_body
        ))
    );
}
