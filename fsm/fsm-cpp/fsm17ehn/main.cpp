#include "Events.h"
#include "StateMachine.h"
#include "States.h"

using Door = StateMachine<ClosedState, OpenState, LockedState>;

int main(int argc, char const* argv[])
{
  Door door{ClosedState{}, OpenState{}, LockedState{0}};

  door.handle(LockEvent{1234});
  door.handle(UnlockEvent{2});
  door.handle(UnlockEvent{1234});

  return 0;
}
