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

/**
 * @brief capture by value without explicit declaring captured variables
 */
void default_value_capture() {
  int player{10};

  auto lbd{[=]() {
    // buffed by a holy priest
    std::cout << "Player get buffed: " << player + 5 << std::endl;
  }};

  lbd();

  std::cout << "The original player: " << player << std::endl;
}

/**
 * @brief capture by reference without explicit declaring captured variables
 */
void default_reference_capture() {
  int boss{100};

  auto lbd{[&]() {
    // attacked by a hunter's aimed shot
    boss -= 20;
    std::cout << "Boss get hit: " << boss << std::endl;
  }};

  lbd();

  std::cout << "The original boss: " << boss << std::endl;
}

/**
 * @brief mixing default capture
 *
 * value capture a & b, reference capture c
 * [a, b, &c](){};
 *
 * reference capture c, value capture the rest
 * [=, &c](){};
 *
 * value capture a, reference capture the rest
 * [&, a](){};
 *
 * ⚠️ illegal, already reference captured all
 * [&, &c](){};
 *
 * ⚠️ illegal, already value captured all
 * [=, a](){};
 *
 * ⚠️ illegal, captured a twice
 * [a, &b, &a](){};
 *
 * ⚠️ illegal, default capture should always at the first
 * [armor, &](){};
 *
 */
void default_mixing_capture() {
  int boss_health{100};
  int player_health{10};

  const int boss_dmg{3};
  const int player_dmg{1};

  // boss -> player
  auto boss_player{[=, &player_health]() {
    player_health -= boss_dmg;
    std::cout << "The boss has made " << boss_dmg << " to the player, and the player has left " << player_health
              << " health!" << std::endl;
  }};

  // player -> boss
  auto player_boss{[&, player_dmg]() {
    boss_health -= player_dmg;
    std::cout << "The play has made " << player_dmg << " to the boss, and the boss has left " << boss_health
              << " health!" << std::endl;
  }};

  boss_player();
  player_boss();
  player_boss();
  player_boss();
  boss_player();
  player_boss();
}

/**
 * @brief capture with initializers
 */
void init_var_capture() {
  int life{3};

  // &o is initialzed as life's reference
  // d is initialzed from cloned life
  auto double_life{[&o = life, d{life * 2}]() {
    o *= 2;
    std::cout << "1. Doubled life is " << o << std::endl;
    std::cout << "2. Doubled life is " << d << std::endl;
  }};

  // print 6, 6
  double_life();
  // print 12, 6
  // this is because `d{life * 2}` only initialzed once (when the first call is defined)
  double_life();

  // 12, no doubt `o *= 2;` has been called twice
  std::cout << "The original life has been changed to: " << life << std::endl;
}

/**
 * @brief lambda is an object, which can be copy and modified
 *
 * Although a `mutable` keyword has been used in here, `stage` is still
 * captured by value (cloned). Hence, `stage` turns into a stateful variable
 * who has been stored inside the lambda. It turns out, whenever a clone has
 * been made upon `step_forward`, its state (`stage`) would have been cloned
 * as well.
 */
void copy_lambda() {
  int stage{0};

  auto step_forward{[stage]() mutable {
    stage++;
    std::cout << "step forward: " << stage << std::endl;
  }};

  step_forward(); // stage -> 1

  auto another_sf{step_forward}; // stage: 1

  step_forward(); // stage -> 2
  another_sf();   // stage -> 2

  // no doubt, stage is still `0` right here
  std::cout << "final stage: " << stage << std::endl;
}

// `auto&` means type deduction of the argument must be a reference
// by the later call, it turns out to be `std::function<void()>&`
void copy_invoke(const auto& fn) { fn(); }

/**
 * @brief copy lambda by reference
 *
 * type `std::reference_wrapper` created by `std::ref`
 */
void copy_ref_lambda() {
  int stage{0};

  auto step_forward{[stage]() mutable { std::cout << ++stage << std::endl; }};

  copy_invoke(std::ref(step_forward)); // stage -> 1
  copy_invoke(std::ref(step_forward)); // stage -> 2
  copy_invoke(std::ref(step_forward)); // stage -> 3
}

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
  // mixing_capture();

  // default_value_capture();
  // default_reference_capture();
  // default_mixing_capture();

  // init_var_capture();
  // copy_lambda();
  copy_ref_lambda();

  return 0;
}
