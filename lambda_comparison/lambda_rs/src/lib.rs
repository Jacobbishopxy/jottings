//! Lambda Comparison
//!
//! Rust version

/**
 * function pointer type
 *
 * Unlike C++, no need to explicit declare a pointer with function type.
 * Instead, a named variable with function pointer type is a pointer variable.
 *
 * Identically, C++'s function pointer only works for non-capturing lambda,
 * and Rust function pointer type can be created non-capturing closures.
 */
fn lambda_and_fn_ptr() {
    let mul = |i: i32| i * 2;

    type M = fn(i32) -> i32;

    let lbd: M = mul;

    println!("{:?}", lbd(2));
    println!("{:?}", lbd(3));
}

/**
 * simplest case
 *
 * type deduction: |i32| -> i32
 */
fn simple_lambda1() {
    let lbd = |i: i32| i * 3;

    println!("{:?}", lbd(2));
}

/**
 * passing function pointer or closure
 */
fn passing_lambda_to_fn(f: fn(i32) -> i32) {
    let input = 2;

    println!("{:?}", f(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lambda_and_fn_ptr_success() {
        lambda_and_fn_ptr();
    }

    #[test]
    fn simple_lambda1_success() {
        simple_lambda1();
    }

    #[test]
    fn passing_lambda_to_fn_success() {
        passing_lambda_to_fn(|i| i * 2);
    }
}
