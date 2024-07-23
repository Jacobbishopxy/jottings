cmake_minimum_required(VERSION 3.14)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(Jotting LANGUAGES CXX VERSION 0.0.1)


find_package(Arrow REQUIRED)
find_package(ArrowDataset REQUIRED)
find_package(ArrowFlight REQUIRED)
find_package(Parquet REQUIRED)

# ================================================================================================
# CPM.cmake
# https://github.com/cpm-cmake/CPM.cmake
# ================================================================================================

# add package mananger
include(./cmake/CPM.cmake)

find_package(Threads REQUIRED)

# catch2
CPMAddPackage("gh:catchorg/Catch2@3.2.1")

# asio
# https://github.com/cpm-cmake/CPM.cmake/blob/master/examples/asio-standalone/CMakeLists.txt
CPMAddPackage("gh:chriskohlhoff/asio#asio-1-28-0@1.28.0")

if(asio_ADDED)
  add_library(asio INTERFACE)

  target_include_directories(asio SYSTEM INTERFACE ${asio_SOURCE_DIR}/asio/include)

  target_compile_definitions(asio INTERFACE ASIO_STANDALONE ASIO_NO_DEPRECATED)

  target_link_libraries(asio INTERFACE Threads::Threads)

endif()


# ================================================================================================
# Sub dir
# ================================================================================================


add_subdirectory(./lambda-comparison/lambda-cpp lbd)
add_subdirectory(./arrow-cpp arw)
add_subdirectory(./fsm/fsm-cpp fsm)
add_subdirectory(./ptr ptr)
add_subdirectory(./stl-cpp stlcpp)
add_subdirectory(./arrow-ipc/ipc-cpp arwipc)
add_subdirectory(./arrow-flight/flight-cpp arwflt)
add_subdirectory(./traits-cpp tcpp)

