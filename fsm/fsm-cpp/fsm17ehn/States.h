#pragma once

#include "Actions.h"
#include "Events.h"

#include "TransitionTo.h"

struct OpenState; // fwd dcl
struct LockedState;

struct ClosedState : public Will<
                         ByDefault<Nothing>,
                         On<LockEvent, TransitionTo<LockedState>>,
                         On<OpenEvent, TransitionTo<OpenState>>>
{
};

struct OpenState : public Will<
                       ByDefault<Nothing>,
                       On<CloseEvent, TransitionTo<ClosedState>>>
{
};

class LockedState : public ByDefault<Nothing>
{
public:
  using ByDefault::handle;

  LockedState(uint32_t key) : key{key} {}

  void onEnter(const LockEvent& e)
  {
    key = e.newKey;
  }

  Maybe<TransitionTo<ClosedState>> handle(const UnlockEvent& e)
  {
    if (e.key == key)
    {
      return TransitionTo<ClosedState>{};
    }
    return Nothing{};
  }

private:
  uint32_t key;
};
