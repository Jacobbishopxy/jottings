#pragma once

#include "MachineStates.h"

class AbstractState;

class Machine
{
  friend class AbstractState;

public:
  Machine(unsigned int _stock);
  ~Machine();

  void sell(unsigned int quantity);
  void refill(unsigned int quantity);
  unsigned int getStock();

private:
  unsigned int stock;
  AbstractState* state;
};
