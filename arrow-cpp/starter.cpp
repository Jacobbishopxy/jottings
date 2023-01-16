/**
 * @file:	dev.cpp
 * @author:	Jacob Xie
 * @date:	2023/01/15 23:56:02 Sunday
 * @brief:	Starter
 **/

#include <arrow/api.h>
#include <iostream>

arrow::Status RunMain()
{
  // Builders 是使用已存在的值来创建 Arrow Array 的主要方式
  arrow::Int8Builder int8builder;
  int8_t days_raw[5] = {1, 12, 17, 23, 28};

  // 如果 `AppendValues()` 失败，该宏直接返回至 `main()`
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(days_raw, 5));

  // 可以使用 `ArrayBuilder::Finish()` 来输出最终的最终的结构给 Array，特别说明是一个 `std::shared_ptr<arrow::Array>`
  // 注意之后代码中 ARROW_ASSIGN_OR_RAISE 的使用。`Finish()` 输出一个 `arrow::Result` 对象，即 ARROW_ASSIGN_OR_RAISE
  // 可以处理。如果该方法失败，它将带着 `Status` 返回至 `main()` 并解释哪里出问题了；如果成功，它将赋值最终输出给左值。

  // 尽管只有一个 Builder，而不是 Array -- 以下代码
  std::shared_ptr<arrow::Array> days;
  ARROW_ASSIGN_OR_RAISE(days, int8builder.Finish());

  // 一旦 `ArrayBuilder` 其 `Finish` 方法被调用，其状态被重置，使得它可再次被使用。因此可以重复上面的步骤创建 array：
  int8_t months_raw[5] = {1, 3, 5, 7, 1};
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(months_raw, 5));
  std::shared_ptr<arrow::Array> months;
  ARROW_ASSIGN_OR_RAISE(months, int8builder.Finish());

  // years
  arrow::Int16Builder int16builder;

  int16_t years_raw[5] = {2000, 2001, 2002, 2003, 2004};
  ARROW_RETURN_NOT_OK(int16builder.AppendValues(years_raw, 5));
  std::shared_ptr<arrow::Array> years;
  ARROW_ASSIGN_OR_RAISE(years, int16builder.Finish());

  // 构建 RecordBatch
  //
  // 1. 定义一个 Schema
  // 2. 加载 Schema 以及 Arrays 至构造函数
  std::shared_ptr<arrow::Field> field_day, field_month, field_year;
  std::shared_ptr<arrow::Schema> schema;

  // 每个字段需要名称与数据类型
  field_day = arrow::field("Day", arrow::int8());
  field_month = arrow::field("Month", arrow::int8());
  field_year = arrow::field("Year", arrow::int16());

  // Schema 可以由字段 vector 构成
  schema = arrow::schema({field_day, field_month, field_year});

  // *重要：每个列都是内部连续的，这与 Tables 相反（之后讲解）
  std::shared_ptr<arrow::RecordBatch> rbatch;
  // RecordBatch 需要 schema，列的长度，且匹配，以及真实数据
  rbatch = arrow::RecordBatch::Make(schema, days->length(), {days, months, years});

  std::cout << rbatch->ToString() << std::endl;

  // 构建 ChunkedArray
  //
  // 为了在 concat 时避免数据拷贝，或者是并行处理，或者是每个 chunk 皆有缓存，或者是超出了 2,147,483,647 行的限制
  // 可以构建有若干子 arrays 组成的数组
  int8_t days_raw2[5] = {6, 12, 3, 30, 22};
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(days_raw2, 5));
  std::shared_ptr<arrow::Array> days2;
  ARROW_ASSIGN_OR_RAISE(days2, int8builder.Finish());

  int8_t months_raw2[5] = {5, 4, 11, 3, 2};
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(months_raw2, 5));
  std::shared_ptr<arrow::Array> months2;
  ARROW_ASSIGN_OR_RAISE(months2, int8builder.Finish());

  int16_t years_raw2[5] = {1980, 2001, 1915, 2020, 1996};
  ARROW_RETURN_NOT_OK(int16builder.AppendValues(years_raw2, 5));
  std::shared_ptr<arrow::Array> years2;
  ARROW_ASSIGN_OR_RAISE(years2, int16builder.Finish());

  // 为了在 ChunkedArray 的构造函数中支持任意数量的 Arrays，Arrow 提供了 `ArrayVector`。
  // 即 Arrays 的 vector，我们将使用它来准备一个 `ChunkedArray`：
  // `ChunkedArray` 允许一个 arrays 的数组，它们互相并不连续。
  arrow::ArrayVector day_vecs{days, days2};
  // 接着用其初始化一个 `ChunkedArray`
  std::shared_ptr<arrow::ChunkedArray> day_chunks = std::make_shared<arrow::ChunkedArray>(day_vecs);
  // 接下来只需要重复 months 和 years，这样就有了三个不同类型的 `ChunkedArray`
  arrow::ArrayVector month_vecs{months, months};
  std::shared_ptr<arrow::ChunkedArray> month_chunks = std::make_shared<arrow::ChunkedArray>(month_vecs);

  arrow::ArrayVector year_vecs{years, years2};
  std::shared_ptr<arrow::ChunkedArray> year_chunks = std::make_shared<arrow::ChunkedArray>(year_vecs);

  // 构建 Table
  //
  // 对于 `ChunkedArray` 而言最有实际用途的是创建一个之前提到过的 Tables。类似于一个 `RecordBatch`，一个 `Table` 存储表格式的数据。
  // 然而 `Table` 并不保证连续性，因为它是由 `ChunkedArray` 构成的。
  // 一个 `Table` 的结构需要非连续的 columns，让它们处于同一处便于如同普通 arrays 那样来使用。
  std::shared_ptr<arrow::Table> table;
  table = arrow::Table::Make(schema, {day_chunks, month_chunks, year_chunks}, 10);

  std::cout << table->ToString() << std::endl;

  // 最后是 `Status` 的返回值
  return arrow::Status::OK();
}

/**
 * 使用 Arrow 的错误处理宏，也意味着 Arrow 永远不会抛出异常，而是基于返回的 Status 进行判断。
 */
int main(int argc, char** argv)
{
  arrow::Status st = RunMain();

  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }

  return 0;
}
