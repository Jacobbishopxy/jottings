/**
 * @file:	write_ipc.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/28 15:17:09 Friday
 * @brief:
 **/

#include "write_ipc.h"

// ================================================================================================
//
// ================================================================================================

arrow::Status write_ipc_file(std::string filename, std::shared_ptr<arrow::Table> table)
{
  ARROW_ASSIGN_OR_RAISE(auto outfile, arrow::io::FileOutputStream::Open(filename));
  ARROW_ASSIGN_OR_RAISE(auto writer, arrow::ipc::MakeFileWriter(outfile, table->schema()));
  ARROW_RETURN_NOT_OK(writer->WriteTable(*table));
  ARROW_RETURN_NOT_OK(writer->Close());

  return arrow::Status::OK();
}
