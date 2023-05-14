# @file:	flight_client.py
# @author:	Jacob Xie
# @date:	2023/05/09 23:49:12 Tuesday
# @brief:

import pyarrow as pa
import pyarrow.flight

if __name__ == "__main__":
    client = pa.flight.connect("grpc+tcp://127.0.0.1:8815")

    # Upload a new dataset
    upload_descriptor = pa.flight.FlightDescriptor.for_path("uploaded.parquet")
    # (a) non-streaming
    # data_table = pa.table([["Mario", "Luigi", "Peach"]], names=["Character"])
    # writer, _ = client.do_put(upload_descriptor, data_table.schema)
    # writer.write_table(data_table)
    # writer.close()

    # (b) streaming
    NUM_BATCHES = 1024
    ROWS_PER_BATCH = 4096
    batch = pa.record_batch([pa.array(range(ROWS_PER_BATCH))], names=["ints"])
    writer, _ = client.do_put(upload_descriptor, batch.schema)
    with writer:
        for _ in range(NUM_BATCHES):
            writer.write_batch(batch)

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
    # (a) non-streaming
    # read_table = reader.read_all()
    # print(read_table.to_pandas().head())

    # (b) streaming
    total_rows = 0
    for chunk in reader:
        total_rows += chunk.data.num_rows
    print("Got ", total_rows, "rows total, expected ", NUM_BATCHES * ROWS_PER_BATCH)

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
