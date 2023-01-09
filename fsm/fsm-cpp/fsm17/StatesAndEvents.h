#pragma once

#include "TransitionTo.h"

struct Nothing
{
  template <typename Machine>
  void execute(Machine&) {}
};

struct OpenEvent
{
};

struct CloseEvent
{
};

struct OpenState; // fwd dcl

struct ClosedState
{
  TransitionTo<OpenState> handle(const OpenEvent&) const;

  Nothing handle(const CloseEvent&) const;
};
struct OpenState
{
  TransitionTo<ClosedState> handle(const CloseEvent&) const;

  Nothing handle(const OpenEvent&) const;
};
