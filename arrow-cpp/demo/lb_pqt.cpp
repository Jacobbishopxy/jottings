/**
 * @file:	lb_pqt.cpp
 * @author:	Jacob Xie
 * @date:	2023/02/01 19:50:46 Wednesday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/dataset/api.h>
#include <iostream>
#include <parquet/arrow/reader.h>
#include <parquet/arrow/writer.h>

#include "IndexedOrderADCollection.hpp"
#include "OrderAD.h"

const auto schema = arrow::schema({
    arrow::field("symbol", arrow::utf8()),
    arrow::field("mi", arrow::int64()),
    arrow::field("a", arrow::list(arrow::int64())),
    arrow::field("d", arrow::list(arrow::int64())),
});

template <unsigned int N>
arrow::Result<std::shared_ptr<arrow::Table>> create_table(
    const SiMapping& si_map,
    const IndexedOrderADCollection<N>& coll
)
{
  arrow::MemoryPool* pool = arrow::default_memory_pool();

  // symbol 字段
  arrow::StringBuilder symbol_b(pool);

  // mi 字段
  arrow::Int64Builder mi_b(pool);

  // a 字段
  arrow::ListBuilder order_A_b(pool, std::make_shared<arrow::Int64Builder>(pool));
  arrow::Int64Builder& order_A_value_b = *static_cast<arrow::Int64Builder*>(order_A_b.value_builder());

  // d 字段
  arrow::ListBuilder order_D_b(pool, std::make_shared<arrow::Int64Builder>(pool));
  arrow::Int64Builder& order_D_value_b = *static_cast<arrow::Int64Builder*>(order_D_b.value_builder());

  for (int si{0}; auto& row : coll.m_data)
  {
    // symbol
    auto symbol = si_map.at(si);

    // 打平处理
    for (int mi{0}; auto& oad : row)
    {
      ARROW_RETURN_NOT_OK(symbol_b.Append(symbol));
      ARROW_RETURN_NOT_OK(mi_b.Append(mi));

      ARROW_RETURN_NOT_OK(order_A_b.Append());
      auto oa = oad.get_order_A();
      ARROW_RETURN_NOT_OK(order_A_value_b.AppendValues(oa.begin(), oa.end()));

      ARROW_RETURN_NOT_OK(order_D_b.Append());
      auto od = oad.get_order_D();
      ARROW_RETURN_NOT_OK(order_D_value_b.AppendValues(od.begin(), od.end()));

      mi++;
    }

    si++;
  }

  // 构建 Array 与 Table
  std::shared_ptr<arrow::Array> symbol_arr;
  ARROW_RETURN_NOT_OK(symbol_b.Finish(&symbol_arr));
  std::shared_ptr<arrow::Array> mi_arr;
  ARROW_RETURN_NOT_OK(mi_b.Finish(&mi_arr));
  std::shared_ptr<arrow::Array> a_arr;
  ARROW_RETURN_NOT_OK(order_A_b.Finish(&a_arr));
  std::shared_ptr<arrow::Array> d_arr;
  ARROW_RETURN_NOT_OK(order_D_b.Finish(&d_arr));

  return arrow::Table::Make(schema, {symbol_arr, mi_arr, a_arr, d_arr});
}

arrow::Status write_pqt(std::shared_ptr<arrow::Table> table)
{
  std::shared_ptr<arrow::io::FileOutputStream> outfile;

  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_out.parquet"));
  PARQUET_THROW_NOT_OK(parquet::arrow::WriteTable(
      *table, arrow::default_memory_pool(), outfile, 1
  ));

  return arrow::Status::OK();
}

arrow::Status run_main()
{
  const SiMapping si_map{{0, "000001.SZ"}, {1, "600000.SH"}};

  IndexedOrderAD<5> si_1{{
      OrderAD({1, 2, 3}, {4}),
      OrderAD({5, 6}, {}),
      OrderAD({8, 10}, {9}),
      OrderAD({11, 12, 13}, {}),
      OrderAD({21, 22, 23}, {17}),
  }};
  IndexedOrderAD<5> si_2{{
      OrderAD({1, 2, 3}, {4}),
      OrderAD({5, 6}, {}),
      OrderAD({8, 10}, {9}),
      OrderAD({11, 12, 13}, {}),
      OrderAD({21, 22, 23}, {17}),
  }};

  IndexedOrderADCollection<5> coll;
  coll.m_data.push_back(si_1);
  coll.m_data.push_back(si_2);

  std::cout << "coll_len:" << coll.len() << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto table, create_table(si_map, coll));

  ARROW_RETURN_NOT_OK(write_pqt(table));

  return arrow::Status::OK();
}

int main(int argc, char** argv)
{
  arrow::Status st{run_main()};
  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }

  return 0;
}
