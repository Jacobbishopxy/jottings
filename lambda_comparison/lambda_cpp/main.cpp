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

int main() {

  // lambda_and_fn_ptr();
  // simple_lambda1();
  // simple_lambda2();

  passing_lambda_to_fn([](int i) { return i * 2; });

  return 0;
}
