#pragma once

#include <stdint.h>

struct OpenEvent
{
};

struct CloseEvent
{
};

struct LockEvent
{
  uint32_t newKey;
};

struct UnlockEvent
{
  uint32_t key;
};
