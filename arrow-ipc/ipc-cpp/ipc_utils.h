/**
 * @file:	ipc_utils.h
 * @author:	Jacob Xie
 * @date:	2023/05/02 00:08:24 Tuesday
 * @brief:
 **/

#ifndef __IPC_UTILS__H__
#define __IPC_UTILS__H__

#include <arrow/api.h>

arrow::Status ParseHost(std::string host, std::string* host_address, std::string* host_port);

arrow::Status ParseEndpoint(std::string endpoint, std::string* endpoint_type, std::string* endpoint_value);

std::shared_ptr<arrow::Table> gen_mock_table();

#endif //!__IPC_UTILS__H__
