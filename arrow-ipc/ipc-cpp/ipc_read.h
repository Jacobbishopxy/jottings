/**
 * @file:	ipc_read.h
 * @author:	Jacob Xie
 * @date:	2023/04/30 20:52:21 Sunday
 * @brief:
 **/

#ifndef __IPC_READ__H__
#define __IPC_READ__H__

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <string>

arrow::Result<std::shared_ptr<arrow::Table>> read_ipc_file(std::string filename);

class SocketInputStream : public arrow::io::InputStream
{
public:
  SocketInputStream(const std::string& endpoint);

  ~SocketInputStream();

  arrow::Status Connect();

  arrow::Status Close();

  bool closed() const;

  arrow::Result<int64_t> Tell() const;

  arrow::Result<int64_t> Read(int64_t nbytes, void* out);

  arrow::Result<std::shared_ptr<arrow::Buffer>> Read(int64_t nbytes);

private:
  const std::string m_endpoint;
  int m_sock;
  int64_t m_pos;
};

#endif //!__IPC_READ__H__
