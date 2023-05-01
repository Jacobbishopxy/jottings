/**
 * @file:	main_ipc_stream_client.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:30 Monday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <asio.hpp>
#include <iostream>
#include <sys/socket.h>

using asio::ip::tcp;

// constexpr uint16_t port = 56565;
// constexpr char host[] = "127.0.0.1";

arrow::Status ParseHost(std::string host, std::string* host_address, std::string* host_port)
{
  size_t sep_pos = host.find(':');
  if (sep_pos == std::string::npos || sep_pos == host.length())
  {
    return arrow::Status::Invalid(
        "Expected host to be in format <host>:<port> but got: " + host
    );
  }

  *host_address = host.substr(0, sep_pos);
  *host_port = host.substr(sep_pos + 1);

  return arrow::Status::OK();
}

arrow::Status ParseEndpoint(std::string endpoint, std::string* endpoint_type, std::string* endpoint_value)
{
  size_t sep_pos = endpoint.find(':');

  // Check for a proper format
  if (sep_pos == std::string::npos)
  {
    return arrow::Status::Invalid(
        "Expected endpoint to be in format <endpoint_type>://<endpoint_value> "
        "or <host>:<port> for tcp IPv4, but got: " +
        endpoint
    );
  }

  // If IPv4 and no endpoint type specified, descriptor is entire endpoint
  if (endpoint.substr(sep_pos + 1, 2) != "//")
  {
    *endpoint_type = "";
    *endpoint_value = endpoint;
    return arrow::Status::OK();
  }

  // Parse string as <endpoint_type>://<endpoint_value>
  *endpoint_type = endpoint.substr(0, sep_pos);
  *endpoint_value = endpoint.substr(sep_pos + 3);

  return arrow::Status::OK();
}

class SocketInputStream : public arrow::io::InputStream
{
public:
  SocketInputStream(const std::string& endpoint)
      : m_endpoint(endpoint), m_sock(-1), m_pos(0) {}

  ~SocketInputStream()
  {
    if (m_sock != -1)
    {
      Close().ok();
    }
  }

  arrow::Status Connect()
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

  arrow::Status Close()
  {
    int status = close(m_sock);
    m_sock = -1;

    if (status != 0)
    {
      return arrow::Status::IOError("Failed to correctly close connection");
    }

    return arrow::Status::OK();
  }

  bool closed() const { return m_sock == -1; }

  arrow::Result<int64_t> Tell() const { return m_pos; }

  arrow::Result<int64_t> Read(int64_t nbytes, void* out)
  {
    // TODO: 0 bytes requested when message body length == 0
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

  arrow::Result<std::shared_ptr<arrow::Buffer>> Read(int64_t nbytes)
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

private:
  const std::string m_endpoint;
  int m_sock;
  int64_t m_pos;
};

int main(int argc, char** argv)
{
  auto stream = new SocketInputStream("127.0.0.1:56565");
  auto sts = stream->Connect();
  std::cout << sts << std::endl;

  auto reader = arrow::ipc::RecordBatchStreamReader::Open(stream).ValueOrDie();

  auto table = arrow::Table::FromRecordBatchReader(reader.get()).ValueOrDie();

  for (auto& cn : table.get()->ColumnNames())
  {
    std::cout << cn << std::endl;
  }

  return 0;
}
