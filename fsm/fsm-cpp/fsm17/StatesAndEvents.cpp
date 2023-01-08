#include "StatesAndEvents.h"
#include <iostream>

TransitionTo<OpenState> ClosedState::handle(const OpenEvent&) const
{
  std::cout << "Opening the door..." << std::endl;
  return {};
}

Nothing ClosedState::handle(const CloseEvent&) const
{
  std::cout << "Cannot close. The door is already closed!" << std::endl;
  return {};
}

TransitionTo<ClosedState> OpenState::handle(const CloseEvent&) const
{
  std::cout << "Cannot open. The door is already open!" << std::endl;

  return {};
}

Nothing OpenState::handle(const OpenEvent&) const
{
  std::cout << "Closing the door..." << std::endl;
  return {};
}
