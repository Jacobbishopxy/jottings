/**
 * @file:	main_ipc_stream_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:17 Monday
 * @brief:
 **/

#include <asio.hpp>
#include <iostream>

#include "ipc_utils.h"
#include "ipc_write.h"

using asio::ip::tcp;

constexpr uint16_t port = 56565;

int main(int argc, char** argv)
{
  int sock = get_sock();
  std::cout << "sock: " << sock << std::endl;

  try
  {
    // Create a mock table
    auto table = gen_mock_table();

    // Create A TCP socket and accept a connection
    asio::io_context io_context;
    tcp::acceptor acceptor(io_context, tcp::endpoint(tcp::v4(), port));

    // iterative server, see `tcp_daytime_server_sync.cpp`
    for (;;)
    {
      tcp::socket socket(io_context);
      asio::error_code ignored_error;
      acceptor.accept(socket);

      printf("Opening SocketOutputStream \n");

      // Create a new output stream
      auto stream = SocketOutputStream::Open(sock).ValueOrDie();
      // arrow::ipc stream writer
      auto writer = arrow::ipc::MakeStreamWriter(stream, table->schema()).ValueOrDie();
      // write table into the output stream
      writer->WriteTable(*table).ok();
      // close writer
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
