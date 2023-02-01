/**
 * @file:	file_io.cpp
 * @author:	Jacob Xie
 * @date:	2023/01/16 21:34:18 Monday
 * @brief:	File I/O
 **/

#include <arrow/api.h>
#include <arrow/csv/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <parquet/arrow/reader.h>
#include <parquet/arrow/writer.h>

#include <iostream>

// 从 stater.cpp 拷贝而来的初始化文件
arrow::Status GenInitialFile()
{
  // 构建两个 8-bit 整数 arrays 以及一个 16-bit 整数 array
  arrow::Int8Builder int8builder;
  int8_t days_raw[5] = {1, 12, 17, 23, 28};
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(days_raw, 5));
  std::shared_ptr<arrow::Array> days;
  ARROW_ASSIGN_OR_RAISE(days, int8builder.Finish());

  int8_t months_raw[5] = {1, 3, 5, 7, 1};
  ARROW_RETURN_NOT_OK(int8builder.AppendValues(months_raw, 5));
  std::shared_ptr<arrow::Array> months;
  ARROW_ASSIGN_OR_RAISE(months, int8builder.Finish());

  arrow::Int16Builder int16builder;
  int16_t years_raw[5] = {1990, 2000, 1995, 2000, 1995};
  ARROW_RETURN_NOT_OK(int16builder.AppendValues(years_raw, 5));
  std::shared_ptr<arrow::Array> years;
  ARROW_ASSIGN_OR_RAISE(years, int16builder.Finish());

  // Arrays 的 vector
  std::vector<std::shared_ptr<arrow::Array>> columns = {days, months, years};

  // 构建 schema 用于 Table 的初始化
  std::shared_ptr<arrow::Field> field_day, field_month, field_year;
  std::shared_ptr<arrow::Schema> schema;

  field_day = arrow::field("Day", arrow::int8());
  field_month = arrow::field("Month", arrow::int8());
  field_year = arrow::field("Year", arrow::int16());

  schema = arrow::schema({field_day, field_month, field_year});
  // 创建 Table
  std::shared_ptr<arrow::Table> table;
  table = arrow::Table::Make(schema, columns);

  // 测试文件 IPC，CSV 以及 Parquet 格式
  std::shared_ptr<arrow::io::FileOutputStream> outfile;
  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_in.arrow"));
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::ipc::RecordBatchWriter> ipc_writer, arrow::ipc::MakeFileWriter(outfile, schema));
  ARROW_RETURN_NOT_OK(ipc_writer->WriteTable(*table));
  ARROW_RETURN_NOT_OK(ipc_writer->Close());

  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_in.csv"));
  ARROW_ASSIGN_OR_RAISE(auto csv_writer, arrow::csv::MakeCSVWriter(outfile, table->schema()));
  ARROW_RETURN_NOT_OK(csv_writer->WriteTable(*table));
  ARROW_RETURN_NOT_OK(csv_writer->Close());

  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_in.parquet"));
  PARQUET_THROW_NOT_OK(
      parquet::arrow::WriteTable(*table, arrow::default_memory_pool(), outfile, 5)
  );

  return arrow::Status::OK();
}

arrow::Status RunMain()
{
  // 通过一个 helper 函数为每种格式生成初始化文件
  ARROW_RETURN_NOT_OK(GenInitialFile());

  // ================================================================================================
  // A. Arrow Files 的 I/O
  // ================================================================================================
  //
  // 备注：IPC 与 Parquet 的区别在本项目 README.md 的 Notes 栏目下
  //
  // 1. 读取文件：
  //  a. 打开文件
  //  b. 绑定文件至 `ipc::RecordBatchFileReader`
  //  c. 读取文件至 `RecordBatch`
  //
  // 2. 写入文件：
  //  a. 获取一个 `io::FileOutputStream`
  //  b. 从 `RecordBatch` 中写入文件

  // 首先构建一个 `ReadableFile` 对象，使 readers 可以指向磁盘中正确的数据。
  // 本例中该对象将被复用，并重新绑定若干文件
  std::shared_ptr<arrow::io::ReadableFile> infile;
  // 需要将 `io::ReadableFile` 绑定至一个通过 `io::ReadableFile::Open()` 打开的文件。
  ARROW_ASSIGN_OR_RAISE(
      infile,
      arrow::io::ReadableFile::Open("test_in.arrow", arrow::default_memory_pool())
  );

  // 打开一个 Arrow file reader
  // 一个 `io::ReadableFile` 在读取 Arrow 文件时所提供的功能还是太泛化了。
  // 我们需要它获取一个 `ipc::RecordBatchFileReader` 对象，其实现了所有读取 Arrow 文件的逻辑。
  ARROW_ASSIGN_OR_RAISE(auto ipc_reader, arrow::ipc::RecordBatchFileReader::Open(infile));

  // 读取一个打开的 Arrow 文件至 RecordBatch
  // Arrow 文件可以拥有若干 `RecordBatches`，因此必须传入索引。本文件只有一个，因此传入 0：
  std::shared_ptr<arrow::RecordBatch> rbatch;
  ARROW_ASSIGN_OR_RAISE(rbatch, ipc_reader->ReadRecordBatch(0));

  // 准备一个 FileOutputStream
  // 对于输出，我们需要一个 `io::FileOutputStream`，与 `io::ReadableFile` 类似是可以被复用的
  std::shared_ptr<arrow::io::FileOutputStream> outfile;
  // 将其绑定至 `test_out.arrow`
  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_out.arrow"));

  // 从 RecordBatch 中写入 Arrow 文件
  // 现在使用之前读取了数据的 `RecordBatch`，与目标文件一起，创建一个 `ipc::RecordBatchWriter`。
  // `ipc::RecordBatchWriter` 需要两个东西：
  // 1. 目标文件
  // 2. `RecordBatch` 的 Schema（万一需要若干同样格式的 `RecordBatch` 写入）
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::ipc::RecordBatchWriter> ipc_writer, arrow::ipc::MakeFileWriter(outfile, rbatch->schema()));
  // 可以调用 `ipc::RecordBatchWriter::WriteRecordBatch()` 与我们的 `RecordBatch` 来填充文件
  ARROW_RETURN_NOT_OK(ipc_writer->WriteRecordBatch(*rbatch));
  // 对于 IPC 需要特别的关闭动作，因为它预期可能会有多个 batch 在被写入
  ARROW_RETURN_NOT_OK(ipc_writer->Close());

  // ================================================================================================
  // B. CSV 的 I/O
  // ================================================================================================
  //
  // 1. 读取文件
  //  a. 打开文件
  //  b. 准备 Table
  //  c. 通过 `csv::TableReader` 读取文件
  //
  // 2. 写入文件
  //  a. 获取一个 `io::FileOutputStream`
  //  b. 从 `Table` 中写入文件

  // 打开一个 CSV 文件需要打开一个 `io::ReadableFile`，类似于 Arrow 文件。
  // 这里可以复用之前的 `io::ReadableFile`
  ARROW_ASSIGN_OR_RAISE(infile, arrow::io::ReadableFile::Open("test_in.csv"));

  // 准备一个 Table
  std::shared_ptr<arrow::Table> csv_table;

  // 读取 CSV 文件至 Table
  // CSV reader 有一些选项需要被传入 -- 幸运的是它们都有默认值可以被直接传入。
  // 其他的选项可以查看 https://arrow.apache.org/docs/cpp/api/formats.html
  ARROW_ASSIGN_OR_RAISE(
      auto csv_reader,
      arrow::csv::TableReader::Make(
          arrow::io::default_io_context(), infile, arrow::csv::ReadOptions::Defaults(),
          arrow::csv::ParseOptions::Defaults(), arrow::csv::ConvertOptions::Defaults()
      )
  )
  // 读取 table
  ARROW_ASSIGN_OR_RAISE(csv_table, csv_reader->Read())

  // 从 Table 中写入 CSV 文件
  // CSV 的写入看起来与 IPC 的写入一样，除了使用的是 `Table` 以外
  // 绑定输出文件至 "test_out.csv"
  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_out.csv"));
  // CSV writer 使用默认值
  ARROW_ASSIGN_OR_RAISE(auto csv_writer, arrow::csv::MakeCSVWriter(outfile, csv_table->schema()));
  ARROW_RETURN_NOT_OK(csv_writer->WriteTable(*csv_table));
  // 虽然不是必须的，但是这是个安全的实践
  ARROW_RETURN_NOT_OK(csv_writer->Close());

  // ================================================================================================
  // C. Parquet 的 I/O
  // ================================================================================================
  //
  // 1. 读取文件
  //  a. 打开文件
  //  b. 准备 `parquet::arrow::FileReader`
  //  c. 从 `Table` 中读取文件
  //
  // 2. 写入文件
  //  a. 从 `Table` 中写入文件

  // 打开一个 Parquet 文件
  // 同样的 Parquet 也需要一个 `io::ReadableFile`
  // 绑定文件至 "test_in.parquet"
  ARROW_ASSIGN_OR_RAISE(infile, arrow::io::ReadableFile::Open("test_in.parquet"));

  // 构建一个 Parquet Reader
  // 同样的也需要一个 Reader 来实际的读取文件。之前用的都是 Arrow 的命名空间获取 Reader，
  // 这次用的是 Parquet 的命名空间使用 `parquet::arrow::FileReader`：
  std::unique_ptr<parquet::arrow::FileReader> reader;
  // 接着调用 `parquet::arrow::OpenFile()`，这是必要的，即使使用了 `io::ReadableFile::Open()`。
  // 注意传递的 `parquet::arrow::FileReader` 是引用，而不是将其赋值给输出：
  PARQUET_THROW_NOT_OK(parquet::arrow::OpenFile(infile, arrow::default_memory_pool(), &reader));

  // 读取 Parquet 文件至 Table
  std::shared_ptr<arrow::Table> parquet_table;
  PARQUET_THROW_NOT_OK(reader->ReadTable(&parquet_table));

  // 通过 Table 写入 Parquet 文件
  // 对于单个写入的 Parquet 文件不需要一个 writer 对象，只需要 table，指向将会必要使用道德内存消费的内存池，
  // 告诉其写到哪里，以及分割文件的 chunk 大小：
  ARROW_ASSIGN_OR_RAISE(outfile, arrow::io::FileOutputStream::Open("test_out.parquet"));
  PARQUET_THROW_NOT_OK(parquet::arrow::WriteTable(
      *parquet_table, arrow::default_memory_pool(), outfile, 5
  ));

  return arrow::Status::OK();
};

int main(int argc, char** argv)
{
  arrow::Status st{RunMain()};
  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }
  return 0;
}
