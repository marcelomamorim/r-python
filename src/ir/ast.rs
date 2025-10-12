use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// Type alias for variable and function names
pub type Name = String;

// Represents a function in the AST
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Name,
    pub kind: Type,
    pub params: Vec<FormalArgument>,
    pub body: Option<Box<Statement>>,
}

impl Function {
    // Creates a new function with default values
    pub fn new() -> Function {
        return Function {
            name: "__main__".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };
    }
}

// Represents a formal argument in a function definition
#[derive(Debug, PartialEq, Clone)]
pub struct FormalArgument {
    pub argument_name: Name,
    pub argument_type: Type,
}

impl FormalArgument {
    // Creates a new formal argument
    pub fn new(argument_name: Name, argument_type: Type) -> Self {
        FormalArgument {
            argument_name,
            argument_type,
        }
    }
}

// Represents the types that can be used in the AST
#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    TInteger,
    TBool,
    TReal,
    TString,
    TVoid,
    TFunction(Box<Type>, Vec<Type>),
    TList(Box<Type>),
    TTuple(Vec<Type>),
    TMaybe(Box<Type>),
    TResult(Box<Type>, Box<Type>), // Ok, Error
    TUnion(Vec<Type>),
    TAny,
    TAlgebraicData(Name, HashMap<Name, Vec<Type>>),
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Type::TInteger => 0u8.hash(state),
            Type::TBool => 1u8.hash(state),
            Type::TReal => 2u8.hash(state),
            Type::TString => 3u8.hash(state),
            Type::TVoid => 4u8.hash(state),
            Type::TFunction(ret, params) => {
                5u8.hash(state);
                ret.hash(state);
                params.hash(state);
            }
            Type::TList(inner) => {
                6u8.hash(state);
                inner.hash(state);
            }
            Type::TTuple(elements) => {
                7u8.hash(state);
                elements.hash(state);
            }
            Type::TMaybe(inner) => {
                8u8.hash(state);
                inner.hash(state);
            }
            Type::TResult(ok, err) => {
                9u8.hash(state);
                ok.hash(state);
                err.hash(state);
            }
            Type::TUnion(types) => {
                10u8.hash(state);
                types.hash(state);
            }
            Type::TAny => 11u8.hash(state),
            Type::TAlgebraicData(name, constructors) => {
                12u8.hash(state);
                name.hash(state);

                let mut entries: Vec<_> = constructors.iter().collect();
                entries.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
                for (ctor_name, ctor_types) in entries {
                    ctor_name.hash(state);
                    ctor_types.hash(state);
                }
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::TInteger => write!(f, "int"),
            Type::TBool => write!(f, "bool"),
            Type::TReal => write!(f, "real"),
            Type::TString => write!(f, "string"),
            Type::TVoid => write!(f, "void"),
            Type::TAny => write!(f, "any"),
            Type::TList(inner) => write!(f, "[{}]", inner),
            Type::TTuple(elements) => {
                let types = elements
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "({})", types)
            }
            Type::TMaybe(inner) => write!(f, "Maybe<{}>", inner),
            Type::TResult(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            Type::TUnion(types) => {
                let types_str = types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ");
                write!(f, "{}", types_str)
            }
            Type::TFunction(ret, params) => {
                let params_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({}) -> {}", params_str, ret)
            }
            Type::TAlgebraicData(name, _constructors) => write!(f, "{}", name),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct FuncSignature {
    pub name: Name,
    pub argument_types: Vec<Type>,
}

impl FuncSignature {
    pub fn new() -> FuncSignature {
        FuncSignature {
            name: "".to_string(),
            argument_types: vec![],
        }
    }

    pub fn from_func(func: &Function) -> FuncSignature {
        FuncSignature {
            name: func.name.clone(),
            argument_types: func
                .params
                .iter()
                .map(|arg| arg.argument_type.clone())
                .collect(),
        }
    }
}

impl fmt::Display for FuncSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.argument_types
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

// Represents expressions in the AST
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    // Constants
    CTrue,
    CFalse,
    CInt(i32),
    CReal(f64),
    CString(String),
    CVoid,

    // Variable reference
    Var(Name),

    // Function call
    FuncCall(Name, Vec<Expression>),

    // Arithmetic expressions over numbers
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),

    // Boolean expressions over booleans
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    // Relational expressions over numbers
    EQ(Box<Expression>, Box<Expression>),
    NEQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),

    // Error-related expressions
    COk(Box<Expression>),
    CErr(Box<Expression>),
    CJust(Box<Expression>),
    CNothing,
    Unwrap(Box<Expression>),
    IsError(Box<Expression>),
    IsNothing(Box<Expression>),
    Propagate(Box<Expression>),

    //Lambda expression
    Lambda(Function),

    // List value
    ListValue(Vec<Expression>),

    // Tuple value
    Tuple(Vec<Expression>),

    // Constructor
    Constructor(Name, Vec<Box<Expression>>),
}

// Represents statements in the AST
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDeclaration(Name, Box<Expression>),
    ValDeclaration(Name, Box<Expression>),
    Assignment(Name, Box<Expression>),
    IfThenElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    IfChain {
        branches: Vec<(Box<Expression>, Box<Statement>)>,
        else_branch: Option<Box<Statement>>,
    },
    While(Box<Expression>, Box<Statement>),
    For(Name, Box<Expression>, Box<Statement>),
    Block(Vec<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    Assert(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertTrue(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertFalse(Box<Expression>, Box<Expression>), //Segundo expression deve ser String
    AssertEQ(Box<Expression>, Box<Expression>, Box<Expression>), //Terceiro expression deve ser String
    AssertNEQ(Box<Expression>, Box<Expression>, Box<Expression>), //Terceiro expression deve ser String
    TestDef(Function),
    ModTestDef(Name, Box<Statement>),
    AssertFails(String),
    FuncDef(Function),
    Return(Box<Expression>),
    TypeDeclaration(Name, HashMap<Name, Vec<Type>>),
    Match(Box<Expression>, Vec<(Expression, Statement)>),
}
