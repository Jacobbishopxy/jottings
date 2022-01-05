# Pyo3 Mongo

from typing import Optional
import p3m
from pydantic import BaseModel


class VertexDto(BaseModel):
    name: str


class EdgeDto(BaseModel):
    source: str
    target: str
    weight: Optional[float]
    label: Optional[str]


if __name__ == "__main__":

    uri = "mongodb://root:secret@localhost:27017"
    db = "graph"
    cat = "dev"

    print("Connecting to mongo...")

    py_graph = p3m.PyGraph(uri, db, cat)

    print("Creating vertices...")

    vertex1 = py_graph.create_vertex("node1")
    print(vertex1.id)

    vertex2 = py_graph.create_vertex("node2")
    print(vertex2.id)

    # BUG: AttributeError: 'builtins.EdgeInput' object has no attribute 'source'
    edge_dto = p3m.EdgeInput(source=vertex1.id, target=vertex2.id, weight=1.0, label="edge1")
    edge = py_graph.create_edge(edge_dto)
    print(edge.id)
