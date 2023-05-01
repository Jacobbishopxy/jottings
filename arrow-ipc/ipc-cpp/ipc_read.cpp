/**
 * @file:	ipc_read.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/30 20:54:39 Sunday
 * @brief:
 **/

#include <asio.hpp>

#include "ipc_read.h"
#include "ipc_utils.h"

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

SocketInputStream::SocketInputStream(const std::string& endpoint)
    : m_endpoint(endpoint), m_sock(-1), m_pos(0) {}

SocketInputStream::~SocketInputStream()
{
  if (m_sock != -1)
    Close().ok();
}

arrow::Status SocketInputStream::Connect()
{
  std::string socket_family;
  std::string host;
  arrow::Status status;

  status = ParseEndpoint(m_endpoint, &socket_family, &host);
  if (!status.ok())
  {
    return arrow::Status::Invalid("Error parsing endpoint string: " + m_endpoint);
  }

  if (socket_family.empty() || socket_family == "tcp")
  {
    std::string addr_str;
    std::string port_str;
    status = ParseHost(host, &addr_str, &port_str);
    if (!status.ok())
    {
      return arrow::Status::Invalid("Error parsing host string: " + host);
    }

    int port_num = std::stoi(port_str);
    struct sockaddr_in serv_addr;

    if (m_sock == -1)
    {
      if ((m_sock = socket(AF_INET, SOCK_STREAM, 0)) < 0)
      {
        return arrow::Status::IOError("Socket creation error");
      }
    }

    bzero((char*)&serv_addr, sizeof(serv_addr));
    serv_addr.sin_addr.s_addr = inet_addr(addr_str.c_str());
    serv_addr.sin_family = AF_INET;
    serv_addr.sin_port = htons(port_num);

    if (connect(m_sock, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0)
    {
      return arrow::Status::IOError("Connection failed to AF_INET: " + host);
    }
  }
  else if (socket_family == "unix")
  {
    if (m_sock == -1)
    {
      if ((m_sock = socket(AF_UNIX, SOCK_STREAM, 0)) < 0)
      {
        return arrow::Status::IOError("Socket creation error");
      }
    }

    struct sockaddr_un serv_addr;
    bzero((char*)&serv_addr, sizeof(serv_addr));
    serv_addr.sun_family = AF_UNIX;
    strcpy(serv_addr.sun_path, host.c_str());

    if (connect(m_sock, (struct sockaddr*)&serv_addr, sizeof(serv_addr)) < 0)
    {
      return arrow::Status::IOError("Connection failed to AF_UNIX: " + host);
    }
  }
  else
  {
    return arrow::Status::Invalid("Unsupported socket family: " + socket_family);
  }

  return arrow::Status::OK();
}

arrow::Status SocketInputStream::Close()
{
  int status = close(m_sock);
  m_sock = -1;

  if (status != 0)
  {
    return arrow::Status::IOError("Failed to correctly close connection");
  }

  return arrow::Status::OK();
}

bool SocketInputStream::closed() const { return m_sock == -1; }

arrow::Result<int64_t> SocketInputStream::Tell() const { return m_pos; }

arrow::Result<int64_t> SocketInputStream::Read(int64_t nbytes, void* out)
{
  if (nbytes == 0)
  {
    return 0;
  }

  int status = recv(m_sock, out, nbytes, MSG_WAITALL);
  if (status == 0)
  {
    return arrow::Status::IOError("connection closed unexpectedly");
  }
  else if (status < 0)
  {
    return arrow::Status::IOError("error reading from socket");
  }

  m_pos += nbytes;
  return nbytes;
}

arrow::Result<std::shared_ptr<arrow::Buffer>> SocketInputStream::Read(int64_t nbytes)
{
  arrow::Result<std::shared_ptr<arrow::ResizableBuffer>> result = arrow::AllocateResizableBuffer(nbytes);
  ARROW_RETURN_NOT_OK(result);
  std::shared_ptr<arrow::ResizableBuffer> buffer = std::move(result).ValueUnsafe();
  int64_t bytes_read;
  ARROW_ASSIGN_OR_RAISE(bytes_read, Read(nbytes, buffer->mutable_data()));
  ARROW_RETURN_NOT_OK(buffer->Resize(bytes_read, false));
  buffer->ZeroPadding();
  return buffer;
}