use crate::{
    ast::{Block, Statement},
    parser::{
        common::comment, expression::expression, function::{fn_declaration, r#return}, raw::{raw_function_declaration, sigil_declaration}, variable::variable_declaration
    },
};
use conditional::conditional;
use winnow::{
    Parser, Result,
    combinator::{alt, opt, preceded, repeat, terminated},
};

pub mod common;
pub mod conditional;
pub mod expression;
pub mod function;
pub mod raw;
pub mod variable;

pub fn statement(input: &mut &str) -> Result<Statement> {
    preceded(
        opt(comment),
        alt((
            variable_declaration,
            fn_declaration,
            sigil_declaration,
            raw_function_declaration,
            conditional,
            r#return,
            terminated(expression, ';').map(Statement::Expr),
        )),
    )
    .parse_next(input)
}

pub fn block(input: &mut &str) -> Result<Block> {
    repeat(0.., statement)
        .map(|statements| Block { statements })
        .parse_next(input)
}
