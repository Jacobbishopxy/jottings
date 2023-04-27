
find_package(Arrow REQUIRED)

add_executable(
    arrowipc-dev
    dev.cpp
)
target_link_libraries(
    arrowipc-dev PRIVATE
    Arrow::arrow_shared
)
