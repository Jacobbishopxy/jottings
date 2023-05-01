/**
 * @file:	main_ipc_stream_client.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 12:50:30 Monday
 * @brief:
 **/

#include <iostream>

#include "ipc_read.h"

// constexpr uint16_t port = 56565;
// constexpr char host[] = "127.0.0.1";

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
