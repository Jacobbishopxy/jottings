#include "OrderAD.h"
#include <iostream>

void OrderAD::insert_A(OrderIdType oa)
{
  order_A.insert(oa);
}

void OrderAD::insert_D(OrderIdType od)
{
  order_D.insert(od);
}

void OrderAD::remove_A(OrderIdType oa)
{
  order_A.erase(oa);
}

void OrderAD::remove_D(OrderIdType od)
{
  order_D.erase(od);
}

bool OrderAD::check_A(OrderIdType oa)
{
  return order_A.find(oa) != order_A.end();
}

bool OrderAD::check_D(OrderIdType od)
{
  return order_D.find(od) != order_D.end();
}

const OrderIdSet& OrderAD::get_order_A() const
{
  return order_A;
}

const OrderIdSet& OrderAD::get_order_D() const
{
  return order_D;
}

const std::tuple<size_t, size_t> OrderAD::size() const
{
  return {order_A.size(), order_D.size()};
}

const bool OrderAD::is_empty() const
{
  return order_A.empty() && order_D.empty();
}
