/**
 * @file:	flight_client.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/10 20:57:54 Wednesday
 * @brief:
 **/

#include <iostream>
#include <memory>
#include <thread>

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
// SimpleClient
// ================================================================================================

class SimpleClient
{
public:
  SimpleClient() {}

  arrow::Status Connect(arrow::flight::Location location, std::string dataset_dir);

  arrow::Status ListFlights();

  arrow::Status DoPut(std::string path);

private:
  std::unique_ptr<arrow::flight::FlightClient> client_;
  std::shared_ptr<arrow::fs::LocalFileSystem> fs_;
  std::shared_ptr<arrow::fs::SubTreeFileSystem> root_;
};

// ================================================================================================
// SimpleClient impl
// ================================================================================================

arrow::Status SimpleClient::Connect(arrow::flight::Location location, std::string dataset_dir)
{
  ARROW_ASSIGN_OR_RAISE(client_, arrow::flight::FlightClient::Connect(location));
  std::cout << "Connected to " << location.ToString() << std::endl;

  fs_ = std::make_shared<arrow::fs::LocalFileSystem>();
  root_ = std::make_shared<arrow::fs::SubTreeFileSystem>(dataset_dir, fs_);

  return arrow::Status::OK();
}

arrow::Status SimpleClient::ListFlights()
{
  std::cout << "Listing Flights..." << std::endl;

  std::unique_ptr<arrow::flight::FlightListing> listing;

  ARROW_ASSIGN_OR_RAISE(listing, client_->ListFlights());

  std::unique_ptr<arrow::flight::FlightInfo> flight_info;
  ARROW_ASSIGN_OR_RAISE(flight_info, listing->Next());
  if (!flight_info)
    return arrow::Status::OK();
  std::cout << flight_info->descriptor().ToString() << std::endl;

  return arrow::Status::OK();
}

arrow::Status SimpleClient::DoPut(std::string path)
{
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::io::RandomAccessFile> input, root_->OpenInputFile(path));

  std::unique_ptr<parquet::arrow::FileReader> reader;

  ARROW_RETURN_NOT_OK(parquet::arrow::OpenFile(std::move(input), arrow::default_memory_pool(), &reader));

  auto descriptor = arrow::flight::FlightDescriptor::Path({path});

  std::shared_ptr<arrow::Schema> schema;
  ARROW_RETURN_NOT_OK(reader->GetSchema(&schema));

  std::unique_ptr<arrow::flight::FlightStreamWriter> writer;
  ARROW_ASSIGN_OR_RAISE(auto put_stream, client_->DoPut(descriptor, schema));
  writer = std::move(put_stream.writer);

  // Upload data
  std::shared_ptr<arrow::RecordBatchReader> batch_reader;
  std::vector<int> row_groups(reader->num_row_groups());
  std::iota(row_groups.begin(), row_groups.end(), 0);
  ARROW_RETURN_NOT_OK(reader->GetRecordBatchReader(row_groups, &batch_reader));

  int64_t batches = 0;
  while (true)
  {
    ARROW_ASSIGN_OR_RAISE(auto batch, batch_reader->Next());
    if (!batch)
      break;
    ARROW_RETURN_NOT_OK(writer->WriteRecordBatch(*batch));
    batches++;
  }

  ARROW_RETURN_NOT_OK(writer->Close());
  std::cout << "Wrote " << batches << " batch(es)" << std::endl;

  return arrow::Status::OK();
}

void sleep_for(double sec)
{
  std::this_thread::sleep_for(
      std::chrono::nanoseconds(static_cast<int64_t>(sec * 1e9))
  );
}

arrow::Status Run(std::string filename, std::string dataset_dir)
{
  arrow::flight::Location location;
  ARROW_ASSIGN_OR_RAISE(location, arrow::flight::Location::ForGrpcTcp(HOST, PORT));

  auto client = new SimpleClient();
  ARROW_RETURN_NOT_OK(client->Connect(location, dataset_dir));

  // while (true)
  {
    arrow::Status st = client->DoPut(filename);

    if (!st.ok())
      std::cout << "DoPut failed: " << st.message() << std::endl;
    else
      std::cout << "DoPut Succeeded!" << std::endl;
  }

  sleep_for(1);

  return arrow::Status::OK();
}

int main(int argc, char** argv)
{
  signal(SIGTERM, [](int i)
         { exit(i); });

  std::string dataset_dir = "./datasets";

  arrow::Status st = Run("uploaded.parquet", dataset_dir);

  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }

  return 0;
}
