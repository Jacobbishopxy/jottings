
template <typename State>
struct TransitionTo
{
  template <typename Machine>
  void execute(Machine& machine)
  {
    machine.template transitionTo<State>();
  }
};
