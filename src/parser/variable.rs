use winnow::{
    Parser, Result,
    ascii::{multispace0, multispace1},
    combinator::{preceded, separated_pair, terminated},
};

use crate::{
    ast::Statement,
    parser::{
        common::{identifier, iota_type},
        expression::expression,
    },
};

pub fn variable_declaration(input: &mut &str) -> Result<Statement> {
    preceded(
        (multispace0, "let", multispace1),
        terminated(
            separated_pair(
                (
                    identifier,
                    preceded((multispace0, ':', multispace0), iota_type),
                ),
                (multispace0, '=', multispace0),
                expression,
            ),
            (multispace0, ';', multispace0),
        ),
    )
    .map(|((name, var_type), value)| Statement::VarDecl {
        name,
        value,
        var_type,
    })
    .parse_next(input)
}
