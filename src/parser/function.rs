use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{delimited, preceded, repeat, terminated},
};

use crate::{
    ast::Statement,
    parser::{
        common::{identifier, parameters, ret_type},
        expression::expression,
        statement,
    },
};

fn fn_body(input: &mut &str) -> Result<Vec<Statement>> {
    delimited(
        (multispace0, '{', multispace0),
        repeat(0.., statement),
        (multispace0, '}', multispace0),
    )
    .parse_next(input)
}

pub fn fn_declaration(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "fn", multispace1),
        (identifier, parameters, ret_type, fn_body),
    )
    .map(|(name, params, ret_type, body)| Statement::FnDecl {
        name,
        params,
        ret_type,
        body,
    })
    .parse_next(input)
}

pub fn r#return(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "return", multispace0),
        terminated(expression, (multispace0, ';', multispace0)),
    )
    .map(Statement::Return)
    .parse_next(input)
}
