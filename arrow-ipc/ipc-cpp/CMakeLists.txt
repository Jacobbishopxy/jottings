
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

add_executable(
    sync-tcp-dt-server
    sync_tcp_daytime_server.cpp
)
target_link_libraries(
    sync-tcp-dt-server PRIVATE
    asio
)

add_executable(
    sync-tcp-dt-client
    sync_tcp_daytime_client.cpp
)
target_link_libraries(
    sync-tcp-dt-client PRIVATE
    asio
)

add_executable(
    async-tcp-dt-server
    async_tcp_daytime_server.cpp
)
target_link_libraries(
    async-tcp-dt-server PRIVATE
    asio
)
