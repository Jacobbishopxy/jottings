/**
 * @file:	main_ipc_file.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/27 15:50:20 Thursday
 * @brief:
 **/

#include <iostream>

#include "ipc_read.h"
#include "ipc_utils.h"
#include "ipc_write.h"

int main(int argc, char** argv)
{
  auto filename = "dev.ipc";

  auto table = gen_mock_table();

  auto st = write_ipc_file(filename, table);
  assert(st.ok());

  auto new_table = read_ipc_file(filename).ValueUnsafe();

  std::cout << "Column name:" << std::endl;
  for (auto& cn : new_table->ColumnNames())
  {
    std::cout << cn << std::endl;
  }

  return 0;
}
