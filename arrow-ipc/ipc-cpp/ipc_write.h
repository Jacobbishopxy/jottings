/**
 * @file:	ipc_write.h
 * @author:	Jacob Xie
 * @date:	2023/04/28 16:07:05 Friday
 * @brief:
 **/

#ifndef __IPC_WRITE__H__
#define __IPC_WRITE__H__

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <string>

arrow::Status write_ipc_file(std::string filename, std::shared_ptr<arrow::Table> table);

int get_sock();

class SocketOutputStream : public arrow::io::OutputStream
{
public:
  SocketOutputStream(std::shared_ptr<arrow::io::FileOutputStream> target);
  ~SocketOutputStream();

  arrow::Status Close() override;
  arrow::Status Abort() override;
  bool closed() const override;
  arrow::Status Flush() override;

  static arrow::Result<std::shared_ptr<SocketOutputStream>> Open(int sock);
  arrow::Status Write(const void* data, int64_t nbytes) override;
  arrow::Status Write(const std::shared_ptr<arrow::Buffer>& data) override;
  arrow::Result<int64_t> Tell() const override;

private:
  std::shared_ptr<arrow::io::FileOutputStream> m_target;
  uint64_t m_position;
};

#endif //!__IPC_WRITE__H__
