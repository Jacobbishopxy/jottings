
add_executable(
    arrowcpp-starter
    starter.cpp
)

find_package(Arrow REQUIRED)

target_link_libraries(
    arrowcpp-starter PRIVATE
    Arrow::arrow_shared
)
