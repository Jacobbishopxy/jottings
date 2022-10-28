/**
 * @file main.cpp
 * @author JacobXie (jacobbishopxy@gmail.com)
 * @brief
 * @version 0.1
 * @date 2022-10-27
 *
 * @copyright Copyright (c) 2022
 *
 * Compiler version: C++20.
 */

#include <functional>
#include <iostream>
#include <string>

/**
 * @brief a function pointer pointed to a lambda expression
 *
 * Note: only works for non-capturing lambda.
 */
void lambda_and_fn_ptr() {
  int (*lbd)(int){[](int i) { return i * 2; }};

  std::cout << lbd(2) << std::endl;
  std::cout << lbd(3) << std::endl;
}

/**
 * @brief simplest case
 *
 * non-capturing
 */
void simple_lambda1() {
  auto lbd{[](int i) { return i * 3; }};

  std::cout << lbd(2) << std::endl;
}

/**
 * @brief simplest case
 *
 * Instead of using auto, type deduction `std::function`.
 * Note: prior C++17, use `std::function<int(int)>` instead.
 */
void simple_lambda2() {
  std::function lbd{[](int i) { return i * 4; }};

  std::cout << lbd(2) << std::endl;
}

/**
 * @brief passing function
 *
 * @param fn a normal function or a lambda expression
 */
void passing_lambda_to_fn(const std::function<int(int)>& fn) {
  constexpr int input = 2;

  std::cout << fn(input) << std::endl;
}

/**
 * @brief generic lambda parameter
 *
 * @param value a generic type, decided at compile time
 */
void generic_lambda0(auto value) {
  auto print{[](auto v) { std::cout << "value: " << v << std::endl; }};

  print(value);
}

/**
 * @brief function template
 *
 * @param value a generic type, decided at compile time
 */
template <typename T> void generic_lambda1(T value) {
  auto print{[](T v) { std::cout << "value: " << v << std::endl; }};

  print(value);
}

/**
 * @brief simple capture, ⚠️ key point: Clone
 *
 * `data` has been cloned when `rise_headless_horseman` defined.
 * By default, variable is captured by `const`, thus it is immutable.
 */
void simple_capture() {
  std::string data{"halloween has come!"};

  auto rise_headless_horseman{[data](std::string_view str) { std::cout << data << " " << str; }};

  const std::string_view greet{
      "Prepare yourselves, the bells have tolled! Shelter your weak, your young and your old! Each "
      "of you shall pay the final sum. Cry for mercy, the reckoning has come!"};

  rise_headless_horseman(greet);
}

/**
 * @brief mutable capture, ⚠️ key point: mutable for all captured variables
 *
 * `mutable` keyword, remove the default `const` capturing limitation,
 * whereas it exposes potential risks as well (all captured variables turn into mutable).
 */
void mutable_capture() {
  int quiver{8};

  auto shoot{[quiver]() mutable {
    --quiver;
    std::cout << "Arrows left: " << quiver << std::endl;
  }};

  shoot();
  shoot();
  shoot();
}

/**
 * @brief reference capture, ⚠️ key point: Reference
 */
void reference_capture() {
  int magazine{120};

  auto fire{[&magazine]() {
    magazine -= 10;
    std::cout << "Bullets left: " << magazine << std::endl;
  }};

  fire();
  fire();
  fire();
}

/**
 * @brief static variable inside a lambda
 *
 * Note: acts like a Rust's closure who prefixes a `move` keyword.
 */
void simple_capture_with_static_var() {

  auto shoot{[]() {
    static int quiver{8};

    --quiver;
    std::cout << "Arrows left: " << quiver << std::endl;
  }};

  shoot();
  shoot();
  shoot();
}

/**
 * @brief mixing capture by reference and by value
 *
 * Note: still, `static` is used in lambdas, not a good idea, but for illustration
 */
void mixing_capture() {
  // capture by reference, by this means, players aiming one single target
  int boss{100};
  // capture by value, by this means, represents different players
  int player{10};

  auto melee{[&boss, player](bool offensive) {
    static int p{player};
    char direction{'>'};
    if (offensive) {
      boss -= 10;
      direction = '<';
    } else {
      p -= 1;
      direction = '>';
    }

    std::cout << "boss[ " << boss << " ] " << direction << " melee[ " << p << " ]" << std::endl;
  }};

  auto range{[&boss, player](bool offensive) {
    static int p{player};
    char direction{'>'};
    if (offensive) {
      boss -= 20;
      direction = '<';
    } else {
      p -= 3;
      direction = '>';
    }

    std::cout << "boss[ " << boss << " ]  " << direction << "  range[ " << p << " ]" << std::endl;
  }};

  melee(false);
  melee(false);
  range(true);
  range(true);
  melee(true);
  range(false);
}

// TODO:
// default capture
// new vars in capture list
// lambda's copy

int main() {

  // lambda_and_fn_ptr();
  // simple_lambda1();
  // simple_lambda2();

  // passing_lambda_to_fn([](int i) { return i * 2; });

  // generic_lambda0("foo");
  // generic_lambda0(233);
  // generic_lambda1("foo");
  // generic_lambda1(233);

  // simple_capture();
  // mutable_capture();
  // reference_capture();
  // simple_capture_with_static_var();
  mixing_capture();

  return 0;
}
