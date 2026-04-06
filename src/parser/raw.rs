use std::str::FromStr;

use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{alt, delimited, opt, preceded, repeat, separated, separated_pair, terminated},
    token::{take_till, take_while},
};

use crate::{
    ast::{AnglePath, Direction, IotaType, Sigil, SigilCall, Statement},
    parser::common::{identifier, iota_type, parameters, ret_type},
};

fn sigil_name(input: &mut &str) -> Result<String> {
    delimited(
        (multispace0, '['),
        take_till(1.., |c| c == ']'),
        (']', multispace0),
    )
    .map(|s: &str| s.to_string())
    .parse_next(input)
}

pub fn sigil_io(input: &mut &str) -> Result<Vec<IotaType>> {
    delimited(
        (multispace0, '(', multispace0),
        separated(0.., iota_type, (multispace0, ',', multispace0)),
        (multispace0, ')', multispace0),
    )
    .parse_next(input)
}

pub fn direction(input: &mut &str) -> Result<Direction> {
    alt((
        "NORTH_EAST".value(Direction::NORTHEAST),
        "EAST".value(Direction::EAST),
        "SOUTH_EAST".value(Direction::SOUTHEAST),
        "SOUTH_WEST".value(Direction::SOUTHWEST),
        "WEST".value(Direction::WEST),
        "NORTH_WEST".value(Direction::NORTHWEST),
    ))
    .parse_next(input)
}

fn angle_path(input: &mut &str) -> Result<AnglePath> {
    take_while(0.., ('w', 'e', 'd', 'q', 'a'))
        .map(|s| AnglePath::from_str(s).unwrap())
        .parse_next(input)
}

pub fn sigil_declaration(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "sigil", multispace1),
        (
            sigil_name,
            sigil_io,
            preceded((multispace0, "->", multispace0), sigil_io),
            delimited(
                (multispace0, '{', multispace0),
                separated_pair(direction, multispace1, angle_path),
                (multispace0, '}', multispace0),
            ),
        ),
    )
    .map(|(name, params, returns, (initial_direction, angle_path))| {
        Statement::SigilDecl(Sigil {
            name,
            params,
            returns,
            initial_direction,
            angle_path,
        })
    })
    .parse_next(input)
}

fn sigil_identifier(input: &mut &str) -> Result<String> {
    take_while(0.., ('a'..='z', 'A'..='Z', '\'', ' '))
        .map(|s: &str| s.to_string())
        .parse_next(input)
}

fn sigil_modifier(input: &mut &str) -> Result<Option<String>> {
    opt(preceded(
        (multispace0, ':', multispace0),
        take_while(0.., ('0'..='9', 'a'..='z', 'A'..='Z', '-', '\'', ' ')),
    ))
    .map(|modifier: Option<&str>| match modifier {
        Some(s) => Some(s.to_string()),
        None => None,
    })
    .parse_next(input)
}

fn sigil_call(input: &mut &str) -> Result<SigilCall> {
    terminated(
        (sigil_identifier, sigil_modifier),
        (multispace0, ';', multispace0),
    )
    .map(|(id, modifier)| SigilCall { id, modifier })
    .parse_next(input)
}

fn fn_body(input: &mut &str) -> Result<Vec<SigilCall>> {
    delimited(
        (multispace0, '{', multispace0),
        repeat(0.., sigil_call),
        (multispace0, '}', multispace0),
    )
    .parse_next(input)
}

pub fn raw_function_declaration(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "rawfn", multispace1),
        (identifier, parameters, ret_type, fn_body),
    )
    .map(|(name, params, ret_type, body)| Statement::RawFnDecl {
        name,
        params,
        ret_type,
        body,
    })
    .parse_next(input)
}
