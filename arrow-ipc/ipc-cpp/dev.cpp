/**
 * @file:	dev.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/27 15:50:20 Thursday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <iostream>
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

  i8b.AppendValues(std::vector<int8_t>{1, 12, 17, 23, 28}).ok();
  auto days = i8b.Finish().ValueUnsafe();

  i8b.AppendValues(std::vector<int8_t>{1, 3, 5, 7, 1}).ok();
  auto months = i8b.Finish().ValueUnsafe();

  i16b.AppendValues(std::vector<int16_t>{1990, 2000, 1995, 2000, 1995}).ok();
  auto years = i16b.Finish().ValueUnsafe();

  auto columns = {days, months, years};

  return arrow::Table::Make(schema, columns);
}

arrow::Status write_ipc(std::string filename, std::shared_ptr<arrow::Table> table)
{
  ARROW_ASSIGN_OR_RAISE(auto outfile, arrow::io::FileOutputStream::Open(filename));
  ARROW_ASSIGN_OR_RAISE(auto writer, arrow::ipc::MakeFileWriter(outfile, table->schema()));
  ARROW_RETURN_NOT_OK(writer->WriteTable(*table));
  ARROW_RETURN_NOT_OK(writer->Close());

  return arrow::Status::OK();
}

arrow::Result<std::shared_ptr<arrow::Table>> sync_read_ipc(std::string filename)
{
  ARROW_ASSIGN_OR_RAISE(auto infile, arrow::io::ReadableFile::Open(filename));
  ARROW_ASSIGN_OR_RAISE(auto reader, arrow::ipc::RecordBatchFileReader::Open(infile));
  std::vector<std::shared_ptr<arrow::RecordBatch>> record_batches;
  auto num = reader->num_record_batches();
  for (int i = 0; i < num; i++)
  {
    ARROW_ASSIGN_OR_RAISE(auto rbatch, reader->ReadRecordBatch(i));
    record_batches.push_back(rbatch);
  }

  return arrow::Table::FromRecordBatches(record_batches);
}

// TODO: async streaming

int main(int argc, char** argv)
{
  auto filename = "dev.ipc";

  auto table = gen_mock_table();

  auto st = write_ipc(filename, table);
  assert(st.ok());

  auto new_table = sync_read_ipc(filename).ValueUnsafe();

  std::cout << "Column name:" << std::endl;
  for (auto& cn : new_table->ColumnNames())
  {
    std::cout << cn << std::endl;
  }

  return 0;
}