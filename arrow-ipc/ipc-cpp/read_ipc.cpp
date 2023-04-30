/**
 * @file:	read_ipc.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/30 20:54:39 Sunday
 * @brief:
 **/

#include "read_ipc.h"

arrow::Result<std::shared_ptr<arrow::Table>> read_ipc_file(std::string filename)
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
