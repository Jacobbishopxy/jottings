# @file:	flight_client.py
# @author:	Jacob Xie
# @date:	2023/05/09 23:49:12 Tuesday
# @brief:

import pyarrow as pa
import pyarrow.flight

if __name__ == "__main__":
    client = pa.flight.connect("grpc+tcp://127.0.0.1:8815")

    # Upload a new dataset
    data_table = pa.table([["Mario", "Luigi", "Peach"]], names=["Character"])
    upload_descriptor = pa.flight.FlightDescriptor.for_path("uploaded.parquet")
    writer, _ = client.do_put(upload_descriptor, data_table.schema)
    writer.write_table(data_table)
    writer.close()

    # Retrieve metadata of newly uploaded dataset
    flight = client.get_flight_info(upload_descriptor)
    descriptor = flight.descriptor
    print(
        "Path:",
        descriptor.path[0].decode("utf-8"),
        "Rows:",
        flight.total_records,
        "Size:",
        flight.total_bytes,
    )
    print("=== Schema ===")
    print(flight.schema)
    print("==============")

    # Read content of the dataset
    reader = client.do_get(flight.endpoints[0].ticket)
    read_table = reader.read_all()
    print(read_table.to_pandas().head())

    # Drop the newly uploaded dataset
    client.do_action(
        pa.flight.Action("drop_dataset", "uploaded.parquet".encode("utf-8"))
    )

    # List existing datasets.
    for flight in client.list_flights():
        descriptor = flight.descriptor
        print(
            "Path:",
            descriptor.path[0].decode("utf-8"),
            "Rows:",
            flight.total_records,
            "Size:",
            flight.total_bytes,
        )
        print("=== Schema ===")
        print(flight.schema)
        print("==============")
        print("")
