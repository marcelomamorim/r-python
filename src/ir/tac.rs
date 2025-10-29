use crate::ir::ast::{Expression, FormalArgument, Function, Statement};
use std::fmt;

/// Representation of a compiled program in three-address code.
#[derive(Debug, Clone, PartialEq)]
pub struct TacProgram {
    functions: Vec<TacFunction>,
}

impl TacProgram {
    /// Creates an empty TAC program.
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    /// Adds a function to the program.
    pub fn push_function(&mut self, function: TacFunction) {
        self.functions.push(function);
    }

    /// Returns all functions defined in the program.
    pub fn functions(&self) -> &[TacFunction] {
        &self.functions
    }

    /// Consumes the program returning all functions.
    pub fn into_functions(self) -> Vec<TacFunction> {
        self.functions
    }

    /// Generates a pretty string representation of the TAC program.
    pub fn to_pretty_string(&self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for TacProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, function) in self.functions.iter().enumerate() {
            writeln!(
                f,
                "function {}({})",
                function.name,
                function.params.join(", ")
            )?;
            for instruction in &function.instructions {
                writeln!(f, "    {}", instruction)?;
            }
            if index + 1 < self.functions.len() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Representation of a function in TAC form.
#[derive(Debug, Clone, PartialEq)]
pub struct TacFunction {
    name: String,
    params: Vec<String>,
    instructions: Vec<TacInstruction>,
}

impl TacFunction {
    fn new(name: String, params: Vec<String>, instructions: Vec<TacInstruction>) -> Self {
        Self {
            name,
            params,
            instructions,
        }
    }

    /// Returns the function name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the list of parameters.
    pub fn params(&self) -> &[String] {
        &self.params
    }

    /// Returns all instructions inside the function.
    pub fn instructions(&self) -> &[TacInstruction] {
        &self.instructions
    }
}

/// Individual TAC instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum TacInstruction {
    Label(String),
    Assign {
        target: String,
        value: TacOperand,
    },
    Binary {
        target: String,
        op: TacBinaryOp,
        left: TacOperand,
        right: TacOperand,
    },
    Unary {
        target: String,
        op: TacUnaryOp,
        operand: TacOperand,
    },
    Call {
        target: Option<String>,
        function: String,
        args: Vec<TacOperand>,
    },
    Goto(String),
    IfGoto {
        condition: TacOperand,
        label: String,
    },
    Return(Option<TacOperand>),
    Construct {
        target: String,
        ctor: String,
        args: Vec<TacOperand>,
    },
    Comment(String),
}

impl fmt::Display for TacInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TacInstruction::Label(label) => write!(f, "{}:", label),
            TacInstruction::Assign { target, value } => write!(f, "{} = {}", target, value),
            TacInstruction::Binary {
                target,
                op,
                left,
                right,
            } => write!(f, "{} = {} {} {}", target, left, op, right),
            TacInstruction::Unary {
                target,
                op,
                operand,
            } => write!(f, "{} = {} {}", target, op, operand),
            TacInstruction::Call {
                target,
                function,
                args,
            } => {
                let joined = join_operands(args);
                if let Some(target) = target {
                    write!(f, "{} = call {}({})", target, function, joined)
                } else {
                    write!(f, "call {}({})", function, joined)
                }
            }
            TacInstruction::Goto(label) => write!(f, "goto {}", label),
            TacInstruction::IfGoto { condition, label } => {
                write!(f, "if {} goto {}", condition, label)
            }
            TacInstruction::Return(Some(value)) => write!(f, "return {}", value),
            TacInstruction::Return(None) => write!(f, "return"),
            TacInstruction::Construct { target, ctor, args } => {
                let joined = join_operands(args);
                write!(f, "{} = {}({})", target, ctor, joined)
            }
            TacInstruction::Comment(message) => write!(f, "# {}", message),
        }
    }
}

fn join_operands(operands: &[TacOperand]) -> String {
    operands
        .iter()
        .map(|op| format!("{}", op))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Operands used by TAC instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum TacOperand {
    Temp(String),
    Var(String),
    ConstInt(i32),
    ConstReal(f64),
    ConstBool(bool),
    ConstString(String),
    Void,
}

impl fmt::Display for TacOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TacOperand::Temp(name) | TacOperand::Var(name) => write!(f, "{}", name),
            TacOperand::ConstInt(value) => write!(f, "{}", value),
            TacOperand::ConstReal(value) => write!(f, "{}", value),
            TacOperand::ConstBool(value) => write!(f, "{}", value),
            TacOperand::ConstString(value) => write!(f, "\"{}\"", value),
            TacOperand::Void => write!(f, "void"),
        }
    }
}

/// Binary operations supported in TAC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
}

impl fmt::Display for TacBinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            TacBinaryOp::Add => "+",
            TacBinaryOp::Sub => "-",
            TacBinaryOp::Mul => "*",
            TacBinaryOp::Div => "/",
            TacBinaryOp::Eq => "==",
            TacBinaryOp::Neq => "!=",
            TacBinaryOp::Gt => ">",
            TacBinaryOp::Lt => "<",
            TacBinaryOp::Gte => ">=",
            TacBinaryOp::Lte => "<=",
            TacBinaryOp::And => "and",
            TacBinaryOp::Or => "or",
        };
        write!(f, "{}", symbol)
    }
}

/// Unary operations supported in TAC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TacUnaryOp {
    Not,
}

impl fmt::Display for TacUnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            TacUnaryOp::Not => "not",
        };
        write!(f, "{}", symbol)
    }
}

/// Helper responsible for turning the AST into TAC instructions.
pub struct TacGenerator {
    temp_counter: usize,
    label_counter: usize,
}

impl TacGenerator {
    /// Creates a new generator with fresh counters.
    pub fn new() -> Self {
        Self {
            temp_counter: 0,
            label_counter: 0,
        }
    }

    /// Generates a TAC program from a list of AST statements.
    pub fn generate(&mut self, statements: &[Statement]) -> TacProgram {
        let mut program = TacProgram::new();
        let mut main_builder = TacFunctionBuilder::new("__main__", vec![]);

        for stmt in statements.iter().cloned() {
            self.compile_statement(stmt, &mut main_builder, &mut program);
        }

        main_builder.ensure_terminal_return();
        program.push_function(main_builder.finish());
        program
    }

    fn compile_statement(
        &mut self,
        stmt: Statement,
        builder: &mut TacFunctionBuilder,
        program: &mut TacProgram,
    ) {
        match stmt {
            Statement::VarDeclaration(name, expr) | Statement::ValDeclaration(name, expr) => {
                let value = self.compile_expression(*expr, builder);
                builder.emit(TacInstruction::Assign {
                    target: name,
                    value,
                });
            }
            Statement::Assignment(name, expr) => {
                let value = self.compile_expression(*expr, builder);
                builder.emit(TacInstruction::Assign {
                    target: name,
                    value,
                });
            }
            Statement::Block(statements) => {
                for nested in statements {
                    self.compile_statement(nested, builder, program);
                }
            }
            Statement::Sequence(first, second) => {
                self.compile_statement(*first, builder, program);
                self.compile_statement(*second, builder, program);
            }
            Statement::Return(expr) => {
                let value = self.compile_expression(*expr, builder);
                builder.emit(TacInstruction::Return(Some(value)));
            }
            Statement::IfThenElse(condition, then_branch, else_branch) => {
                let then_label = self.new_label("then");
                let end_label = self.new_label("end_if");
                let else_label = else_branch
                    .as_ref()
                    .map(|_| self.new_label("else"))
                    .unwrap_or_else(|| end_label.clone());

                let cond_operand = self.compile_expression(*condition, builder);
                builder.emit(TacInstruction::IfGoto {
                    condition: cond_operand,
                    label: then_label.clone(),
                });
                builder.emit(TacInstruction::Goto(else_label.clone()));

                builder.emit(TacInstruction::Label(then_label));
                self.compile_statement(*then_branch, builder, program);
                builder.emit(TacInstruction::Goto(end_label.clone()));

                if let Some(else_branch) = else_branch {
                    builder.emit(TacInstruction::Label(else_label));
                    self.compile_statement(*else_branch, builder, program);
                }

                builder.emit(TacInstruction::Label(end_label));
            }
            Statement::IfChain {
                branches,
                else_branch,
            } => {
                let lowered = lower_if_chain(branches, else_branch);
                self.compile_statement(lowered, builder, program);
            }
            Statement::While(condition, body) => {
                let start_label = self.new_label("loop_start");
                let body_label = self.new_label("loop_body");
                let end_label = self.new_label("loop_end");

                builder.emit(TacInstruction::Label(start_label.clone()));
                let cond_operand = self.compile_expression(*condition, builder);
                builder.emit(TacInstruction::IfGoto {
                    condition: cond_operand,
                    label: body_label.clone(),
                });
                builder.emit(TacInstruction::Goto(end_label.clone()));

                builder.emit(TacInstruction::Label(body_label));
                self.compile_statement(*body, builder, program);
                builder.emit(TacInstruction::Goto(start_label));
                builder.emit(TacInstruction::Label(end_label));
            }
            Statement::For(name, iterable, body) => {
                let temp_iter = self.compile_expression(*iterable, builder);
                builder.emit(TacInstruction::Comment(format!(
                    "for-loop over {} assigned to {}",
                    temp_iter, name
                )));
                self.compile_statement(*body, builder, program);
            }
            Statement::FuncDef(function) => {
                self.compile_function(function, program);
            }
            Statement::Assert(_, _)
            | Statement::AssertTrue(_, _)
            | Statement::AssertFalse(_, _) => {
                builder.emit(TacInstruction::Comment(
                    "assert statements are evaluated at runtime".to_string(),
                ));
            }
            Statement::AssertEQ(_, _, _) | Statement::AssertNEQ(_, _, _) => {
                builder.emit(TacInstruction::Comment(
                    "assert equality checks are evaluated at runtime".to_string(),
                ));
            }
            Statement::TestDef(func) => {
                builder.emit(TacInstruction::Comment(format!(
                    "test definition '{}' skipped in TAC",
                    func.name
                )));
            }
            Statement::ModTestDef(name, _) => {
                builder.emit(TacInstruction::Comment(format!(
                    "module test '{}' skipped in TAC",
                    name
                )));
            }
            Statement::AssertFails(message) => builder.emit(TacInstruction::Comment(format!(
                "assertfails '{}' handled at runtime",
                message
            ))),
            Statement::TypeDeclaration(name, _) => builder.emit(TacInstruction::Comment(format!(
                "type declaration '{}' available at runtime",
                name
            ))),
        }
    }

    fn compile_function(&mut self, function: Function, program: &mut TacProgram) {
        let Function {
            name, params, body, ..
        } = function;

        let params = params
            .into_iter()
            .map(|FormalArgument { argument_name, .. }| argument_name)
            .collect();
        let mut builder = TacFunctionBuilder::new(name, params);

        if let Some(body) = body {
            self.compile_statement(*body, &mut builder, program);
        } else {
            builder.emit(TacInstruction::Comment("empty function body".to_string()));
        }

        builder.ensure_terminal_return();
        program.push_function(builder.finish());
    }

    fn compile_expression(
        &mut self,
        expr: Expression,
        builder: &mut TacFunctionBuilder,
    ) -> TacOperand {
        match expr {
            Expression::CInt(value) => TacOperand::ConstInt(value),
            Expression::CReal(value) => TacOperand::ConstReal(value),
            Expression::CTrue => TacOperand::ConstBool(true),
            Expression::CFalse => TacOperand::ConstBool(false),
            Expression::CString(value) => TacOperand::ConstString(value),
            Expression::CVoid => TacOperand::Void,
            Expression::Var(name) => TacOperand::Var(name),
            Expression::Add(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Add, builder),
            Expression::Sub(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Sub, builder),
            Expression::Mul(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Mul, builder),
            Expression::Div(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Div, builder),
            Expression::EQ(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Eq, builder),
            Expression::NEQ(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Neq, builder),
            Expression::GT(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Gt, builder),
            Expression::LT(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Lt, builder),
            Expression::GTE(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Gte, builder),
            Expression::LTE(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Lte, builder),
            Expression::And(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::And, builder),
            Expression::Or(lhs, rhs) => self.compile_binary(*lhs, *rhs, TacBinaryOp::Or, builder),
            Expression::Not(expr) => self.compile_unary(*expr, TacUnaryOp::Not, builder),
            Expression::FuncCall(name, args) => {
                let compiled_args = args
                    .into_iter()
                    .map(|arg| self.compile_expression(arg, builder))
                    .collect::<Vec<_>>();
                let temp = self.new_temp();
                builder.emit(TacInstruction::Call {
                    target: Some(temp.clone()),
                    function: name,
                    args: compiled_args,
                });
                TacOperand::Temp(temp)
            }
            Expression::COk(expr) => self.compile_constructor("Ok", vec![expr], builder),
            Expression::CErr(expr) => self.compile_constructor("Err", vec![expr], builder),
            Expression::CJust(expr) => self.compile_constructor("Just", vec![expr], builder),
            Expression::CNothing => self.compile_constructor("Nothing", vec![], builder),
            Expression::Unwrap(expr) => {
                let value = self.compile_expression(*expr, builder);
                let temp = self.new_temp();
                builder.emit(TacInstruction::Call {
                    target: Some(temp.clone()),
                    function: "__unwrap".to_string(),
                    args: vec![value],
                });
                TacOperand::Temp(temp)
            }
            Expression::IsError(expr) => {
                let value = self.compile_expression(*expr, builder);
                let temp = self.new_temp();
                builder.emit(TacInstruction::Call {
                    target: Some(temp.clone()),
                    function: "__is_error".to_string(),
                    args: vec![value],
                });
                TacOperand::Temp(temp)
            }
            Expression::IsNothing(expr) => {
                let value = self.compile_expression(*expr, builder);
                let temp = self.new_temp();
                builder.emit(TacInstruction::Call {
                    target: Some(temp.clone()),
                    function: "__is_nothing".to_string(),
                    args: vec![value],
                });
                TacOperand::Temp(temp)
            }
            Expression::Propagate(expr) => {
                let value = self.compile_expression(*expr, builder);
                let temp = self.new_temp();
                builder.emit(TacInstruction::Call {
                    target: Some(temp.clone()),
                    function: "__propagate".to_string(),
                    args: vec![value],
                });
                TacOperand::Temp(temp)
            }
            Expression::ListValue(items) => {
                let compiled_items = items
                    .into_iter()
                    .map(|item| self.compile_expression(item, builder))
                    .collect::<Vec<_>>();
                let temp = self.new_temp();
                builder.emit(TacInstruction::Construct {
                    target: temp.clone(),
                    ctor: "list".to_string(),
                    args: compiled_items,
                });
                TacOperand::Temp(temp)
            }
            Expression::Tuple(items) => {
                let compiled_items = items
                    .into_iter()
                    .map(|item| self.compile_expression(item, builder))
                    .collect::<Vec<_>>();
                let temp = self.new_temp();
                builder.emit(TacInstruction::Construct {
                    target: temp.clone(),
                    ctor: "tuple".to_string(),
                    args: compiled_items,
                });
                TacOperand::Temp(temp)
            }
            Expression::Constructor(name, items) => {
                let compiled_items = items
                    .into_iter()
                    .map(|item| self.compile_expression(*item, builder))
                    .collect::<Vec<_>>();
                let temp = self.new_temp();
                builder.emit(TacInstruction::Construct {
                    target: temp.clone(),
                    ctor: name,
                    args: compiled_items,
                });
                TacOperand::Temp(temp)
            }
        }
    }

    fn compile_binary(
        &mut self,
        lhs: Expression,
        rhs: Expression,
        op: TacBinaryOp,
        builder: &mut TacFunctionBuilder,
    ) -> TacOperand {
        let left = self.compile_expression(lhs, builder);
        let right = self.compile_expression(rhs, builder);
        let temp = self.new_temp();
        builder.emit(TacInstruction::Binary {
            target: temp.clone(),
            op,
            left,
            right,
        });
        TacOperand::Temp(temp)
    }

    fn compile_unary(
        &mut self,
        expr: Expression,
        op: TacUnaryOp,
        builder: &mut TacFunctionBuilder,
    ) -> TacOperand {
        let operand = self.compile_expression(expr, builder);
        let temp = self.new_temp();
        builder.emit(TacInstruction::Unary {
            target: temp.clone(),
            op,
            operand,
        });
        TacOperand::Temp(temp)
    }

    fn compile_constructor(
        &mut self,
        ctor: &str,
        args: Vec<Box<Expression>>,
        builder: &mut TacFunctionBuilder,
    ) -> TacOperand {
        let compiled = args
            .into_iter()
            .map(|arg| self.compile_expression(*arg, builder))
            .collect::<Vec<_>>();
        let temp = self.new_temp();
        builder.emit(TacInstruction::Construct {
            target: temp.clone(),
            ctor: ctor.to_string(),
            args: compiled,
        });
        TacOperand::Temp(temp)
    }

    fn new_temp(&mut self) -> String {
        let name = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }
}

fn lower_if_chain(
    branches: Vec<(Box<Expression>, Box<Statement>)>,
    else_branch: Option<Box<Statement>>,
) -> Statement {
    let mut current = else_branch;
    for (condition, branch) in branches.into_iter().rev() {
        current = Some(Box::new(Statement::IfThenElse(condition, branch, current)));
    }

    current
        .map(|boxed| *boxed)
        .unwrap_or_else(|| Statement::Block(vec![]))
}

struct TacFunctionBuilder {
    name: String,
    params: Vec<String>,
    instructions: Vec<TacInstruction>,
}

impl TacFunctionBuilder {
    fn new(name: impl Into<String>, params: Vec<String>) -> Self {
        Self {
            name: name.into(),
            params,
            instructions: Vec::new(),
        }
    }

    fn emit(&mut self, instruction: TacInstruction) {
        self.instructions.push(instruction);
    }

    fn ensure_terminal_return(&mut self) {
        match self.instructions.last() {
            Some(TacInstruction::Return(_)) => {}
            _ => self.instructions.push(TacInstruction::Return(None)),
        }
    }

    fn finish(self) -> TacFunction {
        TacFunction::new(self.name, self.params, self.instructions)
    }
}

/// Emits assembly for a TAC program when the LLVM backend is enabled.
pub fn emit_assembly(program: &TacProgram) -> Result<String, String> {
    #[cfg(feature = "llvm-backend")]
    {
        use inkwell::context::Context;
        use inkwell::targets::{FileType, InitializationConfig, Target, TargetMachine};
        use std::collections::HashMap;

        Target::initialize_native(&InitializationConfig::default())
            .map_err(|err| format!("failed to initialise native target: {err}"))?;

        let context = Context::create();
        let module = context.create_module("rpython");
        let builder = context.create_builder();
        let i32_type = context.i32_type();

        for function in &program.functions {
            let fn_type = i32_type.fn_type(&[], false);
            let fn_value = module.add_function(&function.name, fn_type, None);
            let entry = context.append_basic_block(fn_value, "entry");
            builder.position_at_end(entry);

            let mut slots: HashMap<String, inkwell::values::IntValue> = HashMap::new();
            let mut has_return = false;

            for instruction in &function.instructions {
                match instruction {
                    TacInstruction::Assign { target, value } => {
                        let val = operand_as_int(value, &context, &slots)?;
                        slots.insert(target.clone(), val);
                    }
                    TacInstruction::Binary {
                        target,
                        op,
                        left,
                        right,
                    } => {
                        let lhs = operand_as_int(left, &context, &slots)?;
                        let rhs = operand_as_int(right, &context, &slots)?;
                        let result = match op {
                            TacBinaryOp::Add => builder.build_int_add(lhs, rhs, "addtmp"),
                            TacBinaryOp::Sub => builder.build_int_sub(lhs, rhs, "subtmp"),
                            TacBinaryOp::Mul => builder.build_int_mul(lhs, rhs, "multmp"),
                            TacBinaryOp::Div => builder.build_int_signed_div(lhs, rhs, "divtmp"),
                            TacBinaryOp::Eq => builder.build_int_compare(
                                inkwell::IntPredicate::EQ,
                                lhs,
                                rhs,
                                "eqtmp",
                            ),
                            TacBinaryOp::Neq => builder.build_int_compare(
                                inkwell::IntPredicate::NE,
                                lhs,
                                rhs,
                                "neqtmp",
                            ),
                            TacBinaryOp::Gt => builder.build_int_compare(
                                inkwell::IntPredicate::SGT,
                                lhs,
                                rhs,
                                "gttmp",
                            ),
                            TacBinaryOp::Lt => builder.build_int_compare(
                                inkwell::IntPredicate::SLT,
                                lhs,
                                rhs,
                                "lttmp",
                            ),
                            TacBinaryOp::Gte => builder.build_int_compare(
                                inkwell::IntPredicate::SGE,
                                lhs,
                                rhs,
                                "gtetmp",
                            ),
                            TacBinaryOp::Lte => builder.build_int_compare(
                                inkwell::IntPredicate::SLE,
                                lhs,
                                rhs,
                                "ltetmp",
                            ),
                            TacBinaryOp::And | TacBinaryOp::Or => {
                                return Err(
                                    "logical operations are not yet supported by the LLVM backend"
                                        .to_string(),
                                )
                            }
                        };
                        slots.insert(target.clone(), result);
                    }
                    TacInstruction::Return(Some(value)) => {
                        let ret = operand_as_int(value, &context, &slots)?;
                        builder.build_return(Some(&ret));
                        has_return = true;
                        break;
                    }
                    TacInstruction::Return(None) => {
                        builder.build_return(Some(&i32_type.const_zero()));
                        has_return = true;
                        break;
                    }
                    TacInstruction::Unary { .. }
                    | TacInstruction::Call { .. }
                    | TacInstruction::Goto(_)
                    | TacInstruction::IfGoto { .. }
                    | TacInstruction::Label(_)
                    | TacInstruction::Construct { .. }
                    | TacInstruction::Comment(_) => {
                        return Err(
                            "current LLVM backend supports only straight-line arithmetic code"
                                .to_string(),
                        )
                    }
                }
            }

            if !has_return {
                builder.build_return(Some(&i32_type.const_zero()));
            }
        }

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|err| format!("failed to obtain target: {err}"))?;
        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                inkwell::targets::RelocMode::Default,
                inkwell::targets::CodeModel::Default,
            )
            .ok_or_else(|| "failed to create target machine".to_string())?;

        let buffer = machine
            .write_to_memory_buffer(&module, FileType::Assembly)
            .map_err(|err| format!("failed to emit assembly: {err}"))?;

        return Ok(String::from_utf8(buffer.as_slice().to_vec())
            .unwrap_or_else(|_| String::from("; invalid UTF-8 assembly output")));
    }

    #[cfg(not(feature = "llvm-backend"))]
    {
        let _ = program;
        Err("Assembly generation requires the 'llvm-backend' feature".to_string())
    }
}

#[cfg(feature = "llvm-backend")]
fn operand_as_int<'ctx>(
    operand: &TacOperand,
    context: &'ctx inkwell::context::Context,
    slots: &std::collections::HashMap<String, inkwell::values::IntValue<'ctx>>,
) -> Result<inkwell::values::IntValue<'ctx>, String> {
    match operand {
        TacOperand::ConstInt(value) => Ok(context.i32_type().const_int(*value as u64, true)),
        TacOperand::ConstBool(value) => Ok(context
            .i1_type()
            .const_int(if *value { 1 } else { 0 }, false)
            .const_cast(context.i32_type(), true)),
        TacOperand::Temp(name) | TacOperand::Var(name) => slots
            .get(name)
            .copied()
            .ok_or_else(|| format!("unknown operand '{name}'")),
        _ => Err(format!(
            "operand '{operand}' is not supported by the LLVM backend"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ast::Expression;

    #[test]
    fn generates_basic_assignment() {
        let statements = vec![Statement::Assignment(
            "x".to_string(),
            Box::new(Expression::CInt(42)),
        )];

        let mut generator = TacGenerator::new();
        let program = generator.generate(&statements);

        assert_eq!(program.functions().len(), 1);
        let main = &program.functions()[0];
        assert_eq!(main.name(), "__main__");
        assert!(matches!(
            main.instructions().first(),
            Some(TacInstruction::Assign { target, value }) if target == "x" && *value == TacOperand::ConstInt(42)
        ));
    }

    #[test]
    fn generates_binary_expression() {
        let statements = vec![Statement::Assignment(
            "result".to_string(),
            Box::new(Expression::Add(
                Box::new(Expression::CInt(1)),
                Box::new(Expression::CInt(2)),
            )),
        )];

        let mut generator = TacGenerator::new();
        let program = generator.generate(&statements);
        let main = &program.functions()[0];

        assert_eq!(main.instructions().len(), 3);
        assert!(matches!(
            main.instructions()[0],
            TacInstruction::Binary { .. }
        ));
        assert!(matches!(
            main.instructions()[1],
            TacInstruction::Assign { .. }
        ));
        assert!(matches!(main.instructions()[2], TacInstruction::Return(_)));
    }
}
