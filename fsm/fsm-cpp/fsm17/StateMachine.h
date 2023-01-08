#pragma once

#include <functional>
#include <tuple>
#include <variant>

template <typename... States>
class StateMachine
{
public:
  template <typename State>
  void transitionTo()
  {
    currentState = &std::get<State>(states);
  }

  template <typename Event>
  void handle(const Event& event)
  {
    auto passEventToState = [this, &event](auto statePtr)
    {
      statePtr->handle(event).execute(*this);
    };

    std::visit(passEventToState, currentState);
  }

private:
  std::tuple<States...> states;
  std::variant<States*...> currentState{&std::get<0>(states)};
};
