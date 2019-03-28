//!
//! ## Quickstart
//!
//! Add `evalexpr` as dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! evalexpr = "2"
//! ```
//!
//! Add the `extern crate` definition to your `main.rs` or `lib.rs`:
//!
//! ```rust
//! extern crate evalexpr;
//! ```
//!
//! Then you can use `evalexpr` to **evaluate expressions** like this:
//!
//! ```rust
//! use evalexpr::*;
//!
//! assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
//! // `eval` returns a variant of the `Value` enum,
//! // while `eval_[type]` returns the respective type directly.
//! // Both can be used interchangeably.
//! assert_eq!(eval_int("1 + 2 + 3"), Ok(6));
//! assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
//! assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
//! assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
//! ```
//!
//! And you can use **variables** and **functions** in expressions like this:
//!
//! ```rust
//! use evalexpr::*;
//! use evalexpr::error::expect_number;
//!
//! let mut context = HashMapContext::new();
//! context.set_value("five".into(), 5.into()).unwrap(); // Do proper error handling here
//! context.set_value("twelve".into(), 12.into()).unwrap(); // Do proper error handling here
//! context.set_function("f".into(), Function::new(Some(1) /* argument amount */, Box::new(|arguments| {
//!     if let Value::Int(int) = arguments[0] {
//!         Ok(Value::Int(int / 2))
//!     } else if let Value::Float(float) = arguments[0] {
//!         Ok(Value::Float(float / 2.0))
//!     } else {
//!         Err(EvalexprError::expected_number(arguments[0].clone()))
//!     }
//! }))).unwrap(); // Do proper error handling here
//! context.set_function("avg".into(), Function::new(Some(2) /* argument amount */, Box::new(|arguments| {
//!     expect_number(&arguments[0])?;
//!     expect_number(&arguments[1])?;
//!
//!     if let (Value::Int(a), Value::Int(b)) = (&arguments[0], &arguments[1]) {
//!         Ok(Value::Int((a + b) / 2))
//!     } else {
//!         Ok(Value::Float((arguments[0].as_number()? + arguments[1].as_number()?) / 2.0))
//!     }
//! }))).unwrap(); // Do proper error handling here
//!
//! assert_eq!(eval_with_context("five + 8 > f(twelve)", &context), Ok(Value::from(true)));
//! // `eval_with_context` returns a variant of the `Value` enum,
//! // while `eval_[type]_with_context` returns the respective type directly.
//! // Both can be used interchangeably.
//! assert_eq!(eval_boolean_with_context("five + 8 > f(twelve)", &context), Ok(true));
//! assert_eq!(eval_with_context("avg(2, 4) == 3", &context), Ok(Value::from(true)));
//! ```
//!
//! You can also **precompile** expressions like this:
//!
//! ```rust
//! use evalexpr::*;
//!
//! let precompiled = build_operator_tree("a * b - c > 5").unwrap(); // Do proper error handling here
//!
//! let mut context = HashMapContext::new();
//! context.set_value("a".into(), 6.into()).unwrap(); // Do proper error handling here
//! context.set_value("b".into(), 2.into()).unwrap(); // Do proper error handling here
//! context.set_value("c".into(), 3.into()).unwrap(); // Do proper error handling here
//! assert_eq!(precompiled.eval_with_context(&context), Ok(Value::from(true)));
//!
//! context.set_value("c".into(), 8.into()).unwrap(); // Do proper error handling here
//! assert_eq!(precompiled.eval_with_context(&context), Ok(Value::from(false)));
//! // `Node::eval_with_context` returns a variant of the `Value` enum,
//! // while `Node::eval_[type]_with_context` returns the respective type directly.
//! // Both can be used interchangeably.
//! assert_eq!(precompiled.eval_boolean_with_context(&context), Ok(false));
//! ```
//!
//! ## Features
//!
//! ### Operators
//!
//! This crate offers a set of binary and unary operators for building expressions.
//! Operators have a precedence to determine their order of evaluation.
//! The precedence should resemble that of most common programming languages, especially Rust.
//! The precedence of variables and values is 200, and the precedence of function literals is 190.
//!
//! Supported binary operators:
//!
//! | Operator | Precedence | Description |   | Operator | Precedence | Description |
//! |----------|------------|-------------|---|----------|------------|-------------|
//! | + | 95 | Sum | | < | 80 | Lower than |
//! | - | 95 | Difference | | \> | 80 | Greater than |
//! | * | 100 | Product | | <= | 80 | Lower than or equal |
//! | / | 100 | Division | | \>= | 80 | Greater than or equal |
//! | % | 100 | Modulo | | == | 80 | Equal |
//! | ^ | 120 | Exponentiation | | != | 80 | Not equal |
//! | && | 75 | Logical and | | , | 40 | Aggregation |
//! | &#124;&#124; | 70 | Logical or | | | | |
//!
//! Supported unary operators:
//!
//! | Operator | Precedence | Description |
//! |----------|------------|-------------|
//! | - | 110 | Negation |
//! | ! | 110 | Logical not |
//!
//! #### The Aggregation Operator
//!
//! The aggregation operator aggregates two values into a tuple.
//! If one of the values is a tuple already, the resulting tuple will be flattened.
//! Example:
//!
//! ```rust
//! use evalexpr::*;
//!
//! assert_eq!(eval("1, 2, 3"), Ok(Value::from(vec![Value::from(1), Value::from(2), Value::from(3)])));
//! ```
//!
//! ### Builtin Functions
//!
//! This crate offers a set of builtin functions.
//!
//! | Identifier | Argument Amount | Description |
//! |------------|-----------------|-------------|
//! | min | >= 1 | Returns the minimum of the arguments |
//! | max | >= 1 | Returns the maximum of the arguments |
//!
//! The `min` and `max` functions can deal with a mixture of integer and floating point arguments.
//! They return the result as the type it was passed into the function.
//!
//! ### Values
//!
//! Operators take values as arguments and produce values as results.
//! Values can be boolean, integer or floating point numbers, tuples or the empty type.
//! Strings are supported as well, but there are no operations defined for them yet.
//! Values are denoted as displayed in the following table.
//!
//! | Value type | Example |
//! |------------|---------|
//! | `Value::Boolean` | `true`, `false` |
//! | `Value::Int` | `3`, `-9`, `0`, `135412` |
//! | `Value::Float` | `3.`, `.35`, `1.00`, `0.5`, `123.554` |
//! | `Value::Tuple` | `(3, 55.0, false, ())`, `(1, 2)` |
//! | `Value::Empty` | `()` |
//!
//! Integers are internally represented as `i64`, and floating point numbers are represented as `f64`.
//! Tuples are represented as `Vec<Value>` and empty values are not stored, but represented by rust's unit type `()` where necessary.
//!
//! There exist type aliases for some of the types.
//! They include `IntType`, `FloatType`, `TupleType` and `EmptyType`.
//!
//! Values can be constructed either directly or using the `From` trait.
//! Values can be decomposed using the `Value::as_[type]` methods.
//! The type of a value can be checked using the `Value::is_[type]` methods.
//!
//! **Examples for constructing a value:**
//!
//! | Code | Result |
//! |------|--------|
//! | `Value::from(4)` | `Value::Int(4)` |
//! | `Value::from(4.4)` | `Value::Float(4.4)` |
//! | `Value::from(true)` | `Value::Boolean(true)` |
//! | `Value::from(vec![Value::from(3)])` | `Value::Tuple(vec![Value::Int(3)])` |
//!
//! **Examples for deconstructing a value:**
//!
//! | Code | Result |
//! |------|--------|
//! | `Value::from(4).as_int()` | `Ok(4)` |
//! | `Value::from(4.4).as_float()` | `Ok(4.4)` |
//! | `Value::from(true).as_int()` | `Err(Error::ExpectedInt {actual: Value::Boolean(true)})` |
//!
//! Operators that take numbers as arguments can either take integers or floating point numbers.
//! If one of the arguments is a floating point number, all others are converted to floating point numbers as well, and the resulting value is a floating point number as well.
//! Otherwise, the result is an integer.
//! An exception to this is the exponentiation operator that always returns a floating point number.
//!
//! Values have a precedence of 200.
//!
//! ### Variables
//!
//! This crate allows to compile parameterizable formulas by using variables.
//! A variable is a literal in the formula, that does not contain whitespace or can be parsed as value.
//! The user needs to provide bindings to the variables for evaluation.
//! This is done with the `Context` trait.
//! Two structs implementing this trait are predefined.
//! There is `EmptyContext`, that returns `None` for each request, and `HashMapContext`, that stores mappings from literals to variables in a hash map.
//!
//! Variables do not have fixed types in the expression itself, but are typed by the context.
//! The `Context` trait contains a function that takes a string literal and returns a `Value` enum.
//! The variant of this enum decides the type on evaluation.
//!
//! Variables have a precedence of 200.
//!
//! ### User-Defined Functions
//!
//! This crate also allows to define arbitrary functions to be used in parsed expressions.
//! A function is defined as a `Function` instance.
//! It contains two properties, the `argument_amount` and the `function`.
//! The `function` is a boxed `Fn(&[Value]) -> EvalexprResult<Value, Error>`.
//! The `argument_amount` determines the length of the slice that is passed to `function` if it is `Some(_)`, otherwise the function is defined to take an arbitrary amount of arguments.
//! It is verified on execution by the crate and does not need to be verified by the `function`.
//!
//! Functions with no arguments are not allowed.
//! Use variables instead.
//!
//! Be aware that functions need to verify the types of values that are passed to them.
//! The `error` module contains some shortcuts for verification, and error types for passing a wrong value type.
//! Also, most numeric functions need to differentiate between being called with integers or floating point numbers, and act accordingly.
//!
//! Functions are identified by literals, like variables as well.
//! A literal identifies a function, if it is followed by an opening brace `(`, another literal, or a value.
//!
//! Same as variables, function bindings are provided by the user via a `Context`.
//! Functions have a precedence of 190.
//!
//! ### Examplary variables and functions in expressions:
//!
//! | Expression | Valid? | Explanation |
//! |------------|--------|-------------|
//! | `a` | yes | |
//! | `abc` | yes | |
//! | `a<b` | no | Expression is interpreted as variable `a`, operator `<` and variable `b` |
//! | `a b` | no | Expression is interpreted as function `a` applied to argument `b` |
//! | `123` | no | Expression is interpreted as `Value::Int` |
//! | `true` | no | Expression is interpreted as `Value::Bool` |
//! | `.34` | no | Expression is interpreted as `Value::Float` |
//!
//! ### [Serde](https://serde.rs)
//!
//! To use this crate with serde, the serde feature flag has to be set.
//! This can be done like this in the `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! evalexpr = {version = "2", features = ["serde"]}
//! ```
//!
//! This crate implements `serde::de::Deserialize` for its type `Node` that represents a parsed expression tree.
//! The implementation expects a [serde `string`](https://serde.rs/data-model.html) as input.
//! Example parsing with [ron format](docs.rs/ron):
//!
//! ```rust
//! extern crate ron;
//! use evalexpr::*;
//!
//! let mut context = HashMapContext::new();
//! context.set_value("five".into(), 5.into()).unwrap(); // Do proper error handling here
//!
//! // In ron format, strings are surrounded by "
//! let serialized_free = "\"five * five\"";
//! match ron::de::from_str::<Node>(serialized_free) {
//!     Ok(free) => assert_eq!(free.eval_with_context(&context), Ok(Value::from(25))),
//!     Err(error) => {
//!         () // Handle error
//!     },
//! }
//! ```
//!
//! With `serde`, expressions can be integrated into arbitrarily complex data.
//!
//! ## License
//!
//! This crate is primarily distributed under the terms of the MIT license.
//! See [LICENSE](LICENSE) for details.
//!

#![warn(missing_docs)]

#[cfg(test)]
extern crate ron;
#[cfg(feature = "serde")]
extern crate serde;

pub use context::{Context, EmptyContext, HashMapContext};
pub use error::{EvalexprError, EvalexprResult};
pub use function::Function;
pub use interface::*;
pub use tree::Node;
pub use value::{
    EMPTY_VALUE, EmptyType, FloatType, IntType, TupleType, Value, value_type::ValueType,
};

mod context;
pub mod error;
#[cfg(feature = "serde")]
mod feature_serde;
mod function;
mod interface;
mod operator;
mod token;
mod tree;
mod value;

// Exports
