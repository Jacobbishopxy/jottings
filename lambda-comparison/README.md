# Lambda Comparison

A simple comparison of C-plus-plus' lambda expression and Rust's closure.

- [lambda-cpp](./lambda-cpp/main.cpp)

- [lambda-rs](./lambda-rs/src/lib.rs)

## Cpp

- lambda_and_fn_ptr: rather than using a function pointer to hold a non-capturing lambda (as its odd syntax), prefer using list initialization with lambda syntax

- simple_lambda: non-capturing lambda

- passing_lambda_to_fn

- generic_lambda

- simple_capture

- mutable_capture

- reference_capture

- mixing_capture

- default_value_capture

- default_reference_capture

- default_mixing_capture

- init_var_capture

- copy_lambda

- copy_ref_lambda

## Rust

- lambda_and_fn_ptr

- simple_lambda: non-capturing closure

- passing_lambda_to_fn

- generic_lambda

- simple_capture: capture by value when `copy`/`clone` has implemented, otherwise capture by using move semantics

- mutable_capture

- reference_capture: unlike C++ reference, a borrowed variable cannot be mutable unless by using `Rc<RefCell<T>>` and etc.

- ownership_capture: move semantics
