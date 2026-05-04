use winnow::{
    ModalResult, Parser, ascii::{alpha1, line_ending, multispace0}, combinator::{alt, cut_err, delimited, opt, preceded, separated, separated_pair}, error::StrContext, token::{take_until, take_while}
};

use crate::ast::{IotaType, Parameter, Statement};

pub fn identifier(input: &mut &str) -> ModalResult<String> {
    ((
        alpha1,
        take_while(0.., ('_', 'a'..='z', 'A'..='Z', '0'..='9')),
    ),)
        .take()
        .map(|s: &str| s.to_string())
        .parse_next(input)
}

pub fn iota_type(input: &mut &str) -> ModalResult<IotaType> {
    alt((
		"Any".value(IotaType::Any),
        "Num".value(IotaType::Num),
        "Vector".value(IotaType::Vector),
        "Bool".value(IotaType::Bool),
        "Entity".value(IotaType::Entity),
        "Pattern".value(IotaType::Pattern),
        "Many".value(IotaType::Many),
        "List".value(IotaType::List),
    ))
    .parse_next(input)
}

fn parameter(input: &mut &str) -> ModalResult<Parameter> {
    separated_pair(identifier, (multispace0, ':', multispace0), iota_type)
        .map(|(id, r#type)| Parameter { id, r#type })
        .parse_next(input)
}

pub fn parameters(input: &mut &str) -> ModalResult<Vec<Parameter>> {
    cut_err(delimited(
        (multispace0, '(', multispace0),
        opt(separated(0.., parameter, (multispace0, ',', multispace0))),
        (multispace0, ')', multispace0),
    ))
	.context(StrContext::Label("parameter"))
    .map(|params| match params {
        Some(p) => p,
        None => vec![],
    })
    .parse_next(input)
}

pub fn ret_type(input: &mut &str) -> ModalResult<IotaType> {
    opt(preceded((multispace0, "->", multispace0), iota_type))
        .map(|r#type| match r#type {
            Some(t) => t,
            None => IotaType::Void,
        })
        .parse_next(input)
}

pub fn comment(input: &mut &str) -> ModalResult<Statement> {
    preceded("//", (take_until(0.., "\n"), opt(line_ending)))
		.map(|_|Statement::Empty)
        .parse_next(input)
}
