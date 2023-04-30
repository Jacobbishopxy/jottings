cmake_minimum_required(VERSION 3.14)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(Jotting LANGUAGES CXX VERSION 0.0.1)


find_package(Arrow REQUIRED)
find_package(ArrowDataset REQUIRED)
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

  if(WIN32)
    # macro see @ https://stackoverflow.com/a/40217291/1746503
    macro(get_win32_winnt version)
      if(CMAKE_SYSTEM_VERSION)
        set(ver ${CMAKE_SYSTEM_VERSION})
        string(REGEX MATCH "^([0-9]+).([0-9])" ver ${ver})
        string(REGEX MATCH "^([0-9]+)" verMajor ${ver})
        # Check for Windows 10, b/c we'll need to convert to hex 'A'.
        if("${verMajor}" MATCHES "10")
          set(verMajor "A")
          string(REGEX REPLACE "^([0-9]+)" ${verMajor} ver ${ver})
        endif("${verMajor}" MATCHES "10")
        # Remove all remaining '.' characters.
        string(REPLACE "." "" ver ${ver})
        # Prepend each digit with a zero.
        string(REGEX REPLACE "([0-9A-Z])" "0\\1" ver ${ver})
        set(${version} "0x${ver}")
      endif()
    endmacro()

    if(NOT DEFINED _WIN32_WINNT)
      get_win32_winnt(ver)
      set(_WIN32_WINNT ${ver})
    endif()

    message(STATUS "Set _WIN32_WINNET=${_WIN32_WINNT}")

    target_compile_definitions(asio INTERFACE _WIN32_WINNT=${_WIN32_WINNT} WIN32_LEAN_AND_MEAN)
  endif()
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
