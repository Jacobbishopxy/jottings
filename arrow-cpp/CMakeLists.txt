
find_package(Arrow REQUIRED)
find_package(ArrowDataset REQUIRED)
find_package(Parquet REQUIRED)

# ================================================================================================
# Tutorial
# ================================================================================================

# Starter
add_executable(
    arrowcpp-starter
    tutorial/starter.cpp
)
target_link_libraries(
    arrowcpp-starter PRIVATE
    Arrow::arrow_shared
)

# File IO
add_executable(
    arrowcpp-fileio
    tutorial/file_io.cpp
)
target_link_libraries(
    arrowcpp-fileio PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
)

# Compute
add_executable(
    arrowcpp-compute
    tutorial/compute.cpp
)
target_link_libraries(
    arrowcpp-compute PRIVATE
    Arrow::arrow_shared
)

# Datasets
add_executable(
    arrowcpp-datasets
    tutorial/datasets.cpp
)
target_link_libraries(
    arrowcpp-datasets PRIVATE
    Arrow::arrow_shared
    ArrowDataset::arrow_dataset_shared
    Parquet::parquet_shared
)

# ================================================================================================
# Demo
# ================================================================================================

add_executable(
    arrowcpp-oad
    demo/OrderAD.cpp
    demo/lb_pqt.cpp
)
target_link_libraries(
    arrowcpp-oad PRIVATE
    Arrow::arrow_shared
    ArrowDataset::arrow_dataset_shared
    Parquet::parquet_shared
)
