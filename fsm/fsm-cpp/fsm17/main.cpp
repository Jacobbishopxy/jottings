#include "StateMachine.h"
#include "StatesAndEvents.h"

using Door = StateMachine<ClosedState, OpenState>;

int main(int argc, char const* argv[])
{
  Door door;

  door.handle(OpenEvent{});
  door.handle(CloseEvent{});

  door.handle(CloseEvent{});
  door.handle(OpenEvent{});

  return 0;
}
