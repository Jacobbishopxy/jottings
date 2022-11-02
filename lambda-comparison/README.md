# Lambda Comparison

A simple comparison of C-plus-plus' lambda expression and Rust's closure.

- [lambda-cpp](./lambda-cpp/main.cpp)

- [lambda-rs](./lambda-rs/src/lib.rs)

## Cpp

- lambda_and_fn_ptr: rather than using a function pointer to hold a non-capturing lambda (as its odd syntax), prefer using list initialization with lambda syntax

- simple_lambda: non-capturing lambda

- passing_lambda_to_fn: passing a function pointer

- generic_lambda: using auto type or function template

- simple_capture: value (clone) capture

- mutable_capture: value capture with keyword `mutable`. Notice that all variables are now mutable, and those variables who was captured by value turns out to be lambda's internal state (`static` variable effectively identical)

- reference_capture

- ownership_capture

- mixing_capture: mixing value capture and reference capture

- default_value_capture: `=` identifier

- default_reference_capture: `&` identifier

- default_mixing_capture: `=` or `&` at the first element of capture list

- init_var_capture: variable initialization in the capture list

- copy_lambda: lambda is an object who plays a functor's role, and any clone made on a lambda would clone its internal states as well.

- copy_ref_lambda: an elegant method of passing a lambda's reference by using `std::ref`

## Rust

- lambda_and_fn_ptr

- simple_lambda: non-capturing closure

- passing_lambda_to_fn

- generic_lambda

- simple_capture: capture by value when `copy`/`clone` has implemented, otherwise capture by using move semantics

- mutable_capture

- reference_capture: unlike C++ reference, a borrowed variable cannot be mutable unless by using `Rc<RefCell<T>>` and etc.

- ownership_capture: move semantics
