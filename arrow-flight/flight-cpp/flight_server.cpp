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

using namespace arrow::flight;
using namespace arrow::fs;

// ================================================================================================
// SimpleServer
// ================================================================================================

class SimpleServer : public FlightServerBase
{
public:
  const ActionType kActionDropDataset{"drop_dataset", "Delete a dataset."};

  const int64_t CHUNK_SIZE = 65536;

  explicit SimpleServer(std::shared_ptr<FileSystem> root) : root_(std::move(root)) {}

  arrow::Status ListFlights(
      const ServerCallContext& context,
      const Criteria* criteria,
      std::unique_ptr<FlightListing>* listings
  ) override;

  arrow::Status GetFlightInfo(
      const ServerCallContext& context,
      const FlightDescriptor& descriptor,
      std::unique_ptr<FlightInfo>* info
  ) override;

  arrow::Status GetSchema(
      const ServerCallContext& context,
      const FlightDescriptor& descriptor,
      std::unique_ptr<SchemaResult>* schema
  ) override;

  arrow::Status DoGet(
      const ServerCallContext& context,
      const Ticket& request,
      std::unique_ptr<FlightDataStream>* stream
  ) override;

  arrow::Status DoPut(
      const ServerCallContext& context,
      std::unique_ptr<FlightMessageReader> reader,
      std::unique_ptr<FlightMetadataWriter> writer
  ) override;

  arrow::Status DoAction(
      const ServerCallContext& context,
      const Action& action,
      std::unique_ptr<ResultStream>* result
  ) override;

  arrow::Status ListActions(
      const ServerCallContext& context,
      std::vector<ActionType>* actions
  ) override;

private:
  arrow::Result<FlightInfo> MakeFlightInfo(
      const FileInfo& file_info
  );

  arrow::Result<std::unique_ptr<SchemaResult>> MakeSchema(
      const FileInfo& file_info
  );

  arrow::Result<FileInfo> FileInfoFromDescriptor(
      const FlightDescriptor& descriptor
  );

  std::shared_ptr<FileSystem> root_;
};

// ================================================================================================
// SimpleServer public impl
// ================================================================================================

arrow::Status SimpleServer::ListFlights(
    const ServerCallContext&,
    const Criteria*,
    std::unique_ptr<FlightListing>* listings
)
{
  std::cout << "ListFlights" << std::endl;

  FileSelector selector;
  selector.base_dir = "/";
  ARROW_ASSIGN_OR_RAISE(auto listing, root_->GetFileInfo(selector));

  std::vector<FlightInfo> flights;
  for (const auto& file_info : listing)
  {
    if (!file_info.IsFile() || file_info.extension() != "parquet")
      continue;
    ARROW_ASSIGN_OR_RAISE(auto info, MakeFlightInfo(file_info));
    flights.push_back(std::move(info));
  }

  *listings = std::unique_ptr<FlightListing>(
      new SimpleFlightListing(std::move(flights))
  );

  return arrow::Status::OK();
}

arrow::Status SimpleServer::GetFlightInfo(
    const ServerCallContext& context,
    const FlightDescriptor& descriptor,
    std::unique_ptr<FlightInfo>* info
)
{
  std::cout << "GetFlightInfo" << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto file_info, FileInfoFromDescriptor(descriptor));
  ARROW_ASSIGN_OR_RAISE(auto flight_info, MakeFlightInfo(file_info));
  *info = std::unique_ptr<FlightInfo>(new FlightInfo(std::move(flight_info)));

  return arrow::Status::OK();
}

arrow::Status SimpleServer::GetSchema(
    const ServerCallContext& context,
    const FlightDescriptor& descriptor,
    std::unique_ptr<SchemaResult>* schema
)
{
  std::cout << "GetSchema" << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto file_info, FileInfoFromDescriptor(descriptor));
  ARROW_ASSIGN_OR_RAISE(*schema, MakeSchema(file_info));

  return arrow::Status::OK();
}

arrow::Status SimpleServer::DoGet(
    const ServerCallContext& context,
    const Ticket& request,
    std::unique_ptr<FlightDataStream>* stream
)
{
  std::cout << "DoGet" << std::endl;

  ARROW_ASSIGN_OR_RAISE(auto input, root_->OpenInputFile(request.ticket));
  std::unique_ptr<parquet::arrow::FileReader> reader;
  ARROW_RETURN_NOT_OK(parquet::arrow::OpenFile(std::move(input), arrow::default_memory_pool(), &reader));

  std::shared_ptr<arrow::Table> table;
  ARROW_RETURN_NOT_OK(reader->ReadTable(&table));
  // Note that we can't directly pass TableBatchReader to
  // RecordBatchStream because TableBatchReader keeps a non-owning
  // reference to the underlying Table, which would then get freed
  // when we exit this function
  std::vector<std::shared_ptr<arrow::RecordBatch>> batches;
  arrow::TableBatchReader batch_reader(*table);
  ARROW_ASSIGN_OR_RAISE(batches, batch_reader.ToRecordBatches());
  ARROW_ASSIGN_OR_RAISE(auto owning_reader, arrow::RecordBatchReader::Make(std::move(batches), table->schema()));
  *stream = std::unique_ptr<FlightDataStream>(new RecordBatchStream(owning_reader));

  return arrow::Status::OK();
}

arrow::Status SimpleServer::DoPut(
    const ServerCallContext&,
    std::unique_ptr<FlightMessageReader> reader,
    std::unique_ptr<FlightMetadataWriter> writer
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

arrow::Status SimpleServer::DoAction(
    const ServerCallContext& context,
    const Action& action,
    std::unique_ptr<ResultStream>* result
)
{
  std::cout << "DoAction" << std::endl;

  if (action.type == kActionDropDataset.type)
  {
    *result = std::unique_ptr<ResultStream>(new SimpleResultStream({}));
    return root_->DeleteFile(action.body->ToString());
  }

  return arrow::Status::NotImplemented("Unknown action type: ", action.type);
}

arrow::Status SimpleServer::ListActions(
    const ServerCallContext& context,
    std::vector<ActionType>* actions
)
{
  std::cout << "ListActions" << std::endl;

  *actions = {kActionDropDataset};

  return arrow::Status::OK();
}

// ================================================================================================
// SimpleServer private impl
// ================================================================================================

arrow::Result<std::unique_ptr<SchemaResult>> SimpleServer::MakeSchema(
    const FileInfo& file_info
)
{
  ARROW_ASSIGN_OR_RAISE(auto input, root_->OpenInputFile(file_info));
  std::unique_ptr<parquet::arrow::FileReader> reader;
  ARROW_RETURN_NOT_OK(parquet::arrow::OpenFile(std::move(input), arrow::default_memory_pool(), &reader));

  std::shared_ptr<arrow::Schema> schema;
  ARROW_RETURN_NOT_OK(reader->GetSchema(&schema));

  return SchemaResult::Make(*schema);
}

arrow::Result<FlightInfo> SimpleServer::MakeFlightInfo(
    const FileInfo& file_info
)
{
  ARROW_ASSIGN_OR_RAISE(auto input, root_->OpenInputFile(file_info));
  std::unique_ptr<parquet::arrow::FileReader> reader;
  ARROW_RETURN_NOT_OK(parquet::arrow::OpenFile(std::move(input), arrow::default_memory_pool(), &reader));

  std::shared_ptr<arrow::Schema> schema;
  ARROW_RETURN_NOT_OK(reader->GetSchema(&schema));

  auto descriptor = FlightDescriptor::Path({file_info.base_name()});

  FlightEndpoint endpoint;
  endpoint.ticket.ticket = file_info.base_name();
  Location location;
  ARROW_ASSIGN_OR_RAISE(location, Location::ForGrpcTcp(HOST, port()));
  endpoint.locations.push_back(location);

  int64_t total_records = reader->parquet_reader()->metadata()->num_rows();
  int64_t total_bytes = file_info.size();

  return FlightInfo::Make(*schema, descriptor, {endpoint}, total_records, total_bytes);
}

arrow::Result<FileInfo> SimpleServer::FileInfoFromDescriptor(
    const FlightDescriptor& descriptor
)
{
  if (descriptor.type != FlightDescriptor::PATH)
    return arrow::Status::Invalid("Must provide PATH-type FlightDescriptor");
  else if (descriptor.path.size() != 1)
    return arrow::Status::Invalid("Must provide PATH-type FlightDescriptor with one path component");
  return root_->GetFileInfo(descriptor.path[0]);
}

// ================================================================================================
// Main
// ================================================================================================

arrow::Status Serve(std::string dataset_dir)
{

  auto fs = std::make_shared<LocalFileSystem>();
  ARROW_RETURN_NOT_OK(fs->CreateDir(dataset_dir));
  auto root = std::make_shared<SubTreeFileSystem>(dataset_dir, fs);

  Location server_location;
  ARROW_ASSIGN_OR_RAISE(server_location, Location::ForGrpcTcp(HOST, PORT));

  FlightServerOptions options(server_location);

  auto server = std::unique_ptr<FlightServerBase>(new SimpleServer(std::move(root)));
  ARROW_RETURN_NOT_OK(server->Init(options));

  std::cout << "🔈 Listening on port " << server->port() << std::endl;
  std::cout << "🏹 Serving " << dataset_dir << std::endl;

  ARROW_RETURN_NOT_OK(server->Serve());

  return arrow::Status::OK();
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
