
add_executable(arrowcpp dev.cpp)

target_link_libraries(arrowcpp PRIVATE Catch2::Catch2WithMain)
