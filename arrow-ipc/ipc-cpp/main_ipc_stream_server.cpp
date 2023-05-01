/**
 * @file:	main_ipc_stream_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:17 Monday
 * @brief:
 **/

#include <asio.hpp>
#include <iostream>
#include <sys/socket.h>

#include "ipc_utils.h"
#include "ipc_write.h"

using asio::ip::tcp;

constexpr uint16_t port = 56565;
constexpr char host[] = "127.0.0.1";

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

      printf("Opening SocketOutputStream \n");

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
