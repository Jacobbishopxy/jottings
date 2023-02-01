/**
 * @file:	OrderAD.h
 * @author:	Jacob Xie
 * @date:	2023/02/01 19:02:45 Wednesday
 * @brief:
 **/

#ifndef __ORDERAD__H__
#define __ORDERAD__H__

#include <map>
#include <set>
#include <string>
#include <tuple>

using OrderIdType = int;
using SymbolId = int;
using MinuteId = int;
using SiMapping = std::map<int, std::string>;

using OrderIdSet = std::set<OrderIdType>;

struct OrderAD
{
  OrderIdSet order_A;
  OrderIdSet order_D;

  OrderAD(OrderIdSet a, OrderIdSet d) : order_A{a}, order_D{d} {};

  void insert_A(OrderIdType oa);
  void insert_D(OrderIdType od);
  void remove_A(OrderIdType oa);
  void remove_D(OrderIdType od);
  bool check_A(OrderIdType oa);
  bool check_D(OrderIdType od);

  const OrderIdSet& get_order_A() const;
  const OrderIdSet& get_order_D() const;

  const std::tuple<size_t, size_t> size() const; // a_size, d_size
  const bool is_empty() const;
};

#endif //!__ORDERAD__H__
