use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{cut_err, delimited, preceded, repeat, terminated},
    error::{StrContext, StrContextValue},
};

use crate::{
    ast::Statement,
    parser::{
        common::{identifier, parameters, ret_type},
        expression::expression,
        statement,
    },
};

fn fn_body(input: &mut &str) -> ModalResult<Vec<Statement>> {
    cut_err(delimited(
        (multispace0, '{', multispace0),
        repeat(0.., statement),
        (multispace0, '}', multispace0),
    ))
	.context(StrContext::Label("function: missing body"))
    .parse_next(input)
}

pub fn fn_declaration(input: &mut &str) -> ModalResult<Statement> {
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

pub fn r#return(input: &mut &str) -> ModalResult<Statement> {
    preceded(
        (multispace0, "return", multispace0),
        terminated(
            cut_err(expression).context(StrContext::Expected(StrContextValue::StringLiteral(
                "expression",
            ))),
            cut_err((multispace0, ';', multispace0)).context(StrContext::Expected(
                StrContextValue::Description("semicolon at the end of statement"),
            )),
        ),
    )
    .map(Statement::Return)
    .parse_next(input)
}
