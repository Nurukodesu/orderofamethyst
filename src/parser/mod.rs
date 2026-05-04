use crate::{
    ast::{Block, Statement},
    parser::{
        common::comment,
        expression::expression,
        function::{fn_declaration, r#return},
        raw::{raw_function_declaration, sigil_declaration},
        variable::variable_declaration,
    },
};
use conditional::conditional;
use winnow::token::any;
use winnow::{
    ModalResult, Parser,
    ascii::multispace0,
    combinator::{alt, cut_err, dispatch, eof, fail, peek, preceded, repeat, terminated},
    error::{StrContext, StrContextValue},
};
pub mod common;
pub mod conditional;
pub mod expression;
pub mod function;
pub mod raw;
pub mod variable;

pub fn statement(input: &mut &str) -> ModalResult<Statement> {
    preceded(
        multispace0,
        dispatch! { peek(any);
            'l' => variable_declaration,
            'f' => fn_declaration,
            's' => sigil_declaration,
            'r' => alt((raw_function_declaration, r#return)),
            'i' => conditional,
            '/' => comment,
            _ => alt((
                terminated(expression,
                cut_err((multispace0, ';', multispace0)).context(StrContext::Expected(
                StrContextValue::Description("semicolon at the end of statement"),
            )),
                ).map(Statement::Expr),
                cut_err(fail)
                    .context(StrContext::Label("keyword"))
                    .context(StrContext::Expected(StrContextValue::Description(
                        "let, fn, rawfn, sigil, return",
                    )))
            )),
        },
    )
    .parse_next(input)
}

pub fn block(input: &mut &str) -> ModalResult<Block> {
    terminated(repeat(0.., statement), eof)
        .map(|statements| Block { statements })
        .parse_next(input)
}
