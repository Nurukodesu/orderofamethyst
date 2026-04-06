use std::vec;

use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{delimited, opt, preceded},
};

use crate::{
    ast::{Block, Statement},
    parser::{block, expression},
};

pub fn conditional(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "if", multispace1),
        (
            expression,
            delimited(
                (multispace0, '{', multispace0),
                block,
                (multispace0, '}', multispace0),
            ),
            opt(preceded(
                (multispace0, "else", multispace0),
                delimited(
                    (multispace0, '{', multispace0),
                    block,
                    (multispace0, '}', multispace0),
                ),
            )),
        ),
    )
    .map(
        |(condition, then_block, opt_else_block)| Statement::Conditional {
            condition,
            then_block,
            else_block: match opt_else_block {
                Some(b) => b,
                None => Block { statements: vec![] },
            },
        },
    )
    .parse_next(input)
}
