/**
 * @file:	main_ipc_stream_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:17 Monday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <asio.hpp>
#include <iostream>
#include <sys/socket.h>

using asio::ip::tcp;

constexpr uint16_t port = 56565;
constexpr char host[] = "127.0.0.1";

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

class SocketOutputStream : public arrow::io::OutputStream
{
public:
  SocketOutputStream(std::shared_ptr<arrow::io::FileOutputStream> target) : m_target(target), m_position(0) {}

  virtual ~SocketOutputStream() {}

  arrow::Status Close() override { return m_target->Close(); }
  arrow::Status Abort() override { return m_target->Abort(); }
  bool closed() const override { return m_target->closed(); }
  arrow::Status Flush() override { return m_target->Flush(); }

  static arrow::Result<std::shared_ptr<SocketOutputStream>> Open(int sock)
  {
    auto target_res = arrow::io::FileOutputStream::Open(sock);
    if (!target_res.ok())
    {
      return target_res.status();
    }
    return std::make_shared<SocketOutputStream>(*target_res);
  }

  arrow::Status Write(const void* data, int64_t nbytes) override
  {
    m_position += nbytes;
    return m_target->Write(data, nbytes);
  }

  arrow::Status Write(const std::shared_ptr<arrow::Buffer>& data) override
  {
    m_position += data->size();
    return m_target->Write(data);
  }

  arrow::Result<int64_t> Tell() const override { return m_position; }

private:
  std::shared_ptr<arrow::io::FileOutputStream> m_target;
  uint64_t m_position;
};

int main(int argc, char** argv)
{
  struct sockaddr_in addr;
  int sock = socket(AF_INET, SOCK_STREAM, 0);
  addr.sin_family = AF_INET;
  addr.sin_port = htons(port);

  try
  {
    // Create a mock table
    auto table = gen_mock_table();

    // Create A TCP socket and accept a connection
    asio::io_context io_context;
    tcp::acceptor acceptor(io_context, tcp::endpoint(tcp::v4(), port));

    for (;;)
    {
      tcp::socket socket(io_context);
      asio::error_code ignored_error;
      acceptor.accept(socket);
      // asio::write(socket, asio::buffer("233"), ignored_error);

      printf("Opening SocketOutputStream \n");

      // inet_pton(AF_INET, host, &addr.sin_addr);
      // connect(sock, (struct sockaddr*)&addr, sizeof(addr));

      // Create an Arrow RecordBatchWriter
      auto stream = SocketOutputStream::Open(sock).ValueOrDie();
      auto writer = arrow::ipc::MakeStreamWriter(stream, table->schema()).ValueOrDie();
      writer->WriteTable(*table).ok();
      writer->Close().ok();

      printf("Table sent\n");
    }
  }
  catch (std::exception& e)
  {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
