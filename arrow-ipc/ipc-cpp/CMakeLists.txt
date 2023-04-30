
# ================================================================================================
# Lib
# ================================================================================================

add_library(
    ioipc
    write_ipc.cpp
    read_ipc.cpp
)
target_link_libraries(
    ioipc  PRIVATE
    Arrow::arrow_shared
    asio
)

# ================================================================================================
# Bin
# ================================================================================================

add_executable(
    arrowipc-dev
    dev.cpp
)
target_link_libraries(
    arrowipc-dev PRIVATE
    ioipc
    Arrow::arrow_shared
)
