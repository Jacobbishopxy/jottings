
# ================================================================================================
# Lib
# ================================================================================================

add_library(
    ioipc
    ipc_write.cpp
    ipc_read.cpp
    ipc_utils.cpp
)
target_link_libraries(
    ioipc  PUBLIC
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
)

# TCP case #1: Sync TCP server
add_executable(
    tcp-dt-server-sync
    tcp_daytime_server_sync.cpp
)
target_link_libraries(
    tcp-dt-server-sync PRIVATE
    asio
)

# TCP case #2: Sync TCP client
add_executable(
    tcp-dt-client-sync
    tcp_daytime_client_sync.cpp
)
target_link_libraries(
    tcp-dt-client-sync PRIVATE
    asio
)

# TCP case #3: Async TCP server
add_executable(
    tcp-dt-server-async
    tcp_daytime_server_async.cpp
)
target_link_libraries(
    tcp-dt-server-async PRIVATE
    asio
)

# IPC stream server
add_executable(
    ipc_stream_server
    main_ipc_stream_server.cpp
)
target_link_libraries(
    ipc_stream_server
    ioipc
)

# IPC stream client
add_executable(
    ipc_stream_client
    main_ipc_stream_client.cpp
)
target_link_libraries(
    ipc_stream_client
    ioipc
)
