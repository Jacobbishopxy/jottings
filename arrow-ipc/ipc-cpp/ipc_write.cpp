/**
 * @file:	ipc_write.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/28 15:17:09 Friday
 * @brief:
 **/

#include "ipc_write.h"

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

SocketOutputStream::SocketOutputStream(std::shared_ptr<arrow::io::FileOutputStream> target)
    : m_target(target), m_position(0) {}

SocketOutputStream::~SocketOutputStream() {}

arrow::Status SocketOutputStream::Close() { return m_target->Close(); }
arrow::Status SocketOutputStream::Abort() { return m_target->Abort(); }
bool SocketOutputStream::closed() const { return m_target->closed(); }
arrow::Status SocketOutputStream::Flush() { return m_target->Flush(); }

arrow::Result<std::shared_ptr<SocketOutputStream>> SocketOutputStream::Open(int sock)
{
  auto target_res = arrow::io::FileOutputStream::Open(sock);
  if (!target_res.ok())
  {
    return target_res.status();
  }
  return std::make_shared<SocketOutputStream>(*target_res);
}

arrow::Status SocketOutputStream::Write(const void* data, int64_t nbytes)
{
  m_position += nbytes;
  return m_target->Write(data, nbytes);
}

arrow::Status SocketOutputStream::Write(const std::shared_ptr<arrow::Buffer>& data)
{
  m_position += data->size();
  return m_target->Write(data);
}

arrow::Result<int64_t> SocketOutputStream::Tell() const { return m_position; }
