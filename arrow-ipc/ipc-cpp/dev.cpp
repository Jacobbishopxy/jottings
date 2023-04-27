/**
 * @file:	dev.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/27 15:50:20 Thursday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <string>
#include <vector>

std::shared_ptr<arrow::Table> gen_mock_table()
{
  auto schema = arrow::schema({
      arrow::field("Day", arrow::int8()),
      arrow::field("Month", arrow::int8()),
      arrow::field("Year", arrow::int16()),
  });

  arrow::Int8Builder i8b;
  arrow::Int16Builder i16b;

  i8b.AppendValues(std::vector<int8_t>{1, 12, 17, 23, 28});
  auto days = i8b.Finish().ValueUnsafe();

  i8b.AppendValues(std::vector<int8_t>{1, 3, 5, 7, 1});
  auto months = i8b.Finish().ValueUnsafe();

  i16b.AppendValues(std::vector<int16_t>{1990, 2000, 1995, 2000, 1995});
  auto years = i16b.Finish().ValueUnsafe();

  auto columns = {days, months, years};

  return arrow::Table::Make(schema, columns);
}

void write_ipc(std::string filename, arrow::Table& table)
{
  // TODO
}

int main(int argc, char** argv)
{
  // TODO

  return 0;
}
