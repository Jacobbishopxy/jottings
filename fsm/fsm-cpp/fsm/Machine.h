#pragma once

#include "States.h"

class AbstractState; // fwd dcl

/**
 * The actual class will be instantiated, which used for handling real event calling,
 * recording data (stock), and maintaining states switching.
 *
 * Notice that `AbstractState` is a friend class of a `Machine`, and this is required by
 * the event functions (`sell`/`refill`/`damage`/`fix`) to actually do the state switching
 * (see the implementation).
 */
class Machine
{
  friend class AbstractState;

public:
  Machine(unsigned int _stock);
  ~Machine();

  void sell(unsigned int quantity);
  void refill(unsigned int quantity);
  void damage();
  void fix();

  unsigned int getStock();

private:
  unsigned int stock;
  AbstractState* state;
};
