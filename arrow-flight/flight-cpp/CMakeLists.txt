

add_executable(
    flight-server
    flight_server.cpp
)
target_link_libraries(
    flight-server PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
    ArrowFlight::arrow_flight_shared
)

add_executable(
    flight-client
    flight_client.cpp
)
target_link_libraries(
    flight-client PRIVATE
    Arrow::arrow_shared
    Parquet::parquet_shared
    ArrowFlight::arrow_flight_shared
)
