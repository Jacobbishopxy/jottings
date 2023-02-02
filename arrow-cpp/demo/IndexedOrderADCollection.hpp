/**
 * @file:	IndexedOrderADCollection.hpp
 * @author:	Jacob Xie
 * @date:	2023/02/01 19:04:29 Wednesday
 * @brief:
 **/

#ifndef __INDEXEDORDERADCOLLECTION__H__
#define __INDEXEDORDERADCOLLECTION__H__

#include <array>
#include <vector>

#include "OrderAD.h"

// Mi -> OrderAD
template <unsigned int N>
using IndexedOrderAD = std::array<OrderAD, N>; // N 分钟

// Si -> Mi -> OrderAD
template <unsigned int N>
using IndexedOrderData = std::vector<IndexedOrderAD<N>>;

// fwd dcl
template <unsigned int N>
struct IndexedOrderADCollection;

// fwd dcl
template <unsigned int N>
std::ostream& operator<<(std::ostream& os, const IndexedOrderADCollection<N>& o);

// ================================================================================================
// IndexedOrderADCollection
// ================================================================================================

template <unsigned int N>
struct IndexedOrderADCollection
{
  IndexedOrderData<N> m_data;

  IndexedOrderADCollection(){};
  IndexedOrderADCollection(int symbol_size)
  {
    m_data.reserve(symbol_size);
  };

  void resize(int symbol_size)
  {
    m_data.resize(symbol_size);
  }

  void insert_A(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return;
    m_data[si][mi].insert_A(oid);
  }
  void insert_D(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return;
    m_data[si][mi].insert_D(oid);
  }
  void remove_A(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return;
    m_data[si][mi].remove_A(oid);
  }
  void remove_D(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return;
    m_data[si][mi].remove_D(oid);
  }
  void swap_A(SymbolId si, MinuteId prev_mi, MinuteId curr_mi, OrderIdType oid)
  {
    if (si < 0 || prev_mi < 0 || curr_mi < 0)
      return;
    remove_A(si, prev_mi, oid);
    insert_A(si, curr_mi, oid);
  }
  void swap_D(SymbolId si, MinuteId prev_mi, MinuteId curr_mi, OrderIdType oid)
  {
    if (si < 0 || prev_mi < 0 || curr_mi < 0)
      return;
    remove_D(si, prev_mi, oid);
    insert_D(si, curr_mi, oid);
  }

  const OrderIdSet* get_order_A(SymbolId si, MinuteId mi) const
  {
    if (si < 0 || mi < 0)
      return nullptr;
    return &m_data[si][mi].get_order_A();
  }
  const OrderIdSet* get_order_D(SymbolId si, MinuteId mi) const
  {
    if (si < 0 || mi < 0)
      return nullptr;
    return &m_data[si][mi].get_order_D();
  }

  bool check_A(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return false;
    return m_data[si][mi].check_A(oid);
  }
  bool check_D(SymbolId si, MinuteId mi, OrderIdType oid)
  {
    if (si < 0 || mi < 0)
      return false;
    return m_data[si][mi].check_D(oid);
  }
  MinuteId search_A(SymbolId si, OrderIdType oid)
  {
    if (si < 0)
      return -1;
    for (int mi = 0; mi < m_data[si].size(); ++mi)
    {
      if (m_data[si][mi].check_A(oid))
        return mi;
    }

    return -1;
  }
  MinuteId search_D(SymbolId si, OrderIdType oid)
  {
    if (si < 0)
      return -1;
    for (int mi = 0; mi < m_data[si].size(); ++mi)
    {
      if (m_data[si][mi].check_D(oid))
        return mi;
    }

    return -1;
  }

  const bool is_empty(size_t idx) const
  {
    // watch out the boundary issue
    IndexedOrderAD<N> s_ad = m_data[idx];
    return std::all_of(
        s_ad.begin(), s_ad.end(), [](auto e)
        { return e.is_empty(); }
    );
  }
  const size_t len() const
  {
    return m_data.size();
  }
};

template <unsigned int N>
std::ostream& operator<<(std::ostream& os, const IndexedOrderADCollection<N>& o)
{
  for (auto& si : o.m_data)
  {
    for (auto& mi_oad : si)
    {
      os << "{" << mi_oad << "}";
    }
    os << std::endl;
  }

  return os;
}

#endif //!__INDEXEDORDERADCOLLECTION__H__
