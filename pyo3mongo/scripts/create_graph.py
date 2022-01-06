# Pyo3 Mongo

from typing import List, Optional
from pydantic import BaseModel
import p3m

# printable JSON model
class Vertex(BaseModel):
    id: str
    name: str


# printable JSON model
class Edge(BaseModel):
    id: str
    source: str
    target: str
    weight: Optional[float]
    label: Optional[str]


class Graph(BaseModel):
    vertexes: List[Vertex]
    edges: List[Edge]


# turn vertex returned by Mongo into a printable model
def into_vertex(vertex):
    return Vertex(id=vertex.id, name=vertex.name)


# turn edge returned by Mongo into a printable model
def into_edge(edge):
    return Edge(
        id=edge.id,
        source=edge.source,
        target=edge.target,
        weight=edge.weight,
        label=edge.label,
    )


def into_graph(graph):
    vertexes = [into_vertex(v) for v in graph.vertexes]
    edges = [into_edge(e) for e in graph.edges]
    return Graph(vertexes=vertexes, edges=edges)


if __name__ == "__main__":

    uri = "mongodb://root:secret@localhost:27017"
    db = "graph"
    cat = "dev"

    # initialize PyGraph
    py_graph = p3m.PyGraph(uri, db, cat)

    # create 3 vertexes

    vertex1 = py_graph.create_vertex("node1")
    print(into_vertex(vertex1).json())

    vertex2 = py_graph.create_vertex("node2")
    print(into_vertex(vertex2).json())

    vertex3 = py_graph.create_vertex("node3")
    print(into_vertex(vertex3).json())

    # create 3 edges: node1 -> node2 -> node3 -> node1

    edge_dto1 = p3m.EdgeInput(
        source=vertex1.id, target=vertex2.id, weight=1.0, label="circuit"
    )
    edge1 = py_graph.create_edge(edge_dto1)
    print(into_edge(edge1).json())

    edge_dto2 = p3m.EdgeInput(
        source=vertex2.id, target=vertex3.id, weight=1.2, label="circuit"
    )
    edge2 = py_graph.create_edge(edge_dto2)
    print(into_edge(edge2).json())

    edge_dto3 = p3m.EdgeInput(
        source=vertex1.id, target=vertex3.id, weight=1.3, label="circuit"
    )
    edge3 = py_graph.create_edge(edge_dto3)
    print(into_edge(edge3).json())

    # query graph, will get following:
    # vertexes: [node2, node3]
    # edges: [edge1, edge2, edge3]

    graph = py_graph.get_graph(vertex1.id, "circuit", None)
    print(into_graph(graph).json())
