//! Service
//!

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, Document};
use mongodb::options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern};
use mongodb::Collection;
use tokio_stream::StreamExt;

use super::db::MongoClient;
use super::model::{Edge, EdgeDto, FindEdgeByVertexDto, Vertex, VertexDto};
use super::{Pyo3MongoError, Pyo3MongoResult};

/// The graphService is responsible for creating and deleting vertices and edges.
///
/// A graphService contains two collections:
/// 1. ${cat}_vertex
/// 1. ${cat}_edge
#[allow(dead_code)]
pub struct GraphService {
    client: MongoClient,
    cat: String,
}

#[allow(dead_code)]
impl GraphService {
    pub async fn new(uri: &str, cat: &str) -> Pyo3MongoResult<Self> {
        Ok(GraphService {
            client: MongoClient::new(uri).await?,
            cat: cat.to_owned(),
        })
    }

    /// collection of vertex
    fn collection_vertex(&self) -> Collection<Vertex> {
        self.client
            .collection::<Vertex>(&format!("{}_vertex", self.cat))
    }

    /// collection of edge
    fn collection_edge(&self) -> Collection<Edge> {
        self.client
            .collection::<Edge>(&format!("{}_edge", self.cat))
    }

    pub async fn create_vertex<'a>(&self, dto: VertexDto<'a>) -> Pyo3MongoResult<Vertex> {
        let vertex = dto.to_vertex(&self.cat);
        let res = self.collection_vertex().insert_one(vertex, None).await?;

        let id = res.inserted_id.as_object_id().unwrap();
        let res = self
            .client
            .collection::<Vertex>(&self.cat)
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }

    pub async fn get_vertex(&self, id: ObjectId) -> Pyo3MongoResult<Vertex> {
        let res = self
            .collection_vertex()
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }

    pub async fn get_vertexes(&self, ids: Vec<ObjectId>) -> Pyo3MongoResult<Vec<Vertex>> {
        let res = self
            .collection_vertex()
            .find(Some(bson::doc! {"_id": bson::doc! {"$in": ids}}), None)
            .await?
            .map(|v| v.unwrap())
            .collect::<Vec<Vertex>>()
            .await;

        Ok(res)
    }

    pub async fn get_all_vertexes(&self) -> Pyo3MongoResult<Vec<Vertex>> {
        let mut cursor = self.collection_vertex().find(None, None).await?;

        let mut res = Vec::new();
        while let Some(doc) = cursor.next().await {
            res.push(doc?);
        }

        Ok(res)
    }

    pub async fn update_vertex<'a>(
        &self,
        id: ObjectId,
        dto: VertexDto<'a>,
    ) -> Pyo3MongoResult<Vertex> {
        let id = bson::doc! {"_id": id};
        let update = bson::doc! {
            "$set": Document::from(&dto.to_vertex(&self.cat))
        };
        let res = self
            .collection_vertex()
            .find_one_and_update(id, update, None)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        Ok(res?)
    }

    /// look up source & target vertexes whether existed
    async fn check_edge_legitimacy<'a>(&self, dto: &EdgeDto<'a>) -> Pyo3MongoResult<()> {
        // make sure source vertex existed
        self.get_vertex(dto.source).await?;

        // make sure target vertexes existed
        let targets = self.get_vertexes(dto.targets.clone()).await?;
        if targets.len() != dto.targets.len() {
            return Err(Pyo3MongoError::Common("target vertex not in this graph"));
        }

        Ok(())
    }

    pub async fn create_edge<'a>(&self, dto: EdgeDto<'a>) -> Pyo3MongoResult<Edge> {
        if let Err(e) = self.check_edge_legitimacy(&dto).await {
            return Err(e);
        }

        let edge = dto.to_edge(&self.cat);
        let res = self.collection_edge().insert_one(edge, None).await?;

        let id = res.inserted_id.as_object_id().unwrap();
        let res = self
            .collection_edge()
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("edge not found"));

        Ok(res?)
    }

    pub async fn get_edge(&self, id: ObjectId) -> Pyo3MongoResult<Edge> {
        let res = self
            .collection_edge()
            .find_one(Some(bson::doc! {"_id": id}), None)
            .await?
            .ok_or(Pyo3MongoError::Common("edge not fount"));

        Ok(res?)
    }

    pub async fn get_edges(&self, ids: Vec<ObjectId>) -> Pyo3MongoResult<Vec<Edge>> {
        let res = self
            .collection_edge()
            .find(Some(bson::doc! {"_id": bson::doc! {"$in": ids}}), None)
            .await?
            .map(|v| v.unwrap())
            .collect::<Vec<Edge>>()
            .await;

        Ok(res)
    }

    pub async fn get_all_edges(&self) -> Pyo3MongoResult<Vec<Edge>> {
        let mut cursor = self.collection_edge().find(None, None).await?;

        let mut res = Vec::new();
        while let Some(doc) = cursor.next().await {
            res.push(doc?);
        }

        Ok(res)
    }

    pub async fn update_edge<'a>(&self, id: ObjectId, dto: EdgeDto<'a>) -> Pyo3MongoResult<Edge> {
        if let Err(e) = self.check_edge_legitimacy(&dto).await {
            return Err(e);
        }

        let id = bson::doc! {"_id": id};
        let update = bson::doc! {
            "$set": Document::from(&dto.to_edge(&self.cat))
        };
        let res = self
            .collection_edge()
            .find_one_and_update(id, update, None)
            .await?
            .ok_or(Pyo3MongoError::Common("edge not found"));

        Ok(res?)
    }

    pub async fn delete_edge(&self, id: ObjectId) -> Pyo3MongoResult<()> {
        let res = self
            .collection_edge()
            .delete_one(bson::doc! {"_id": id}, None)
            .await?;

        if res.deleted_count == 0 {
            return Err(Pyo3MongoError::Common("edge not found"));
        }

        Ok(())
    }

    pub async fn delete_edges(&self, ids: Vec<ObjectId>) -> Pyo3MongoResult<()> {
        let res = self
            .collection_edge()
            .delete_many(bson::doc! {"_id": { "$in": ids }}, None)
            .await?;

        if res.deleted_count == 0 {
            return Err(Pyo3MongoError::Common("edge not found"));
        }

        Ok(())
    }

    fn edges_by_vertex_pipeline(&self, find_dto: FindEdgeByVertexDto) -> Vec<Document> {
        // match object id in vertex collection
        let match_id = |id: ObjectId| bson::doc! {"$match": bson::doc! {"_id": id}};
        // from edge collection
        let from = format!("{}_edge", self.cat);
        // lookup related edges, source/target/both
        let lookup = |field: &str| {
            bson::doc! {"$lookup": bson::doc! {
                "from": &from,
                "localField": "_id",
                "foreignField": field,
                "as": "edges"
            }}
        };
        // turn aggregations into a vector of edges document
        let unwind = bson::doc! {"$unwind": "$edges"};

        match find_dto {
            FindEdgeByVertexDto::Source(id) => {
                vec![match_id(id), lookup("source"), unwind]
            }
            FindEdgeByVertexDto::Target(id) => {
                vec![match_id(id), lookup("target"), unwind]
            }
            FindEdgeByVertexDto::Bidirectional(id) => {
                vec![match_id(id), lookup("source"), lookup("target"), unwind]
            }
        }
    }

    /// get all related edges
    pub async fn get_edges_by_vertex(
        &self,
        find_dto: FindEdgeByVertexDto,
    ) -> Pyo3MongoResult<Vec<Edge>> {
        let pipeline = self.edges_by_vertex_pipeline(find_dto);

        // cursor, futures iterator
        let mut cursor = self.collection_vertex().aggregate(pipeline, None).await?;

        let mut res = Vec::new();
        while let Some(doc) = cursor.next().await {
            let edge: Edge = bson::from_document(doc?)?;
            res.push(edge);
        }

        Ok(res)
    }

    /// delete vertex
    /// atomically delete all related edges and then delete vertex
    pub async fn delete_vertex(&self, id: ObjectId) -> Pyo3MongoResult<Vertex> {
        // get all related edges
        let mut pipeline = self.edges_by_vertex_pipeline(FindEdgeByVertexDto::Bidirectional(id));
        // project only _id
        pipeline.push(bson::doc! {"$project": bson::doc! {"_id": 1}});
        // look up all related edges' _id
        let mut cursor = self.collection_vertex().aggregate(pipeline, None).await?;
        let mut ids = Vec::new();
        while let Some(doc) = cursor.next().await {
            let id: ObjectId = bson::from_document(doc?)?;
            ids.push(id);
        }

        // start a transaction
        let mut session = self.client.client().start_session(None).await?;
        let options = TransactionOptions::builder()
            .read_concern(ReadConcern::local())
            .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
            .build();
        session.start_transaction(options).await?;

        // delete all related edges
        let delete_edges = self
            .collection_edge()
            .delete_many_with_session(bson::doc! {"_id": { "$in": ids }}, None, &mut session)
            .await;

        if let Err(e) = delete_edges {
            session.abort_transaction().await?;
            return Err(Pyo3MongoError::Mongo(e));
        }

        // delete vertex
        let delete_vertex = self
            .collection_vertex()
            .find_one_and_delete_with_session(bson::doc! {"_id": id}, None, &mut session)
            .await?
            .ok_or(Pyo3MongoError::Common("vertex not found"));

        if let Err(e) = delete_vertex {
            session.abort_transaction().await?;
            return Err(e);
        }

        session.commit_transaction().await?;

        Ok(delete_vertex?)
    }

    pub async fn get_graph_from_vertex_by_label(
        &self,
        vertex_id: ObjectId,
        label: Option<&str>,
        depth: Option<i32>,
    ) -> Pyo3MongoResult<Vec<Edge>> {
        let pipeline = vec![
            bson::doc! {"$match": bson::doc! {"_id": vertex_id}},
            bson::doc! {"$graphLookup": bson::doc! {
                "from": format!("{}_edge", self.cat),
                "startWith": "$_id",
                "connectFromField": "source",
                "connectToField": "target",
                "maxDepth": match depth {
                    Some(d) => d,
                    None => 1,
                },
                "as": "edges",
                "restrictSearchWithMatch": bson::doc! {"label": label}
            }},
            bson::doc! {"$unwind": "$edges"},
        ];

        let mut cursor = self.collection_vertex().aggregate(pipeline, None).await?;

        let mut res = Vec::new();
        while let Some(doc) = cursor.next().await {
            let edge: Edge = bson::from_document(doc?)?;
            res.push(edge);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test_service {
    use super::super::model::{EdgeDto, VertexDto};
    use super::*;

    const URI: &str = "mongodb://root:secret@localhost:27017";
    const CAT: &str = "dev";

    #[tokio::test]
    async fn test_vertex_crud() {
        let gs = GraphService::new(URI, CAT).await.unwrap();

        let create = gs.create_vertex(VertexDto::new("node-1")).await.unwrap();
        println!("{:?}", create);

        let id = create.id.unwrap();
        let get = gs.get_vertex(id).await.unwrap();
        println!("{:?}", get);

        assert_eq!(create, get);

        let update = gs
            .update_vertex(id, VertexDto::new("node-2"))
            .await
            .unwrap();
        let get = gs.get_vertex(id).await.unwrap();

        assert_eq!(update, get);

        let delete = gs.delete_vertex(id).await;
        assert!(delete.is_ok());
    }

    #[tokio::test]
    async fn test_edge_crud() {
        let gs = GraphService::new(URI, CAT).await.unwrap();

        let node1 = gs.create_vertex(VertexDto::new("node-1")).await.unwrap();
        let node2 = gs.create_vertex(VertexDto::new("node-2")).await.unwrap();
        let node3 = gs.create_vertex(VertexDto::new("node-3")).await.unwrap();

        // node1 -> node2
        let _edge1 = gs
            .create_edge(EdgeDto::new(
                node1.id.unwrap(),
                vec![node2.id.unwrap()],
                Some(2.0),
                Some("edge-1"),
            ))
            .await
            .unwrap();

        // node2 -> node3
        let _edge2 = gs
            .create_edge(EdgeDto::new(
                node2.id.unwrap(),
                vec![node3.id.unwrap()],
                Some(3.0),
                Some("edge-2"),
            ))
            .await
            .unwrap();

        // TODO:
    }
}
