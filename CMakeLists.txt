cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(Jotting LANGUAGES CXX VERSION 0.0.1)

find_package(Catch2 REQUIRED)

add_subdirectory(./lambda-comparison/lambda-cpp lbd)
add_subdirectory(./arrow-cpp arw)
add_subdirectory(./fsm/fsm-cpp fsm)
add_subdirectory(./ptr ptr)
add_subdirectory(./stl-cpp stlcpp)
add_subdirectory(./arrow-ipc/ipc-cpp arwipc)
