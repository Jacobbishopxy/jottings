#pragma once

#include "Machine.h"
#include <exception>
#include <stdexcept>

class Machine; // fwd dcl

/**
 * An abstract class used for concrete states to be derived from.
 *
 * Notice that `sell` and `refill` are pure virtual functions,
 * and `damage`/`fix` have default implementation.
 *
 * `setState` is the actual function that changes the state (using friend class in `Machine`),
 * which is designed for `sell`/`refill`/`damage`/`fix` callings (events change states).
 *
 * `updateStock` is used for recording data, which will be called by event functions.
 */
class AbstractState
{
public:
  virtual void sell(Machine& machine, unsigned int quantity) = 0;
  virtual void refill(Machine& machine, unsigned int quantity) = 0;
  virtual void damage(Machine& machine);
  virtual void fix(Machine& machine);
  virtual ~AbstractState();

protected:
  void setState(Machine& machine, AbstractState* st);
  void updateStock(Machine& machine, unsigned int quantity);
};

// Notice there is no `damage` function, since it is implemented in `AbstractState`,
// and `fix` in here is used for throwing exceptions because it is not in `Broken` state.
class Normal : public AbstractState
{
public:
  virtual void sell(Machine& machine, unsigned int quantity);
  virtual void refill(Machine& machine, unsigned int quantity);
  virtual void fix(Machine& machine);
  virtual ~Normal();
};

// Notice there is no `damage` function, since it is implemented in `AbstractState`,
// and `fix` in here is used for throwing exceptions because it is not in `Broken` state.
class SoldOut : public AbstractState
{
public:
  virtual void sell(Machine& machine, unsigned int quantity);
  virtual void refill(Machine& machine, unsigned int quantity);
  virtual void fix(Machine& machine);
  virtual ~SoldOut();
};

// Notice there is no `damage`/`fix` function, since it is implemented in `AbstractState`
class Broken : public AbstractState
{
public:
  virtual void sell(Machine& machine, unsigned int quantity);
  virtual void refill(Machine& machine, unsigned int quantity);
  virtual ~Broken();
};
