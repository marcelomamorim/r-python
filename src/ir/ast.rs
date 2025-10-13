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

//Represents function signature
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

impl Type {
    fn hash_helper<H: Hasher>(&self, state: &mut H) {
        match self {
            Type::TInteger => state.write_u8(0),
            Type::TBool => state.write_u8(1),
            Type::TReal => state.write_u8(2),
            Type::TString => state.write_u8(3),
            Type::TVoid => state.write_u8(4),
            Type::TFunction(ret, params) => {
                state.write_u8(5);
                ret.hash(state);
                for param in params {
                    param.hash(state);
                }
            }
            Type::TList(inner) => {
                state.write_u8(6);
                inner.hash(state);
            }
            Type::TTuple(elements) => {
                state.write_u8(7);
                for element in elements {
                    element.hash(state);
                }
            }
            Type::TMaybe(inner) => {
                state.write_u8(8);
                inner.hash(state);
            }
            Type::TResult(ok, err) => {
                state.write_u8(9);
                ok.hash(state);
                err.hash(state);
            }
            Type::TUnion(types) => {
                state.write_u8(10);
                for ty in types {
                    ty.hash(state);
                }
            }
            Type::TAny => state.write_u8(11),
            Type::TAlgebraicData(name, constructors) => {
                state.write_u8(12);
                name.hash(state);
                let mut entries: Vec<_> = constructors.iter().collect();
                entries.sort_by(|(a, _), (b, _)| a.cmp(b));
                for (ctor_name, ctor_types) in entries {
                    ctor_name.hash(state);
                    for ty in ctor_types {
                        ty.hash(state);
                    }
                }
            }
        }
    }
}

impl Hash for Type {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_helper(state);
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
            Type::TFunction(ret, params) => {
                let params_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn({}) -> {}", params_str, ret)
            }
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
                let types = types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ");
                write!(f, "{}", types)
            }
            Type::TAny => write!(f, "any"),
            Type::TAlgebraicData(name, _constructors) => write!(f, "{}", name),
        }
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
