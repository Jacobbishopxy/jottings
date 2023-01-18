
find_package(Arrow REQUIRED)
find_package(Parquet REQUIRED)

# Starter
add_executable(
    arrowcpp-starter
    starter.cpp
)
target_link_libraries(
    arrowcpp-starter PRIVATE
    Arrow::arrow_shared
)

# File IO
add_executable(
    arrowcpp-fileio
    file_io.cpp
)
target_link_libraries(
    arrowcpp-fileio PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
)

# Compute
add_executable(
    arrowcpp-compute
    compute.cpp
)
target_link_libraries(
    arrowcpp-compute PRIVATE
    Arrow::arrow_shared
)

