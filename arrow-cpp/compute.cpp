/**
 * @file:	compute.cpp
 * @author:	Jacob Xie
 * @date:	2023/01/18 08:51:50 Wednesday
 * @brief:	Compute
 **/

#include <arrow/api.h>
#include <arrow/compute/api.h>

#include <iostream>

arrow::Result<std::shared_ptr<arrow::Table>> mock_table()
{
  // 创建若干 32-bit 整数 arrays
  arrow::Int32Builder int32builder;

  int32_t some_nums_raw[5]{34, 624, 2223, 5654, 4356};
  ARROW_RETURN_NOT_OK(int32builder.AppendValues(some_nums_raw, 5));
  std::shared_ptr<arrow::Array> some_nums;
  ARROW_ASSIGN_OR_RAISE(some_nums, int32builder.Finish());

  int32_t more_nums_raw[5] = {75342, 23, 64, 17, 736};
  ARROW_RETURN_NOT_OK(int32builder.AppendValues(more_nums_raw, 5));
  std::shared_ptr<arrow::Array> more_nums;
  ARROW_ASSIGN_OR_RAISE(more_nums, int32builder.Finish());

  // 用上述的 arrays 创建一个 table
  std::shared_ptr<arrow::Field> field_a, field_b;
  std::shared_ptr<arrow::Schema> schema;

  field_a = arrow::field("A", arrow::int32());
  field_b = arrow::field("B", arrow::int32());
  schema = arrow::schema({field_a, field_b});

  return arrow::Table::Make(schema, {some_nums, more_nums}, 5);
}

arrow::Status RunMain()
{
  // mock 函数生成一个 table（使用 Arrow 宏来进行赋值）
  ARROW_ASSIGN_OR_RAISE(std::shared_ptr<arrow::Table> table, mock_table());

  // ================================================================================================
  // A. 计算一个 Array 的 Sum
  // ================================================================================================
  //
  // 使用计算函数的时候主要有两个通常的步骤：
  // 1. 准备一个 `Datum` 用于输出
  // 2. 调用 `compute::Sum()`，一个便捷的函数用作于一个 Array 上做求和运算
  // 3. 获取结果并打印输出

  // 通过 Datum 来为输出准备内存
  // 当计算完成时，我们需要存储结果。在 Arrow 中，为输出准备的对象被称为 `Datum`。该对象用于计算函数中的传递输入与输出，
  // 且可以包含不同形状的 Arrow 数据结构。
  arrow::Datum sum;

  // 调用 Sum()
  // 使用之前准备的 `Table`，其中包含了 “A” 与 “B” 列。对于求和运输而言，有一个便捷的函数，称为 `compute::Sum()`，
  // 其减少了计算接口的复杂度。之后我们将学习更复杂的计算。对于给定的函数，请参考 https://arrow.apache.org/docs/cpp/api/compute.html
  // 来查阅是否有便捷的函数。`compute::Sum()` 需要一个 `Array` 或 `ChunkedArray` -- 这里使用了 `Table::GetColumnByName()`
  // 来传递列 A。接着它输入至 `Datum`：
  ARROW_ASSIGN_OR_RAISE(sum, arrow::compute::Sum({table->GetColumnByName("A")}));

  // 从 Datum 中获取结果
  // 之前的步骤留下了一个包含了 sum 结果的 `Datum`。然而我们并不能直接打印它 -- 它用于存储任意 Arrow 数据结构的灵活性
  // 意味着我们在获取数据时要非常小心。首先，需要知道它里面有什么，我们可以通过检查其是何种数据结构，接着是检查存储的是何种原始类型：
  std::cout << "Datum kind: " << sum.ToString() << " content type: " << sum.type()->ToString() << std::endl;
  // 上述应该报告 `Datum` 存储了一个 64-bit 整数类型的 `Scalar`：
  // 注意这里显式请求了一个标量 -- Datum 并不能直接返回它，用户需要带着正确类型来进行请求
  std::cout << sum.scalar_as<arrow::Int64Scalar>().value << std::endl;

  // ================================================================================================
  // B. 通过 CallFunction() 来计算元素向的 Array 加法
  // ================================================================================================
  //
  // 对于 `compute::Sum()` 所隐藏了复杂性的下一层是 `compute::CallFunction()`。下面的例子中我们将探索如何使用更为灵活的
  // 带有 “add” 计算函数的 `compute::CallFunction`。模式仍然相同：
  // 1. 准备一个 `Datum` 用于输出
  // 2. 带着 “add” 调用 `compute::CallFunction`
  // 3. 获取结果并打印输出

  // 通过 Datum 来为输出准备内存
  arrow::Datum element_wise_sum;

  // 带着 “add” 调用 `CallFunction()`
  // 对 Table 中的 A 与 B 列都进行元素向的求和。注意这里使用的是 CallFunction()，其将函数名作为第一入参
  // 访问 https://arrow.apache.org/docs/cpp/compute.html#compute-function-list 查阅更多有效函数
  ARROW_ASSIGN_OR_RAISE(
      element_wise_sum,
      arrow::compute::CallFunction("add", {table->GetColumnByName("A"), table->GetColumnByName("B")})
  );

  // 从 Datum 中获取结果
  std::cout << "Datum kind: " << element_wise_sum.ToString() << " content type: " << element_wise_sum.type()->ToString() << std::endl;
  // 由于其为 `ChunkedArray`，`Datum` 的 `ChunkedArray` 中拥有一个 `ChunkedArray::ToString()` 方法：
  std::cout << element_wise_sum.chunked_array()->ToString() << std::endl;

  // ================================================================================================
  // C. 通过 CallFunction() 以及 Options 来查找值
  // ================================================================================================
  //
  // `compute::CallFunction` 使用一个 vector 作为数据输入，但是计算过程中通常需要额外的参数。为此，计算函数可能会
  // 关联一些结构体，其中可以定义参数。用户可以检查给定的函数来确保哪个结构体是为其所用的。下面的例子中，我们将在列 “A” 中
  // 使用 “index” 计算函数来查找一个值。这个过程中有三个步骤与之前不同：
  // 1. 准备一个 `Datum` 用于输出
  // 2. 准备 `compute::IndexOptions`
  // 3. 带着 “index” 与 `compute::IndexOptions` 调用 `compute::CallFunction()`
  // 4. 获取结果并打印输出

  // 通过 Datum 来为输出准备内存
  arrow::Datum third_item;

  // 通过 IndexOptions 配置 “index”
  // 本次探索中我们将使用 “index” 函数 -- 这是一种查询方法，其返回输入值的索引。为此我们需要一个 `compute::IndexOptions` 结构体。
  arrow::compute::IndexOptions index_options;
  // 在查询函数中需要目标值。这里使用 2223，即列 A 中的第三个元素，并相应的配置结构体：
  index_options.value = arrow::MakeScalar(2223);

  // 带着 “index” 与 IndexOptions 来调用 CallFunction()
  ARROW_ASSIGN_OR_RAISE(
      third_item,
      arrow::compute::CallFunction("index", {table->GetColumnByName("A")}, &index_options)
  )

  // 从 Datum 中获取结果
  std::cout << "Datum kind: " << third_item.ToString()
            << " content type: " << third_item.type()->ToString() << std::endl;
  std::cout << third_item.scalar_as<arrow::Int64Scalar>().value << std::endl;

  return arrow::Status::OK();
}

int main(int argc, char** argv)
{
  arrow::Status st = RunMain();
  if (!st.ok())
  {
    std::cerr << st << std::endl;
    return 1;
  }

  return 0;
}
