# Environment Module Documentation

The environment module provides a lexically-scoped symbol table implementation for the R-Python language. It supports both variable and function bindings, with proper scope chain resolution.

## Overview

The module implements three core building blocks:
- [`FuncSignature`](../src/ir/ast.rs): a lightweight descriptor composed of the
  function name plus the ordered list of parameter types. Signatures are used as
  the canonical keys for every stored function, enabling overloading by
  argument types.
- `Scope<A>`: Represents a single scope with its variable, function, test and
  algebraic data type (ADT) bindings.
- `Environment<A>`: Manages a stack of scopes with a global scope at the bottom
  and keeps track of the currently-checked function signature.

The generic parameter `A` allows the environment to be used for different purposes:
- Type checking: `Environment<Type>`
- Interpretation: `Environment<Expression>`

## Structures

### Scope<A>

A single scope containing mappings for variables and functions.

```rust
pub struct Scope<A> {
    pub variables: HashMap<Name, (bool, A)>,
    pub functions: HashMap<FuncSignature, Function>,
    pub adts: HashMap<Name, Arc<HashMap<Name, Vec<Type>>>>,
    pub tests: IndexMap<Name, Function>,
}
```

#### Methods

- `new() -> Scope<A>`: Creates a new empty scope
- `map_variable(var: Name, mutable: bool, value: A)`: Binds a variable in the
  current scope together with its mutability flag (`var` for mutable,
  `val` for immutable).
- `map_function(function: Function)`: Binds a function in the current scope
  using its computed `FuncSignature` as the key.
- `map_test(test: Function)`: Registers a `test` definition in the current scope.
- `map_adt(name: Name, constructors: HashMap<Name, Vec<Type>>)`: Registers an
  ADT definition.
- `lookup_var(var: &Name) -> Option<(bool, A)>`: Looks up a variable in this scope
  and returns both the mutability flag and the stored value.
- `lookup_function(signature: &FuncSignature) -> Option<&Function>`: Looks up a
  function in this scope by its signature.
- `lookup_function_by_name(name: &Name) -> Option<&Function>`: Retrieves the
  most recent overload for a given name.
- `lookup_test(name: &Name) -> Option<&Function>`: Looks up a `test` definition
  in this scope.
- `lookup_adt(name: &Name) -> Option<&Arc<HashMap<Name, Vec<Type>>>>`: Resolves
  an ADT definition.

### Environment<A>

Manages a stack of scopes with lexical scoping rules.

```rust
pub struct Environment<A: Clone + Debug> {
    pub current_func: FuncSignature,
    pub globals: Scope<A>,
    pub stack: LinkedList<Scope<A>>,
}
```

#### Methods

- `new() -> Environment<A>`: Creates a new environment with empty global scope
- `get_current_func() -> FuncSignature`: Returns the signature of the function
  currently being type-checked or executed.
- `set_current_func(signature: &FuncSignature)`: Updates the current function
  signature (used by the type checker and interpreter when entering function
  bodies).
- `set_global_functions(functions: HashMap<FuncSignature, Function>)`: Replaces
  the global function table—handy when cloning environments for function calls.
- `map_variable(var: Name, mutable: bool, value: A)`: Maps a variable in the
  current scope.
- `map_function(function: Function)`: Maps a function in the current scope
  using its signature.
- `map_test(function: Function)`: Registers a `test` definition in the current
  scope.
- `map_adt(name: Name, constructors: HashMap<Name, Vec<Type>>)`: Registers an
  ADT definition.
- `lookup(var: &Name) -> Option<(bool, A)>`: Looks up a variable through the
  scope chain, returning both the mutability flag and the stored value.
- `lookup_function(signature: &FuncSignature) -> Option<&Function>`: Looks up a
  function through the scope chain using an exact signature match.
- `lookup_var_or_func(name: &Name) -> Option<FuncOrVar<A>>`: Convenience helper
  used by the interpreter and type checker to resolve whether an identifier
  refers to a variable or an overload set.
- `get_all_functions() -> HashMap<FuncSignature, Function>`: Aggregates all
  functions visible from the current environment, with inner scopes shadowing
  outer ones.
- `push()`: Creates a new scope at the top of the stack
- `pop()`: Removes the topmost scope
- `scoped_function() -> bool`: Checks if we're in a function scope

## Scoping Rules

1. **Variable Resolution**:
   - First checks the local scopes from innermost to outermost
   - Falls back to global scope if not found in any local scope
   - Returns None if the variable is not found anywhere

2. **Function Resolution**:
   - Resolved by `FuncSignature`, allowing different overloads for the same
     function name so long as the parameter types differ.
   - Follows the same lexical search strategy as variables.
   - Helper `lookup_function_by_name` (available through `lookup_var_or_func`)
     returns the innermost overload when only the name is known.

3. **Scope Management**:
   - New scopes are pushed when entering a function or block
   - Scopes are popped when exiting their block
   - Global scope always remains at the bottom

## Usage Examples

### Type Checking

```rust
let mut type_env: Environment<Type> = Environment::new();

// In global scope
type_env.map_variable("x".to_string(), Type::TInteger);

// In function scope
type_env.push();
type_env.map_variable("y".to_string(), Type::TReal);
```

### Interpretation

```rust
let mut runtime_env: Environment<Expression> = Environment::new();

// In global scope
runtime_env.map_variable("x".to_string(), Expression::CInt(42));

// In function scope
runtime_env.push();
runtime_env.map_variable("y".to_string(), Expression::CReal(3.14));
```

### Nested Scopes

```rust
let mut env: Environment<i32> = Environment::new();

// Global scope
env.map_variable("x".to_string(), 1);

// Outer function scope
env.push();
env.map_variable("y".to_string(), 2);

// Inner function scope
env.push();
env.map_variable("z".to_string(), 3);

// Variables from all scopes are accessible
assert!(env.lookup(&"x".to_string()).is_some()); // from global
assert!(env.lookup(&"y".to_string()).is_some()); // from outer
assert!(env.lookup(&"z".to_string()).is_some()); // from inner

// Pop scopes to clean up
env.pop(); // removes inner scope
env.pop(); // removes outer scope
```

## Implementation Notes

1. The environment uses a `LinkedList` for the scope stack to efficiently push/pop scopes
2. All lookups traverse the entire scope chain for proper lexical scoping
3. The generic parameter `A` must implement both `Clone` and `Debug` because
   environments frequently clone scoped values and log them during debugging.
4. Functions are keyed by `FuncSignature`, not by name, so callers should build
   the signature (name + argument types) before performing lookups.
5. `FuncOrVar` is a small enum returned by `lookup_var_or_func` to distinguish
   between variable and function bindings when only an identifier name is known.
6. The global scope is always accessible, regardless of the current scope depth.
7. Test definitions use an `IndexMap` so they can be iterated deterministically
   in the order they were declared.