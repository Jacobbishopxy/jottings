# @file:	flight_server.py
# @author:	Jacob Xie
# @date:	2023/05/09 22:48:27 Tuesday
# @brief:
#
# https://arrow.apache.org/cookbook/py/flight.html


import pathlib

import pyarrow as pa
import pyarrow.flight
import pyarrow.parquet


class FlightServer(pa.flight.FlightServerBase):
    def __init__(
        self,
        location="grpc+tcp://127.0.0.1:8815",
        repo=pathlib.Path("./datasets"),
        **kwargs
    ):
        super(FlightServer, self).__init__(location, **kwargs)
        self._location = location
        self._repo = repo

    def _make_flight_info(self, dataset):
        dataset_path = self._repo / dataset
        schema = pa.parquet.read_schema(dataset_path)
        metadata = pa.parquet.read_metadata(dataset_path)
        descriptor = pa.flight.FlightDescriptor.for_path(dataset.encode("utf-8"))
        endpoints = [pa.flight.FlightEndpoint(dataset, [self._location])]
        return pa.flight.FlightInfo(
            schema, descriptor, endpoints, metadata.num_rows, metadata.serialized_size
        )

    def list_flights(self, context, criteria):
        for dataset in self._repo.iterdir():
            yield self._make_flight_info(dataset.name)

    def get_flight_info(self, context, descriptor):
        return self._make_flight_info(descriptor.path[0].decode("utf-8"))

    def do_put(self, context, descriptor, reader, writer):
        dataset = descriptor.path[0].decode("utf-8")
        dataset_path = self._repo / dataset
        data_table = reader.read_all()
        pa.parquet.write_table(data_table, dataset_path)

    def do_get(self, context, ticket):
        dataset = ticket.ticket.decode("utf-8")
        dataset_path = self._repo / dataset
        return pa.flight.RecordBatchStream(pa.parquet.read_table(dataset_path))

    def list_actions(self, context):
        return [("drop_dataset", "Delete a dataset.")]

    def do_action(self, context, action):
        if action.type == "drop_dataset":
            self.do_drop_dataset(action.body.to_pybytes().decode("utf-8"))
        else:
            raise NotImplementedError

    def do_drop_dataset(self, dataset):
        dataset_path = self._repo / dataset
        dataset_path.unlink()


if __name__ == "__main__":
    server = FlightServer()
    server._repo.mkdir(exist_ok=True)
    server.serve()

    pass
