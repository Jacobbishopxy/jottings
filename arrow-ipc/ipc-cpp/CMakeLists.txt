
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

# IPC File
add_executable(
    ipc_file
    main_ipc_file.cpp
)
target_link_libraries(
    ipc_file PRIVATE
    ioipc
    Arrow::arrow_shared
)

# Sync TCP server
add_executable(
    sync-tcp-dt-server
    sync_tcp_daytime_server.cpp
)
target_link_libraries(
    sync-tcp-dt-server PRIVATE
    asio
)

# Sync TCP client
add_executable(
    sync-tcp-dt-client
    sync_tcp_daytime_client.cpp
)
target_link_libraries(
    sync-tcp-dt-client PRIVATE
    asio
)

# Async TCP server
add_executable(
    async-tcp-dt-server
    async_tcp_daytime_server.cpp
)
target_link_libraries(
    async-tcp-dt-server PRIVATE
    asio
)

# IPC stream server
add_executable(
    ipc_stream_server
    main_ipc_stream_server.cpp
)
target_link_libraries(
    ipc_stream_server
    Arrow::arrow_shared
    asio
)

# IPC stream client
add_executable(
    ipc_stream_client
    main_ipc_stream_client.cpp
)
target_link_libraries(
    ipc_stream_client
    Arrow::arrow_shared
    asio
)
