use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, multispace1},
    combinator::{cut_err, opt, preceded, separated_pair, terminated}, error::{StrContext, StrContextValue},
};

use crate::{
    ast::{IotaType, Statement},
    parser::{
        common::{identifier, iota_type},
        expression::expression,
    },
};

pub fn variable_declaration(input: &mut &str) -> ModalResult<Statement> {
    preceded(
        (multispace0, "let", multispace1),
        terminated(
            separated_pair(
                (
                    cut_err(identifier)
					.context(StrContext::Label("identifier")),
                    opt(preceded((multispace0, ':', multispace0), iota_type)),
                ),
                (multispace0, '=', multispace0),
                cut_err(expression)
				.context(StrContext::Label("declaration"))
				.context(StrContext::Expected(StrContextValue::Description("expression")))
            ),
            cut_err((multispace0, ';', multispace0)).context(StrContext::Expected(
                StrContextValue::Description("semicolon at the end of statement"),
            )),
        ),
    )
    .map(|((name, var_type), value)| Statement::VarDecl {
        name,
        value,
        var_type: match var_type {
            Some(t) => t,
            None => IotaType::Any,
        },
    })
    .parse_next(input)
}
