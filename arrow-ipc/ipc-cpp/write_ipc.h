/**
 * @file:	write_ipc.h
 * @author:	Jacob Xie
 * @date:	2023/04/28 16:07:05 Friday
 * @brief:
 **/

#include <arrow/api.h>
#include <arrow/io/api.h>
#include <arrow/ipc/api.h>
#include <string>

arrow::Status write_ipc_file(std::string filename, std::shared_ptr<arrow::Table> table);