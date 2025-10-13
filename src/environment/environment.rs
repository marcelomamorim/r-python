use crate::ir::ast::{FuncSignature, Function, Name, Type};
use indexmap::IndexMap;
use std::collections::{HashMap, LinkedList};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct Scope<A> {
    pub variables: HashMap<Name, (bool, A)>,
    pub functions: HashMap<FuncSignature, Function>,
    pub adts: HashMap<Name, Arc<HashMap<Name, Vec<Type>>>>,
    pub tests: IndexMap<Name, Function>,
}

impl<A: Clone> Scope<A> {
    fn new() -> Scope<A> {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            adts: HashMap::new(),
            tests: IndexMap::new(),
        }
    }

    fn map_variable(&mut self, var: Name, mutable: bool, value: A) {
        self.variables.insert(var, (mutable, value));
    }

    fn map_function(&mut self, function: Function) {
        let signature = FuncSignature::from_func(&function);
        self.functions.insert(signature, function);
    }

    fn map_test(&mut self, test: Function) {
        self.tests.insert(test.name.clone(), test);
    }

    fn map_adt(&mut self, name: Name, adt: HashMap<Name, Vec<Type>>) {
        self.adts.insert(name.clone(), Arc::new(adt));
    }

    fn lookup_var(&self, var: &Name) -> Option<(bool, A)> {
        self.variables
            .get(var)
            .map(|(mutable, value)| (*mutable, value.clone()))
    }

    fn update_var(&mut self, var: &Name, value: A) -> bool {
        if let Some((_, slot)) = self.variables.get_mut(var) {
            *slot = value;
            true
        } else {
            false
        }
    }

    fn remove_var(&mut self, var: &Name) -> bool {
        self.variables.remove(var).is_some()
    }

    fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        self.functions.get(func_signature)
    }

    fn lookup_function_by_name(&self, name: &Name) -> Option<&Function> {
        self.functions.iter().find_map(|(signature, function)| {
            if &signature.name == name {
                Some(function)
            } else {
                None
            }
        })
    }

    fn lookup_test(&self, name: &Name) -> Option<&Function> {
        self.tests.get(name)
    }

    fn lookup_adt(&self, name: &Name) -> Option<&Arc<HashMap<Name, Vec<Type>>>> {
        self.adts.get(name)
    }
}

#[derive(Clone)]
pub struct Environment<A: Clone> {
    pub current_func: FuncSignature,
    pub globals: Scope<A>,
    pub stack: LinkedList<Scope<A>>,
}

impl<A: Clone> Environment<A> {
    pub fn new() -> Environment<A> {
        Environment {
            current_func: FuncSignature::new(),
            globals: Scope::new(),
            stack: LinkedList::new(),
        }
    }

    pub fn get_current_func(&self) -> FuncSignature {
        self.current_func.clone()
    }

    pub fn set_current_func(&mut self, func_signature: &FuncSignature) {
        self.current_func = func_signature.clone();
    }

    pub fn get_current_scope(&self) -> &Scope<A> {
        self.stack.front().unwrap_or(&self.globals)
    }

    pub fn set_global_functions(&mut self, global_functions: HashMap<FuncSignature, Function>) {
        self.globals.functions = global_functions;
    }

    pub fn map_variable(&mut self, var: Name, mutable: bool, value: A) {
        match self.stack.front_mut() {
            None => self.globals.map_variable(var, mutable, value),
            Some(top) => top.map_variable(var, mutable, value),
        }
    }

    pub fn map_function(&mut self, function: Function) {
        match self.stack.front_mut() {
            None => self.globals.map_function(function),
            Some(top) => top.map_function(function),
        }
    }

    pub fn map_test(&mut self, test: Function) {
        match self.stack.front_mut() {
            None => self.globals.map_test(test),
            Some(top) => top.map_test(test),
        }
    }

    pub fn map_adt(&mut self, name: Name, cons: HashMap<Name, Vec<Type>>) {
        match self.stack.front_mut() {
            None => self.globals.map_adt(name, cons),
            Some(top) => top.map_adt(name, cons),
        }
    }

    pub fn lookup(&self, var: &Name) -> Option<(bool, A)> {
        for scope in &self.stack {
            if let Some(value) = scope.lookup_var(var) {
                return Some(value);
            }
        }
        self.globals.lookup_var(var)
    }

    /// Update an existing variable in the nearest scope where it's defined.
    /// Returns true if the variable existed and was updated; false otherwise.
    pub fn update_existing_variable(&mut self, var: &Name, value: A) -> bool {
        for scope in self.stack.iter_mut() {
            if scope.update_var(var, value.clone()) {
                return true;
            }
        }
        self.globals.update_var(var, value)
    }

    /// Remove a variable from the nearest scope where it's defined.
    /// Returns true if something was removed; false otherwise.
    pub fn remove_variable(&mut self, var: &Name) -> bool {
        for scope in self.stack.iter_mut() {
            if scope.remove_var(var) {
                return true;
            }
        }
        self.globals.remove_var(var)
    }

    pub fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        for scope in &self.stack {
            if let Some(func) = scope.lookup_function(func_signature) {
                return Some(func);
            }
        }
        self.globals.lookup_function(func_signature)
    }

    pub fn lookup_function_by_name(&self, name: &Name) -> Option<&Function> {
        for scope in &self.stack {
            if let Some(func) = scope.lookup_function_by_name(name) {
                return Some(func);
            }
        }
        self.globals.lookup_function_by_name(name)
    }

    pub fn lookup_var_or_func(&self, name: &Name) -> Option<FuncOrVar<A>>
    where
        A: Debug,
    {
        for scope in &self.stack {
            if let Some(value) = scope.lookup_var(name) {
                return Some(FuncOrVar::Var(value));
            }
            if let Some(func) = scope.lookup_function_by_name(name) {
                return Some(FuncOrVar::Func(func.clone()));
            }
        }
        if let Some(value) = self.globals.lookup_var(name) {
            return Some(FuncOrVar::Var(value));
        }
        if let Some(func) = self.globals.lookup_function_by_name(name) {
            return Some(FuncOrVar::Func(func.clone()));
        }
        None
    }

    pub fn lookup_test(&self, name: &Name) -> Option<&Function> {
        for scope in &self.stack {
            if let Some(test) = scope.lookup_test(name) {
                return Some(test);
            }
        }
        self.globals.lookup_test(name)
    }

    pub fn get_all_tests(&self) -> Vec<Function> {
        let mut tests = Vec::new();
        for scope in &self.stack {
            for test in scope.tests.values() {
                tests.push(test.clone());
            }
        }
        for test in self.globals.tests.values() {
            tests.push(test.clone());
        }
        tests
    }

    pub fn lookup_adt(&self, name: &Name) -> Option<&Arc<HashMap<Name, Vec<Type>>>> {
        for scope in &self.stack {
            if let Some(cons) = scope.lookup_adt(name) {
                return Some(cons);
            }
        }
        self.globals.lookup_adt(name)
    }

    pub fn scoped_function(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn push(&mut self) {
        self.stack.push_front(Scope::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop_front();
    }

    pub fn get_all_variables(&self) -> Vec<(Name, (bool, A))> {
        let mut vars = Vec::new();

        for scope in &self.stack {
            for (name, value) in &scope.variables {
                if !vars.iter().any(|(n, _)| n == name) {
                    vars.push((name.clone(), value.clone()));
                }
            }
        }

        for (name, value) in &self.globals.variables {
            if !vars.iter().any(|(n, _)| n == name) {
                vars.push((name.clone(), value.clone()));
            }
        }

        vars
    }

    pub fn get_all_functions(&self) -> HashMap<FuncSignature, Function> {
        let mut all_functions = self.globals.functions.clone();
        for scope in self.stack.iter().rev() {
            for (signature, function) in &scope.functions {
                all_functions.insert(signature.clone(), function.clone());
            }
        }
        all_functions
    }
}

pub enum FuncOrVar<A: Clone + Debug> {
    Func(Function),
    Var((bool, A)),
}

pub struct TestResult {
    pub name: Name,
    pub result: bool,
    pub error: Option<String>,
}

impl TestResult {
    pub fn new(name: Name, result: bool, error: Option<String>) -> Self {
        TestResult {
            name,
            result,
            error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_scoping() {
        let mut env: Environment<i32> = Environment::new();

        env.map_variable("x".to_string(), true, 32);
        assert_eq!(Some((true, 32)), env.lookup(&"x".to_string()));

        env.push();
        env.map_variable("y".to_string(), true, 23);
        env.map_variable("x".to_string(), true, 55);

        env.push();
        env.map_variable("z".to_string(), true, 44);

        assert_eq!(Some((true, 55)), env.lookup(&"x".to_string()));
        assert_eq!(Some((true, 23)), env.lookup(&"y".to_string()));
        assert_eq!(Some((true, 44)), env.lookup(&"z".to_string()));

        env.pop();
        assert_eq!(Some((true, 55)), env.lookup(&"x".to_string()));
        assert_eq!(Some((true, 23)), env.lookup(&"y".to_string()));
        assert_eq!(None, env.lookup(&"z".to_string()));

        env.pop();
        assert_eq!(Some((true, 32)), env.lookup(&"x".to_string()));
        assert_eq!(None, env.lookup(&"y".to_string()));
    }

    #[test]
    fn test_function_scoping() {
        let mut env: Environment<i32> = Environment::new();

        let global_func = Function {
            name: "global".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        let local_func = Function {
            name: "local".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        env.map_function(global_func.clone());
        let global_signature = FuncSignature::from_func(&global_func);
        assert!(env.lookup_function(&global_signature).is_some());

        env.push();
        env.map_function(local_func.clone());
        let local_signature = FuncSignature::from_func(&local_func);

        assert!(env.lookup_function(&global_signature).is_some());
        assert!(env.lookup_function(&local_signature).is_some());

        env.pop();
        assert!(env.lookup_function(&global_signature).is_some());
        assert!(env.lookup_function(&local_signature).is_none());
    }
}
