/**
 * @file:	flight_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/09 19:15:54 Tuesday
 * @brief:
 *
 * Documentation: https://arrow.apache.org/docs/format/Flight.html
 * Code: https://arrow.apache.org/cookbook/cpp/flight.html
 **/

#include <iostream>
#include <memory>

#include <arrow/api.h>
#include <arrow/filesystem/api.h>
#include <arrow/flight/api.h>
#include <parquet/arrow/reader.h>
#include <parquet/arrow/writer.h>

// ================================================================================================
// Const
// ================================================================================================

const std::string HOST = "127.0.0.1";
const int PORT = 8815;

// ================================================================================================
// SimpleServer
// ================================================================================================

class SimpleServer : public arrow::flight::FlightServerBase
{
public:
  const arrow::flight::ActionType kActionDropDataset{"drop_dataset", "Delete a dataset."};

  const int64_t CHUNK_SIZE = 65536;

  explicit SimpleServer(std::shared_ptr<arrow::fs::FileSystem> root) : root_(std::move(root)) {}

  arrow::Status ListFlights(
      const arrow::flight::ServerCallContext&,
      const arrow::flight::Criteria*,
      std::unique_ptr<arrow::flight::FlightListing>* listings
  ) override;

  arrow::Status DoPut(
      const arrow::flight::ServerCallContext&,
      std::unique_ptr<arrow::flight::FlightMessageReader> reader,
      std::unique_ptr<arrow::flight::FlightMetadataWriter> writer
  ) override;

private:
  arrow::Result<arrow::flight::FlightInfo> MakeFlightInfo(
      const arrow::fs::FileInfo& file_info
  );

  arrow::Result<arrow::fs::FileInfo> FileInfoFromDescriptor(
      const arrow::flight::FlightDescriptor& descriptor
  );

  std::shared_ptr<arrow::fs::FileSystem> root_;
};

// ================================================================================================
// SimpleServer impl
// ================================================================================================

arrow::Status SimpleServer::ListFlights(
    const arrow::flight::ServerCallContext&,
    const arrow::flight::Criteria*,
    std::unique_ptr<arrow::flight::FlightListing>* listings
)
{
  std::cout << "ListFlights" << std::endl;

  arrow::fs::FileSelector selector;
  selector.base_dir = "/";
  ARROW_ASSIGN_OR_RAISE(auto listing, root_->GetFileInfo(selector));

  std::vector<arrow::flight::FlightInfo> flights;
  for (const auto& file_info : listing)
  {
    if (!file_info.IsFile() || file_info.extension() != "parquet")
      continue;
    ARROW_ASSIGN_OR_RAISE(auto info, MakeFlightInfo(file_info));
    flights.push_back(std::move(info));
  }

  *listings = std::unique_ptr<arrow::flight::FlightListing>(
      new arrow::flight::SimpleFlightListing(std::move(flights))
  );

  return arrow::Status::OK();
}

arrow::Status SimpleServer::DoPut(
    const arrow::flight::ServerCallContext&,
    std::unique_ptr<arrow::flight::FlightMessageReader> reader,
    std::unique_ptr<arrow::flight::FlightMetadataWriter> writer
)
{
  std::cout << "DoPut" << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto file_info, FileInfoFromDescriptor(reader->descriptor()));
  std::cout << file_info.path() << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto sink, root_->OpenOutputStream(file_info.path()));
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::Table> table, reader->ToTable());

  ARROW_RETURN_NOT_OK(parquet::arrow::WriteTable(*table, arrow::default_memory_pool(), sink, CHUNK_SIZE));

  return arrow::Status::OK();
}

arrow::Status Serve(std::string dataset_dir)
{

  auto fs = std::make_shared<arrow::fs::LocalFileSystem>();
  ARROW_RETURN_NOT_OK(fs->CreateDir(dataset_dir));
  auto root = std::make_shared<arrow::fs::SubTreeFileSystem>(dataset_dir, fs);

  arrow::flight::Location server_location;
  ARROW_ASSIGN_OR_RAISE(server_location, arrow::flight::Location::ForGrpcTcp(HOST, PORT));

  arrow::flight::FlightServerOptions options(server_location);

  auto server = std::unique_ptr<arrow::flight::FlightServerBase>(new SimpleServer(std::move(root)));
  ARROW_RETURN_NOT_OK(server->Init(options));

  std::cout << "🔈 Listening on port " << server->port() << std::endl;
  std::cout << "🏹 Serving " << dataset_dir << std::endl;

  ARROW_RETURN_NOT_OK(server->Serve());

  return arrow::Status::OK();
}

arrow::Result<arrow::flight::FlightInfo> SimpleServer::MakeFlightInfo(
    const arrow::fs::FileInfo& file_info
)
{
  ARROW_ASSIGN_OR_RAISE(auto input, root_->OpenInputFile(file_info));
  std::unique_ptr<parquet::arrow::FileReader> reader;
  ARROW_RETURN_NOT_OK(parquet::arrow::OpenFile(std::move(input), arrow::default_memory_pool(), &reader));

  std::shared_ptr<arrow::Schema> schema;
  ARROW_RETURN_NOT_OK(reader->GetSchema(&schema));

  auto descriptor = arrow::flight::FlightDescriptor::Path({file_info.base_name()});

  arrow::flight::FlightEndpoint endpoint;
  endpoint.ticket.ticket = file_info.base_name();
  arrow::flight::Location location;
  ARROW_ASSIGN_OR_RAISE(location, arrow::flight::Location::ForGrpcTcp("HOST", port()));
  endpoint.locations.push_back(location);

  int64_t total_records = reader->parquet_reader()->metadata()->num_rows();
  int64_t total_bytes = file_info.size();

  return arrow::flight::FlightInfo::Make(*schema, descriptor, {endpoint}, total_records, total_bytes);
}

arrow::Result<arrow::fs::FileInfo> SimpleServer::FileInfoFromDescriptor(
    const arrow::flight::FlightDescriptor& descriptor
)
{
  if (descriptor.type != arrow::flight::FlightDescriptor::PATH)
    return arrow::Status::Invalid("Must provide PATH-type FlightDescriptor");
  else if (descriptor.path.size() != 1)
    return arrow::Status::Invalid("Must provide PATH-type FlightDescriptor with one path component");
  return root_->GetFileInfo(descriptor.path[0]);
}

int main(int argc, char** argv)
{
  signal(SIGTERM, [](int i)
         { exit(i); });

  std::string dataset_dir = "./datasets";

  arrow::Status st = Serve(dataset_dir);

  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }

  return 0;
}
