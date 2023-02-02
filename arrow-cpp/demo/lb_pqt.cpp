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

const auto filename = std::string{"test.parquet"};

struct PqtSchema
{
  std::string symbol;
  int mi;
  std::set<int> a;
  std::set<int> d;
};

const auto schema = arrow::schema({
    arrow::field("symbol", arrow::utf8()),
    arrow::field("mi", arrow::int64()),
    arrow::field("a", arrow::list(arrow::int64())),
    arrow::field("d", arrow::list(arrow::int64())),
});

// ================================================================================================
// Create a Table from IndexedOrderAD and write it into a Parquet
// ================================================================================================

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

    if (coll.is_empty(si))
      continue;

    // 打平处理
    for (int mi{0}; auto& oad : row)
    {
      auto oa = oad.get_order_A();
      auto od = oad.get_order_D();

      // 两者皆无效时跳过
      if (!oa.empty() && !od.empty())
      {
        ARROW_RETURN_NOT_OK(symbol_b.Append(symbol));
        ARROW_RETURN_NOT_OK(mi_b.Append(mi));

        ARROW_RETURN_NOT_OK(order_A_b.Append());
        ARROW_RETURN_NOT_OK(order_A_value_b.AppendValues(oa.begin(), oa.end()));

        ARROW_RETURN_NOT_OK(order_D_b.Append());
        ARROW_RETURN_NOT_OK(order_D_value_b.AppendValues(od.begin(), od.end()));
      }

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

  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open(filename));
  PARQUET_THROW_NOT_OK(parquet::arrow::WriteTable(
      *table, arrow::default_memory_pool(), outfile, 1
  ));

  return arrow::Status::OK();
}

// mock
arrow::Status mock_data_write()
{
  const SiMapping si_map{{0, "000001.SZ"}, {1, "600000.SH"}, {2, "300001.SH"}};

  IndexedOrderAD<5> si_1{{
      OrderAD({1, 2, 3}, {4}),
      OrderAD({5, 6}, {}),
      OrderAD({8, 10}, {9}),
      OrderAD({}, {}),
      OrderAD({21, 22, 23}, {17}),
  }};
  IndexedOrderAD<5> si_2{{
      OrderAD({1, 2, 3}, {4}),
      OrderAD({}, {}),
      OrderAD({8, 10}, {9}),
      OrderAD({11, 12, 13}, {}),
      OrderAD({21, 22, 23}, {17}),
  }};
  IndexedOrderAD<5> si_3{{
      OrderAD(),
      OrderAD(),
      OrderAD(),
      OrderAD(),
      OrderAD(),
  }};

  IndexedOrderADCollection<5> coll(3);
  coll.m_data.push_back(si_1);
  coll.m_data.push_back(si_2);
  coll.m_data.push_back(si_3);

  std::cout << "coll_len:" << coll.len() << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto table, create_table(si_map, coll));

  ARROW_RETURN_NOT_OK(write_pqt(table));

  return arrow::Status::OK();
}

// ================================================================================================
// Read Parquet and turn Table into IndexedOrderAD
// ================================================================================================

arrow::Result<std::shared_ptr<arrow::Table>> read_pqt()
{
  std::shared_ptr<arrow::io::ReadableFile> infile;

  ARROW_ASSIGN_OR_RAISE(infile, arrow::io::ReadableFile::Open(filename));

  std::unique_ptr<parquet::arrow::FileReader> reader;

  PARQUET_THROW_NOT_OK(parquet::arrow::OpenFile(infile, arrow::default_memory_pool(), &reader));

  std::shared_ptr<arrow::Table> table;

  PARQUET_THROW_NOT_OK(reader->ReadTable(&table));

  return table;
}

arrow::Result<std::vector<PqtSchema>> deconstruct_table(std::shared_ptr<arrow::Table> table)
{
  // symbol 字段
  auto symbol_arr = std::static_pointer_cast<arrow::StringArray>(table->column(0)->chunk(0));

  // mi 字段
  auto mi_arr = std::static_pointer_cast<arrow::Int64Array>(table->column(1)->chunk(0));

  // a 字段
  auto order_A_arr = std::static_pointer_cast<arrow::ListArray>(table->column(2)->chunk(0));
  auto order_A_arr_values = std::static_pointer_cast<arrow::Int64Array>(order_A_arr->values());
  const int* oa_ptr = order_A_arr_values->data()->GetValues<int>(1);

  // d 字段
  auto order_D_arr = std::static_pointer_cast<arrow::ListArray>(table->column(3)->chunk(0));
  auto order_D_arr_values = std::static_pointer_cast<arrow::Int64Array>(order_A_arr->values());
  const int* od_ptr = order_D_arr_values->data()->GetValues<int>(1);

  std::vector<PqtSchema> res;

  for (int64_t i = 0; i < table->num_rows(); i++)
  {
    // symbol
    std::string symbol = static_cast<std::string>(symbol_arr->Value(i));

    // mi
    int mi = mi_arr->Value(i);

    // order_A
    const int* oa_first = oa_ptr + order_A_arr->value_offset(i);
    const int* oa_last = oa_ptr + order_A_arr->value_offset(i + 1);
    std::set<int> oa(oa_first, oa_last);

    // order_D
    const int* od_first = od_ptr + order_D_arr->value_offset(i);
    const int* od_last = od_ptr + order_D_arr->value_offset(i + 1);
    std::set<int> od(od_first, od_last);

    res.push_back({symbol, mi, oa, od});
  }

  return res;
}

// mock
arrow::Status mock_data_read()
{
  // TODO

  return arrow::Status::OK();
}

int main(int argc, char** argv)
{
  arrow::Status st_w{mock_data_write()};
  if (!st_w.ok())
  {
    std::cerr << st_w << std::endl;
    return 1;
  }

  arrow::Status st_r{mock_data_read()};
  if (!st_r.ok())
  {
    std::cerr << st_r << std::endl;
    return 1;
  }

  return 0;
}
