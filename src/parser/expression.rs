use crate::{
    ast::{Expression, Op},
    parser::{common::identifier, raw::sigil_call},
};
use winnow::{
    Parser, Result,
    ascii::{float, multispace0},
    combinator::{alt, delimited, opt, separated, separated_foldl1},
};

pub fn expression(input: &mut &str) -> Result<Expression> {
    fourth_level.parse_next(input)
}

// Paranthese, Literal
fn first_level(input: &mut &str) -> Result<Expression> {
    alt((
        delimited(
            (multispace0, '(', multispace0),
            expression,
            (multispace0, ')', multispace0),
        ),
        boolean,
        float.map(Expression::Num),
		pattern,
        vector,
        list,
        atom_identifier,
    ))
    .parse_next(input)
}

// Multiplication, Division, Remainder
fn second_level(input: &mut &str) -> Result<Expression> {
    separated_foldl1(
        first_level,
        (multispace0, alt(('*', '/', '%')), multispace0),
        |lhs, (_, op_sign, _), rhs| {
            let op = match op_sign {
                '*' => Op::Mul,
                '/' => Op::Div,
                '%' => Op::Mod,
                _ => unreachable!(),
            };
            Expression::BinaryOps {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            }
        },
    )
    .parse_next(input)
}

// Addition, Subtraction
fn third_level(input: &mut &str) -> Result<Expression> {
    separated_foldl1(
        second_level,
        (multispace0, alt(('+', '-')), multispace0),
        |lhs, (_, op_sign, _), rhs| {
            let op = if op_sign == '+' { Op::Add } else { Op::Sub };
            Expression::BinaryOps {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            }
        },
    )
    .parse_next(input)
}

// Conditions
fn fourth_level(input: &mut &str) -> Result<Expression> {
    separated_foldl1(
        third_level,
        (
            multispace0,
            alt(("==", "!=", ">", ">=", "<", "<=")),
            multispace0,
        ),
        |lhs, (_, op_sign, _), rhs| {
            let op = match op_sign {
                "==" => Op::Equal,
                "!=" => Op::NotEqual,
                ">" => Op::GreaterThan,
                ">=" => Op::GreaterEqual,
                "<" => Op::LessThan,
                "<=" => Op::LessEqual,
                _ => unreachable!(),
            };
            Expression::BinaryOps {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
            }
        },
    )
    .parse_next(input)
}

fn boolean(input: &mut &str) -> Result<Expression> {
    alt((
        "true".value(Expression::Bool(true)),
        "false".value(Expression::Bool(false)),
    ))
    .parse_next(input)
}

fn atom_identifier(input: &mut &str) -> Result<Expression> {
    (identifier, opt(arguments))
        .map(|(id, p)| match p {
            Some(p) => Expression::Call(id, p),
            None => Expression::Id(id),
        })
        .parse_next(input)
}

pub fn arguments(input: &mut &str) -> Result<Vec<Expression>> {
    delimited(
        (multispace0, '(', multispace0),
        opt(separated(0.., expression, (multispace0, ',', multispace0))),
        (multispace0, ')', multispace0),
    )
    .map(|option| match option {
        Some(p) => p,
        None => vec![],
    })
    .parse_next(input)
}

pub fn vector(input: &mut &str) -> Result<Expression> {
    delimited(
        (multispace0, '(', multispace0),
        (
            float,
            (multispace0, ',', multispace0),
            float,
            (multispace0, ',', multispace0),
            float,
        ),
        (multispace0, ')', multispace0),
    )
    .map(|(x, _, y, _, z)| Expression::Vector(x, y, z))
    .parse_next(input)
}

pub fn list(input: &mut &str) -> Result<Expression> {
    delimited(
        (multispace0, '[', multispace0),
        separated(0.., expression, (multispace0, ',', multispace0)),
        (multispace0, ']', multispace0),
    )
    .map(Expression::List)
    .parse_next(input)
}

pub fn pattern(input: &mut &str) -> Result<Expression> {
    delimited(
        (multispace0, '['),
        sigil_call,
        (']', multispace0),
    )
	.map(|sigil| Expression::Pattern(sigil))
    .parse_next(input)
}