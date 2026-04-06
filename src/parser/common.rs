use winnow::{
    Parser, Result,
    ascii::{alpha1, line_ending, multispace0},
    combinator::{alt, delimited, opt, preceded, separated, separated_pair},
    token::{take_until, take_while},
};

use crate::ast::{IotaType, Parameter};

pub fn identifier(input: &mut &str) -> Result<String> {
    ((
        alpha1,
        take_while(0.., ('_', 'a'..='z', 'A'..='Z', '0'..='9')),
    ),)
        .take()
        .map(|s: &str| s.to_string())
        .parse_next(input)
}

pub fn iota_type(input: &mut &str) -> Result<IotaType> {
    alt((
        "Num".value(IotaType::Num),
        "Bool".value(IotaType::Bool),
        "Vector".value(IotaType::Vector),
        "Entity".value(IotaType::Entity),
        "Pattern".value(IotaType::Pattern),
        "Many".value(IotaType::Many),
        "AnyList".value(IotaType::AnyList),
        ("List", delimited('[', iota_type, ']'))
            .map(|(_, r#type)| IotaType::List(Box::new(r#type))),
    ))
    .parse_next(input)
}

fn parameter(input: &mut &str) -> Result<Parameter> {
    separated_pair(identifier, (multispace0, ':', multispace0), iota_type)
        .map(|(id, r#type)| Parameter { id, r#type })
        .parse_next(input)
}

pub fn parameters(input: &mut &str) -> Result<Vec<Parameter>> {
    delimited(
        (multispace0, '(', multispace0),
        opt(separated(0.., parameter, (multispace0, ',', multispace0))),
        (multispace0, ')', multispace0),
    )
    .map(|params| match params {
        Some(p) => p,
        None => vec![],
    })
    .parse_next(input)
}

pub fn ret_type(input: &mut &str) -> Result<IotaType> {
    opt(preceded((multispace0, "->", multispace0), iota_type))
        .map(|r#type| match r#type {
            Some(t) => t,
            None => IotaType::Void,
        })
        .parse_next(input)
}

pub fn comment(input: &mut &str) -> Result<()> {
    preceded("//", (take_until(0.., "\n"), opt(line_ending)))
        .void()
        .parse_next(input)
}
