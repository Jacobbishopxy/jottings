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
# Advanced
# ================================================================================================

add_executable(
    arrowcpp-flight-server
    advanced/flight_server.cpp
)
target_link_libraries(
    arrowcpp-flight-server PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
    ArrowFlight::arrow_flight_shared
)

add_executable(
    arrowcpp-flight-client
    advanced/flight_client.cpp
)
target_link_libraries(
    arrowcpp-flight-client PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
    ArrowFlight::arrow_flight_shared
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
