#pragma once

#include <variant>

struct Nothing
{
  template <typename Machine, typename State, typename Event>
  void execute(Machine&, State&, const Event&) {}
};

template <typename Action>
struct ByDefault
{
  template <typename Event>
  Action handle(const Event&) const
  {
    return {};
  }
};

template <typename Event, typename Action>
struct On
{
  Action handle(const Event&) const
  {
    return {};
  }
};

template <typename... Handlers>
struct Will : Handlers...
{
  using Handlers::handle...;
};

template <typename... Actions>
class OneOf
{
public:
  template <typename T>
  OneOf(T&& arg) : options(std::forward<T>(arg)) {}

  template <typename Machine, typename State, typename Event>
  void execute(Machine& machine, State& state, const Event& event)
  {
    std::visit(
        [&machine, &state, &event](auto& action)
        { action.execute(machine, state, event); },
        options);
  }

private:
  std::variant<Actions...> options;
};

template <typename Action>
struct Maybe : public OneOf<Action, Nothing>
{
  using OneOf<Action, Nothing>::OneOf;
};