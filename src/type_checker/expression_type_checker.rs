use crate::environment::environment::Environment;
use crate::ir::ast::{Expression, Name, Type};
use std::sync::Arc;

type ErrorMessage = String;

pub fn check_expr(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    match exp {
        Expression::CTrue => Ok(Type::TBool),
        Expression::CFalse => Ok(Type::TBool),
        Expression::CVoid => Ok(Type::TVoid),
        Expression::CInt(_) => Ok(Type::TInteger),
        Expression::CReal(_) => Ok(Type::TReal),
        Expression::CString(_) => Ok(Type::TString),
        Expression::Add(l, r) => check_bin_arithmetic_expression(l, r, env),
        Expression::Sub(l, r) => check_bin_arithmetic_expression(l, r, env),
        Expression::Mul(l, r) => check_bin_arithmetic_expression(l, r, env),
        Expression::Div(l, r) => check_bin_arithmetic_expression(l, r, env),
        Expression::And(l, r) => check_bin_boolean_expression(l, r, env),
        Expression::Or(l, r) => check_bin_boolean_expression(l, r, env),
        Expression::Not(e) => check_not_expression(e, env),
        Expression::EQ(l, r) => check_bin_relational_expression(l, r, env),
        Expression::GT(l, r) => check_bin_relational_expression(l, r, env),
        Expression::LT(l, r) => check_bin_relational_expression(l, r, env),
        Expression::GTE(l, r) => check_bin_relational_expression(l, r, env),
        Expression::LTE(l, r) => check_bin_relational_expression(l, r, env),
        Expression::Var(name) => check_var_name(name, env),
        Expression::COk(e) => check_result_ok(e, env),
        Expression::CErr(e) => check_result_err(e, env),
        Expression::CJust(e) => check_maybe_just(e, env),
        Expression::CNothing => Ok(Type::TMaybe(Box::new(Type::TAny))),
        Expression::IsError(e) => check_iserror_type(e, env),
        Expression::IsNothing(e) => check_isnothing_type(e, env),
        Expression::Unwrap(e) => check_unwrap_type(e, env),
        Expression::Propagate(e) => check_propagate_type(e, env),
        Expression::ListValue(elements) => check_list_value(elements.as_slice(), env),
        Expression::Constructor(name, args) => check_adt_constructor(name.clone(), args, env),

        _ => Err("not implemented yet.".to_string()),
    }
}

fn check_var_name(name: &Name, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    match env.lookup(name) {
        Some((_, t)) => Ok(t.clone()),
        None => Err(format!(
            "[Name Error] Variable '{}' is not defined in the current scope.",
            name
        )),
    }
}

fn check_bin_arithmetic_expression(
    left: &Expression,
    right: &Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type.clone(), right_type.clone()) {
        (Type::TInteger, Type::TInteger) => Ok(Type::TInteger),
        (Type::TInteger, Type::TReal) => Ok(Type::TReal),
        (Type::TReal, Type::TInteger) => Ok(Type::TReal),
        (Type::TReal, Type::TReal) => Ok(Type::TReal),
        (Type::TInteger, wrong_type) => Err(format!(
            "[Type Error] Both operands must be of numeric types. Right operand is {:?}",
            wrong_type
        )),
        (wrong_type, Type::TInteger) => Err(format!(
            "[Type Error] Both operands must be of numeric types. Left operand is {:?}",
            wrong_type
        )),
        (Type::TReal, wrong_type) => Err(format!(
            "[Type Error] Both operands must be of numeric types. Right operand is {:?}",
            wrong_type
        )),
        (wrong_type, Type::TReal) => Err(format!(
            "[Type Error] Both operands must be of numeric types. Left operand is {:?}",
            wrong_type
        )),
        (lt, rt) => Err(format!(
            "[Type Error] Both operands must be of numeric types, but neither is. Left operand is {:?}, right operand is {:?}",
            lt, rt
        )),
    }
}

fn check_bin_boolean_expression(
    left: &Expression,
    right: &Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type.clone(), right_type.clone()) {
        (Type::TBool, Type::TBool) => Ok(Type::TBool),
        (Type::TBool, wrong_type) => Err(format!(
            "[Type Error] Both operands must be evaluated to boolean values. Right operand is {:?}",
            wrong_type
        )),
        (wrong_type, Type::TBool) => Err(format!(
            "[Type Error] Both operands must be evaluated to boolean values. Left operand is {:?}",
            wrong_type
        )),
        (lt, rt) => Err(format!(
            "[Type Error] Both operands must be evaluated to boolean values. Left operand is {:?}, right operand is {:?}",
            lt, rt
        )),
    }
}

fn check_not_expression(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TBool => Ok(Type::TBool),
        wrong_type => Err(format!(
            "[Type Error] Expected boolean value, but got {:?}",
            wrong_type
        )),
    }
}

fn check_bin_relational_expression(
    left: &Expression,
    right: &Expression,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let left_type = check_expr(left, env)?;
    let right_type = check_expr(right, env)?;

    match (left_type.clone(), right_type.clone()) {
        (Type::TInteger, Type::TInteger)
        | (Type::TInteger, Type::TReal)
        | (Type::TReal, Type::TInteger)
        | (Type::TReal, Type::TReal) => Ok(Type::TBool),
        (Type::TInteger, wrong_type) => Err(format!(
            "[Type Error] Both operands must be evaluated to numeric values. Right operand is {:?}",
            wrong_type
        )),
        (wrong_type, Type::TInteger) => Err(format!(
            "[Type Error] Both operands must be evaluated to numeric values. Left operand is {:?}",
            wrong_type
        )),
        (Type::TReal, wrong_type) => Err(format!(
            "[Type Error] Both operands must be evaluated to numeric values. Right operand is {:?}",
            wrong_type
        )),
        (wrong_type, Type::TReal) => Err(format!(
            "[Type Error] Both operands must be evaluated to numeric values. Left operand is {:?}",
            wrong_type
        )),
        (lt, rt) => Err(format!(
            "[Type Error] Both operands must be evaluated to numeric values. Left operand is {:?}, right operand is {:?}",
            lt, rt
        )),
    }
}

fn check_result_ok(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    Ok(Type::TResult(Box::new(exp_type), Box::new(Type::TAny)))
}

fn check_result_err(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    Ok(Type::TResult(Box::new(Type::TAny), Box::new(exp_type)))
}

fn check_unwrap_type(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, _) => Ok(*tl),
        wrong_type => Err(format!(
            "[Type Error] Expected a maybe or result type value, but got {:?}",
            wrong_type
        )),
    }
}

fn check_propagate_type(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(t) => Ok(*t),
        Type::TResult(tl, tr) => Ok(Type::TUnion(vec![*tl, *tr])),
        wrong_type => Err(format!(
            "[Type Error] Expected a maybe or result type value, but got {:?}",
            wrong_type
        )),
    }
}

fn check_maybe_just(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;
    Ok(Type::TMaybe(Box::new(exp_type)))
}

fn check_iserror_type(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TResult(_, _) => Ok(Type::TBool),
        wrong_type => Err(format!(
            "[Type Error] Expected a result type value, but got {:?}",
            wrong_type
        )),
    }
}

fn check_isnothing_type(exp: &Expression, env: &Environment<Type>) -> Result<Type, ErrorMessage> {
    let exp_type = check_expr(exp, env)?;

    match exp_type {
        Type::TMaybe(_) => Ok(Type::TBool),
        wrong_type => Err(format!(
            "[Type Error] Expected a maybe type value, but got {:?}",
            wrong_type
        )),
    }
}

fn check_list_value(
    elements: &[Expression],
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    if elements.is_empty() {
        return Ok(Type::TList(Box::new(Type::TAny)));
    }

    let expected_type = check_expr(&elements[0], env)?;
    for element in elements.iter().skip(1) {
        let element_type = check_expr(element, env)?;
        if element_type != expected_type {
            return Err(format!(
                "[Type Error] List elements must have the same type. Expected '{:?}', found '{:?}'.",
                expected_type, element_type
            ));
        }
    }

    Ok(Type::TList(Box::new(expected_type)))
}

fn check_adt_constructor(
    name: Name,
    args: &Vec<Box<Expression>>,
    env: &Environment<Type>,
) -> Result<Type, ErrorMessage> {
    let found = env
        .stack
        .iter()
        .find_map(|scope| {
            scope.adts.iter().find_map(|(adt_name, constructors)| {
                constructors.get(&name).map(|constructor| {
                    (
                        adt_name.clone(),
                        constructor.clone(),
                        Arc::clone(constructors),
                    )
                })
            })
        })
        .or_else(|| {
            env.globals
                .adts
                .iter()
                .find_map(|(adt_name, constructors)| {
                    constructors.get(&name).map(|constructor| {
                        (
                            adt_name.clone(),
                            constructor.clone(),
                            Arc::clone(constructors),
                        )
                    })
                })
        });

    match found {
        Some((adt_type_name, constructor, constructors)) => {
            if args.len() != constructor.len() {
                return Err(format!(
                    "[Type Error] Constructor '{}' expects {} arguments, but {} were provided.",
                    name,
                    constructor.len(),
                    args.len()
                ));
            }

            for (arg, expected_type) in args.iter().zip(constructor.iter()) {
                let arg_type = check_expr(&*arg, env)?;
                if arg_type != *expected_type {
                    return Err(format!(
                        "[Type Error] Argument type mismatch in constructor '{}'. Expected '{:?}', found '{:?}'.",
                        name, expected_type, arg_type
                    ));
                }
            }

            Ok(Type::TAlgebraicData(adt_type_name, (*constructors).clone()))
        }
        None => Err(format!(
            "[Name Error] Constructor '{}' is not defined in any loaded Algebraic Data Type (ADT).",
            name
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::environment::Environment;
    use crate::ir::ast::Expression::*;
    use crate::ir::ast::Type;
    use crate::ir::ast::Type::*;
    use std::collections::HashMap;

    #[test]
    fn check_constant() {
        let env = Environment::new();
        let c10 = CInt(10);

        assert_eq!(check_expr(&c10, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_integers() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(&add, &env), Ok(TInteger));
    }

    #[test]
    fn check_add_reals() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(&add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_real_and_integer() {
        let env = Environment::new();

        let c10 = CInt(10);
        let c20 = CReal(20.3);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(&add, &env), Ok(TReal));
    }

    #[test]
    fn check_add_integer_and_real() {
        let env = Environment::new();

        let c10 = CReal(10.5);
        let c20 = CInt(20);
        let add = Add(Box::new(c10), Box::new(c20));

        assert_eq!(check_expr(&add, &env), Ok(TReal));
    }

    #[test]
    fn check_type_error_arithmetic_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CFalse;
        let e3 = Add(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(&e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_not_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = Not(Box::new(e1));

        assert!(
            matches!(check_expr(&e2, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_and_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = And(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(&e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_type_error_or_expression() {
        let env = Environment::new();

        let e1 = CInt(10);
        let e2 = CTrue;
        let e3 = Or(Box::new(e1), Box::new(e2));

        assert!(
            matches!(check_expr(&e3, &env), Err(_)),
            "Expecting a type error."
        );
    }

    #[test]
    fn check_ok_result() {
        let env = Environment::new();

        let e1 = CReal(10.0);
        let e2 = COk(Box::new(e1));

        assert_eq!(
            check_expr(&e2, &env),
            Ok(TResult(Box::new(TReal), Box::new(TAny)))
        );
    }

    #[test]
    fn check_err_result() {
        let env = Environment::new();

        let e1 = CInt(1);
        let e2 = CErr(Box::new(e1));

        assert_eq!(
            check_expr(&e2, &env),
            Ok(TResult(Box::new(TAny), Box::new(TInteger)))
        );
    }

    #[test]
    fn check_just_integer() {
        let env = Environment::new();

        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));

        assert_eq!(check_expr(&e2, &env), Ok(TMaybe(Box::new(TInteger))))
    }

    #[test]
    fn check_is_error_result_positive() {
        let env = Environment::new();

        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = IsError(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_error_result_error() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = IsError(Box::new(e1));

        assert!(
            matches!(check_expr(&e2, &env), Err(_)),
            "Expecting a result type value."
        );
    }

    #[test]
    fn check_nothing() {
        let env = Environment::new();

        assert_eq!(check_expr(&CNothing, &env), Ok(TMaybe(Box::new(TAny))));
    }

    #[test]
    fn check_is_nothing_on_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = IsNothing(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TBool));
    }

    #[test]
    fn check_is_nothing_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = IsNothing(Box::new(e1));

        assert!(
            matches!(check_expr(&e2, &env), Err(_)),
            "Expecting a maybe type value."
        );
    }

    #[test]
    fn check_unwrap_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_unwrap_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Unwrap(Box::new(e1));

        assert!(
            matches!(check_expr(&e2, &env), Err(_)),
            "Expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_unwrap_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Unwrap(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TBool));
    }

    #[test]
    fn check_propagate_maybe() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = CJust(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TInteger));
    }

    #[test]
    fn check_propagate_maybe_type_error() {
        let env = Environment::new();
        let e1 = CInt(5);
        let e2 = Propagate(Box::new(e1));

        assert!(
            matches!(check_expr(&e2, &env), Err(_)),
            "Expecting a maybe or result type value."
        );
    }

    #[test]
    fn check_propagate_result() {
        let env = Environment::new();
        let e1 = CTrue;
        let e2 = COk(Box::new(e1));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TUnion(vec![TBool, TAny])));
    }

    #[test]
    fn check_propagate_result_with_specific_error_type() {
        let env = Environment::new();

        let e2 = CErr(Box::new(CString("error".to_string())));
        let e3 = Propagate(Box::new(e2));

        assert_eq!(check_expr(&e3, &env), Ok(TUnion(vec![TAny, TString])));
    }

    #[test]
    fn test_undefined_variable() {
        let env = Environment::new();
        let exp = Expression::Var("x".to_string());

        assert!(check_expr(&exp, &env).is_err());
    }

    #[test]
    fn test_defined_variable() {
        let mut env = Environment::new();
        env.map_variable("x".to_string(), true, Type::TInteger);
        let exp = Expression::Var("x".to_string());

        assert_eq!(check_expr(&exp, &env), Ok(Type::TInteger));
    }

    #[test]
    fn test_adt_constructor_valid() {
        let mut env = Environment::new();
        let figure_type = HashMap::from([
            ("Circle".to_string(), vec![Type::TInteger]),
            (
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ]);
        env.map_adt("Figure".to_string(), figure_type);

        let circle = Constructor("Circle".to_string(), vec![Box::new(CInt(5))]);
        let result = check_expr(&circle, &env);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adt_constructor_wrong_args() {
        let mut env = Environment::new();
        let figure_type = HashMap::from([
            ("Circle".to_string(), vec![Type::TInteger]),
            (
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ]);
        env.map_adt("Figure".to_string(), figure_type);

        let circle = Constructor(
            "Circle".to_string(),
            vec![Box::new(CString("invalid".to_string()))],
        );
        let result = check_expr(&circle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_wrong_count() {
        let mut env = Environment::new();
        let figure_type = HashMap::from([
            ("Circle".to_string(), vec![Type::TInteger]),
            (
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ]);
        env.map_adt("Figure".to_string(), figure_type);

        let rectangle = Constructor("Rectangle".to_string(), vec![Box::new(CInt(5))]);
        let result = check_expr(&rectangle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_undefined() {
        let env = Environment::new();
        let circle = Constructor("Circle".to_string(), vec![Box::new(CInt(5))]);
        let result = check_expr(&circle, &env);
        assert!(result.is_err());
    }

    #[test]
    fn test_adt_constructor_with_mutable_vars() {
        let mut env = Environment::new();
        let figure_type = HashMap::from([
            ("Circle".to_string(), vec![Type::TInteger]),
            (
                "Rectangle".to_string(),
                vec![Type::TInteger, Type::TInteger],
            ),
        ]);
        env.map_adt("Figure".to_string(), figure_type);

        env.map_variable("radius".to_string(), true, Type::TInteger);

        let circle = Constructor(
            "Circle".to_string(),
            vec![Box::new(Var("radius".to_string()))],
        );
        let result = check_expr(&circle, &env);
        assert!(result.is_ok());
    }
}
