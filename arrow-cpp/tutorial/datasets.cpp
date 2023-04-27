/**
 * @file:	datasets.cpp
 * @author:	Jacob Xie
 * @date:	2023/01/19 17:24:46 Thursday
 * @brief:	Datasets
 **/

#include <arrow/api.h>
#include <arrow/dataset/api.h>
#include <parquet/arrow/reader.h>
#include <parquet/arrow/writer.h>

#include <iostream>
#include <tuple>
#include <unistd.h>

// ================================================================================================
// A. 生成读取所需要的文件
// ================================================================================================

// 生成 Table
arrow::Result<std::shared_ptr<arrow::Table>> CreateTable()
{
  auto schema = arrow::schema({
      arrow::field("a", arrow::int64()),
      arrow::field("b", arrow::int64()),
      arrow::field("c", arrow::int64()),
  });

  std::shared_ptr<arrow::Array> array_a;
  std::shared_ptr<arrow::Array> array_b;
  std::shared_ptr<arrow::Array> array_c;

  arrow::NumericBuilder<arrow::Int64Type> builder;

  ARROW_RETURN_NOT_OK(builder.AppendValues({0, 1, 2, 3, 4, 5, 6, 7, 8, 9}));
  ARROW_RETURN_NOT_OK(builder.Finish(&array_a));
  builder.Reset();
  ARROW_RETURN_NOT_OK(builder.AppendValues({9, 8, 7, 6, 5, 4, 3, 2, 1, 0}));
  ARROW_RETURN_NOT_OK(builder.Finish(&array_b));
  builder.Reset();
  ARROW_RETURN_NOT_OK(builder.AppendValues({1, 2, 1, 2, 1, 2, 1, 2, 1, 2}));
  ARROW_RETURN_NOT_OK(builder.Finish(&array_c));

  return arrow::Table::Make(schema, {array_a, array_b, array_c});
}

// 编写两个 Parquet 文件来构建 dataset
arrow::Result<std::string> CreateExampleParquetDataset(
    const std::shared_ptr<arrow::fs::FileSystem>& filesystem,
    const std::string& root_path
)
{
  auto base_path = root_path + "parquet_dataset";
  ARROW_RETURN_NOT_OK(filesystem->CreateDir(base_path));
  ARROW_ASSIGN_OR_RAISE(auto table, CreateTable());
  ARROW_ASSIGN_OR_RAISE(auto output, filesystem->OpenOutputStream(base_path + "/data1.parquet"));
  ARROW_RETURN_NOT_OK(parquet::arrow::WriteTable(
      *table->Slice(0, 5), arrow::default_memory_pool(), output, 2048
  ));
  ARROW_ASSIGN_OR_RAISE(output, filesystem->OpenOutputStream(base_path + "/data2.parquet"));
  ARROW_RETURN_NOT_OK(parquet::arrow::WriteTable(
      *table->Slice(5), arrow::default_memory_pool(), output, 2048
  ));

  return base_path;
}

// 配置环境
arrow::Status PrepareEnv()
{
  ARROW_ASSIGN_OR_RAISE(auto src_table, CreateTable());
  std::shared_ptr<arrow::fs::FileSystem> setup_fs;
  // 主要要在可执行程序的目录中运行
  char setup_path[256];
  char* result = getcwd(setup_path, 256);
  if (result == NULL)
  {
    return arrow::Status::IOError("Fetching PWD failed.");
  }

  ARROW_ASSIGN_OR_RAISE(setup_fs, arrow::fs::FileSystemFromUriOrPath(setup_path));
  ARROW_ASSIGN_OR_RAISE(auto dset_path, CreateExampleParquetDataset(setup_fs, ""));

  return arrow::Status::OK();
}

// ================================================================================================
// B. 读取分区的 Dataset
// ================================================================================================
//
// 读取 Dataset 的任务与读取单个文件不同，这个过程可以被拆解为以下四个步骤：
// 1. 在本地文件系统中获取一个 `fs::FileSystem` 对象
// 2. 创建一个 `fs::FileSelector` 并使用它来准备一个 `dataset::FileSystemDatasetFactory`
// 3. 使用 `dataset::FileSystemDatasetFactory` 构建一个 `dataset::Dataset`
// 4. 使用一个 `dataset::Scanner` 读取至一个 `Table`

using ReadResult = arrow::Result<std::tuple<std::shared_ptr<arrow::Table>, std::shared_ptr<arrow::fs::FileSystem>>>;

ReadResult ReadDataset()
{
  // 准备一个 FileSystem 对象
  //
  // 一个 `fs::FileSystem` 是一个抽象，其允许用户使用相同的接口而不需要关注使用的是 Amazon S3，
  // Google 云存储，或者本地磁盘 -- 本例使用本地磁盘：
  std::shared_ptr<arrow::fs::FileSystem> fs;
  // `fs::FileSystemFromUriOrPath()` 允许我们获取任意一个支持类型的文件系统的 `fs::FileSystem` 对象
  // 访问 https://arrow.apache.org/docs/cpp/api/filesystem.html#_CPPv4N5arrow2fs10FileSystemE
  // 以查阅更多支持的文件系统
  char init_path[256];
  char* result = getcwd(init_path, 256);
  if (result == NULL)
  {
    return arrow::Status::IOError("Fetching PWD failed.");
  }
  ARROW_ASSIGN_OR_RAISE(fs, arrow::fs::FileSystemFromUriOrPath(init_path));

  // 创建一个 FileSystemDatasetFactory
  //
  // 一个 `fs::FileSystem` 存储了大量的元数据，但是我们需要遍历它并进行解析。在 Arrow 中使用一个
  // `FileSelector` 来完成若干文件 dataset 的遍历
  arrow::fs::FileSelector selector;
  // 该 `fs::FileSystem` 暂且还不能做任何事情，需要配置它才能使用：
  selector.base_dir = "parquet_dataset";
  // 如果在不了解 dataset 的嵌套环境下，递归是安全的
  selector.recursive = true;
  // 从一个 `fs::FileSystem` 中获取一个 `dataset::Dataset` 需要先准备一个 `dataset::FileSystemDatasetFactory`。
  // 它会创建一个工厂从 `fs::FileSystem` 中获取数据。首先需要填写一个 `dataset::FileSystemFactoryOptions`
  // 的结构体来配置它：
  arrow::dataset::FileSystemFactoryOptions options;
  // 使用 Hive 风格的分区，将 Arrow Datasets 指向分区 schema。这里不需要其他选项，默认的就够了
  options.partitioning = arrow::dataset::HivePartitioning::MakeFactory();
  // 有很多种文件格式，我们需要在读取时挑选出合适的那个。Parquet 是磁盘的文件，因此选中它：
  auto read_format = std::make_shared<arrow::dataset::ParquetFileFormat>();
  // 设置完 `fs::FileSystem`，`fs::FileSelector`，选项，以及文件格式后，就可以创建 `dataset::FileSystemDatasetFactory`
  ARROW_ASSIGN_OR_RAISE(
      auto factory,
      arrow::dataset::FileSystemDatasetFactory::Make(fs, selector, read_format, options)
  );

  // 使用 Factory 构建 Dataset
  //
  // 类似于 `ArrayBuilder` 的方式
  ARROW_ASSIGN_OR_RAISE(auto read_dataset, factory->Finish());
  // 现在有了一个 `dataset::Dataset` 对象在内存里，这并不意味着整个 dataset 被加载到内存了，
  // 可以通过工具来进行访问。例如可以获取部分（文件）打印它们，同时带上一些信息：
  ARROW_ASSIGN_OR_RAISE(auto fragments, read_dataset->GetFragments());
  for (const auto& fragment : fragments)
  {
    std::cout << "Found fragment: " << (*fragment)->ToString() << std::endl;
    std::cout << "Partition expression: " << (*fragment)->partition_expression().ToString() << std::endl;
  }

  // 移动 Dataset 至 Table
  //
  // 访问 https://arrow.apache.org/docs/cpp/streaming_execution.html 查阅如何避免加载整个 dataset 至内存。
  // 为了将 `Dataset` 的内容移至一个 `Table` 内，我们需要一个 `dataset::Scanner`，其扫描数据并输出至 `Table`。
  ARROW_ASSIGN_OR_RAISE(auto read_scan_builder, read_dataset->NewScan());
  // Builder 需要 `Finish()` 函数
  ARROW_ASSIGN_OR_RAISE(auto read_scanner, read_scan_builder->Finish());
  // 现在需要一个工具来移动 `dataset::Dataset`。`dataset::Scanner::ToTable()` 提供了这个功能
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::Table> table, read_scanner->ToTable());
  std::cout << table->ToString() << std::endl;
  // 再次声明，如果不需要移动至一个 `Table` 的时候，考虑使用 `Acero`。

  return std::make_tuple(table, fs);
}

// ================================================================================================
// C. 从 Table 写 Dataset 至硬盘
// ================================================================================================
//
// 写 `dataset::Dataset` 与写单个文件不同，这个过程可以被拆解为以下五个步骤：
// 1. 准备一个 `TableBatchReader`
// 2. 创建一个 `dataset::Scanner` 从 `TableBatchReader` 中拉取数据
// 3. 准备 schema，分区，以及文件格式选项
// 4. 设置 `dataset::FileSystemDatasetWriteOptions` -- 一个用于配置写函数的结构体
// 5. 写 dataset 至硬盘

arrow::Status WriteDataset(std::shared_ptr<arrow::Table> table, std::shared_ptr<arrow::fs::FileSystem> fs)
{
  // 准备需要写的 Table
  //
  // 现在有了一个 `Table` 想要写入磁盘中。首先需要一个 `TableBatchReader`，它使得写入 `Dataset` 变得简单，
  // 同时可以被用于拆分 `Table` 至流式的 `RecordBatches`。这里使用了 `TableBatchReader` 的构造函数：
  std::shared_ptr<arrow::TableBatchReader> write_dataset = std::make_shared<arrow::TableBatchReader>(table);

  // 创建 Scanner 用于移动 Table 数据
  //
  // 这个过程用于写一个 `dataset::Dataset`，一旦数据源可用，其类似于读取的相反过程。之前使用了一个 `dataset::Scanner`
  // 来扫描至一个 `Table` -- 现在我们需要从 `TableBatchReader` 中读取数据。
  auto write_scanner_builder = arrow::dataset::ScannerBuilder::FromRecordBatchReader(write_dataset);
  ARROW_ASSIGN_OR_RAISE(auto write_scanner, write_scanner_builder->Finish());

  // 准备 Schema，分区，以及文件格式这些变量
  //
  // 因为我们想根据 “a” 列来进行分区，需要进行声明。当定义分区 `Schema` 时，需要一个 `Field` 包含 “a”：
  auto partition_schema = arrow::schema({arrow::field("a", arrow::utf8())});
  // 该 `Schema` 定义了哪个键作为分区，但是还需要选择分区算法。这里使用的是 Hive 风格，这次将 schema 传入配置：
  auto partitioning = std::make_shared<arrow::dataset::HivePartitioning>(partition_schema);
  // 若干文件格式可用，不过 Parquet 是 Arrow 常用的
  auto write_format = std::make_shared<arrow::dataset::ParquetFileFormat>();

  // 配置 FileSystemDatasetWriteOptions
  //
  // 还需要一些配置才能写入磁盘：设置 `dataset::FileSystemDatasetWriteOptions` 结构体
  arrow::dataset::FileSystemDatasetWriteOptions write_options;
  // 使用默认
  write_options.file_write_options = write_format->DefaultWriteOptions();
  // 写入文件还有一个重要的步骤就是设置 `fs::FileSystem`，可以使用之前读取时候的 fs。
  write_options.filesystem = fs;
  // Arrow 可以创建路径
  write_options.base_dir = "write_dataset";
  // 设置分区
  write_options.partitioning = partitioning;
  // 设置分区的文件命名方式
  write_options.basename_template = "part{i}.parquet";
  // 有时数据会被多次写入到同一个地方，那么覆盖是可以接受的
  write_options.existing_data_behavior = arrow::dataset::ExistingDataBehavior::kOverwriteOrIgnore;

  // 写 Dataset 至硬盘
  //
  // 一旦 `dataset::FileSystemDatasetWriteOptions` 被配置好了，同时一个 `dataset::Scanner` 也准备好用于解析数据了，
  // 我们可以将它们传入 `dataset::FileSystemDataset::Write()` 函数中进行数据写入至硬盘了：
  ARROW_RETURN_NOT_OK(arrow::dataset::FileSystemDataset::Write(write_options, write_scanner));

  return arrow::Status::OK();
}

arrow::Status RunMain()
{
  // 生成 mock 文件
  ARROW_RETURN_NOT_OK(PrepareEnv());

  // 读文件
  ARROW_ASSIGN_OR_RAISE(auto res, ReadDataset());
  auto [table, fs] = res;

  // 写文件
  ARROW_RETURN_NOT_OK(WriteDataset(table, fs));

  return arrow::Status::OK();
}

int main()
{
  arrow::Status st = RunMain();
  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }
  return 0;
}
