/**
 * @file:	ipc_utils.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 23:45:01 Monday
 * @brief:
 **/

#include "ipc_utils.h"

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

// generate a mock table
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
