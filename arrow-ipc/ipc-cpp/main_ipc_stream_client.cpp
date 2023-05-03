/**
 * @file:	main_ipc_stream_client.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:30 Monday
 * @brief:
 **/

#include <asio.hpp>
#include <iostream>

#include "ipc_read.h"

using asio::ip::tcp;

const std::string host{"127.0.0.1"};
const std::string port{"56565"};
const std::string addr = host + ":" + port;

int main(int argc, char** argv)
{
  try
  {
    // See `tcp_daytime_client_sync.cpp`
    asio::io_context io_context;
    tcp::resolver resolver(io_context);
    tcp::resolver::results_type endpoints = resolver.resolve(host, port);
    tcp::socket socket(io_context);
    asio::connect(socket, endpoints);

    // Create a new input stream and let it connect to the server
    auto stream = new SocketInputStream(addr);
    auto sts = stream->Connect();
    std::cout << "Connection status: " << sts << std::endl;

    // arrow::ipc stream reader
    auto reader = arrow::ipc::RecordBatchStreamReader::Open(stream);
    std::cout << "Reader status" << reader.status() << std::endl;
    // get table from stream reader
    auto table = arrow::Table::FromRecordBatchReader(reader.ValueOrDie().get()).ValueOrDie();

    for (auto& cn : table.get()->ColumnNames())
    {
      std::cout << cn << std::endl;
    }

    // close stream
    stream->Close().ok();
  }
  catch (std::exception& e)
  {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
