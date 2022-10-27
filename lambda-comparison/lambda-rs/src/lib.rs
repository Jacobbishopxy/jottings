//! Lambda Comparison
//!
//! Rust version

use std::fmt::Debug;

/**
 * function pointer type
 *
 * Unlike C++, no need to explicit declare a pointer with function type.
 * Instead, a named variable with function pointer type is a pointer variable.
 *
 * Identically, C++'s function pointer only works for non-capturing lambda,
 * and Rust function pointer type can be created non-capturing closures.
 */
#[allow(dead_code)]
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
#[allow(dead_code)]
fn simple_lambda1() {
    let lbd = |i: i32| i * 3;

    println!("{:?}", lbd(2));
}

/**
 * passing function pointer or closure
 */
#[allow(dead_code)]
fn passing_lambda_to_fn(f: fn(i32) -> i32) {
    let input = 2;

    println!("{:?}", f(input));
}

/**
 * generic lambda parameter
 */
#[allow(dead_code)]
fn generic_lambda<T: Debug>(value: T) {
    let lbd = |v| {
        println!("value: {v:?}");
    };

    lbd(value);
}

/**
 * acting like C++, a normal capture has cloned variable
 */
#[allow(dead_code)]
fn simple_capture1() {
    let data = String::from("halloween has come!");

    let rise_headless_horseman = |s| {
        println!("{data:?} {s:?}",);
    };

    const GREET: &str = "Prepare yourselves, the bells have tolled! Shelter your weak, your young and your old! Each of you shall pay the final sum. Cry for mercy, the reckoning has come!";

    rise_headless_horseman(GREET);

    let _check_ownership = data.into_bytes();
}

/**
 * The difference between `simple_capture1` & `simple_capture2` is the former `data` is a `String`,
 * who has `Clone` trait implemented (see `simple_capture3` where `Clone` is impl, manually).
 */
#[allow(dead_code)]
fn simple_capture2() {
    struct MyStr(String);

    impl MyStr {
        fn turn(self) -> String {
            self.0
        }
    }

    let data = MyStr(String::from("halloween coming soon"));

    let prepare_pumpkins = |s| {
        println!("{:?} -> {s}", data.0);
    };

    const G: &str = "Carving pumpkins";

    prepare_pumpkins(G);

    // compile error here! since data has been moved into `prepare_pumpkins`
    // let _check_ownership = data.turn();
}

#[allow(dead_code)]
fn simple_capture3() {
    #[derive(Clone)]
    struct MyStr(String);

    impl MyStr {
        fn turn(self) -> String {
            self.0
        }
    }

    let data = MyStr(String::from("halloween coming soon"));

    let prepare_pumpkins = |s| {
        println!("{:?} -> {s}", data.0);
    };

    const G: &str = "Carving pumpkins";

    prepare_pumpkins(G);

    // Ownership hasn't been take until now.
    // Clone has been made while compiling `prepare_pumpkins` on account of `#[derive(Clone)]`
    let _check_ownership = data.turn();
}

#[allow(dead_code)]
fn mutable_capture1() {
    let mut quiver = 8;

    let mut shoot = || {
        // let q = &mut quiver;
        quiver -= 1;
        println!("Arrows left {quiver:?}");
    };

    shoot();
    shoot();
    shoot();

    println!("Check the rest: {quiver:?}");
}

#[allow(dead_code)]
fn mutable_capture2() {
    struct Quiver(i32);

    impl Quiver {
        fn shoot(&mut self) {
            self.0 -= 1;
        }

        fn check(&self) -> i32 {
            self.0
        }
    }

    let mut quiver = Quiver(8);

    let mut shoot = || {
        // let q = &mut quiver;
        // q.shoot();
        quiver.shoot();
        println!("Arrows left {:?}", quiver.check());
    };

    shoot();
    shoot();
    shoot();

    println!("Check the rest: {:?}", quiver.check());
}

/**
 * Capture by ownership
 *
 * Note: acts like a `static` variable in a C++'s lambda function.
 *
 * The difference between `ownership_capture1` & `ownership_capture2` is
 * the former `quiver` variable is a primitive type, which
 */
#[allow(dead_code)]
fn ownership_capture1() {
    let mut quiver = 8;

    let mut shoot = move || {
        quiver -= 1;
        println!("Arrows left {quiver:?}");
    };

    shoot();
    shoot();
    shoot();

    // ⚠️ watch out! this still prints `8`, because all the primitive type in Rust has `Copy` implemented!
    println!("Check the rest {quiver:?}");
}

#[allow(dead_code)]
fn ownership_capture2() {
    struct Quiver(i32);

    impl Quiver {
        fn shoot(&mut self) {
            self.0 -= 1;
        }

        fn check(&self) -> i32 {
            self.0
        }
    }

    let mut quiver = Quiver(8);

    let mut shoot = move || {
        quiver.shoot();
        println!("Arrows left {:?}", quiver.check());
    };

    shoot();
    shoot();
    shoot();

    // compile error here! Since `quiver` has been captured by ownership,
    // which in other words `quiver` is owned by the closure.
    // println!("Check the rest {:?}", quiver.check());
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

    #[test]
    fn generic_lambda_success() {
        generic_lambda("foo");
        generic_lambda(233);
    }

    #[test]
    fn simple_capture_success() {
        simple_capture1();
        simple_capture2();
        simple_capture3();
    }

    #[test]
    fn mutable_capture_success() {
        mutable_capture1();
        mutable_capture2();
    }

    #[test]
    fn ownership_capture_success() {
        ownership_capture1();
    }
}
