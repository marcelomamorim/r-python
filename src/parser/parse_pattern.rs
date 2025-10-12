use nom::{branch::alt, combinator::map, IResult};

use crate::ir::ast::Expression;
use crate::parser::parser_common::identifier;
use crate::parser::parser_expr::parse_literal_expression;

pub fn parse_literal_pattern(input: &str) -> IResult<&str, Expression> {
    parse_literal_expression(input)
}

pub fn parse_pattern_argument(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_literal_pattern,
        map(identifier, |name| Expression::Var(name.to_string())),
    ))(input)
}
