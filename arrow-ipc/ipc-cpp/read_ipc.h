/**
 * @file:	read_ipc.h
 * @author:	Jacob Xie
 * @date:	2023/04/30 20:52:21 Sunday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <string>

arrow::Result<std::shared_ptr<arrow::Table>> read_ipc_file(std::string filename);
