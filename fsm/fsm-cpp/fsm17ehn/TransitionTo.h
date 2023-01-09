#pragma once

template <typename TargetState>
struct TransitionTo
{
  template <typename Machine, typename State, typename Event>
  void execute(Machine& machine, State& prevState, const Event& event)
  {
    leave(prevState, event);
    // Note: `.template` is telling `transitionTo` is in this template
    TargetState& newState = machine.template transitionTo<TargetState>();
    enter(newState, event);
  }

private:
  // Note: C-style params
  void leave(...) {}

  template <typename State, typename Event>
  auto leave(State& state, const Event& event) -> decltype(state.onLeave(event))
  {
    return state.onLeave(event);
  }

  // Note: C-style params
  void enter(...) {}

  template <typename State, typename Event>
  auto enter(State& state, const Event& event) -> decltype(state.onEnter(event))
  {
    return state.onEnter(event);
  }
};
