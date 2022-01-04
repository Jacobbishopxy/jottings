use std::str::FromStr;

use bson::oid::ObjectId;
use clap::Parser;
use pyo3mongo::{EdgeDto, GraphService, Pyo3MongoResult};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    uri: String,

    #[clap(short, long, default_value = "graph")]
    database: String,

    #[clap(short, long, default_value = "dev")]
    category: String,

    #[clap(long)]
    source: String,

    #[clap(long)]
    target: String,

    #[clap(long)]
    weight: Option<f64>,

    #[clap(long)]
    label: Option<String>,
}

#[tokio::main]
async fn main() -> Pyo3MongoResult<()> {
    let args = Args::parse();

    let gs = GraphService::new(&args.uri, &args.database, &args.category).await?;

    let source = ObjectId::from_str(&args.source)?;
    let target = ObjectId::from_str(&args.target)?;
    let weight = args.weight;
    let label = args.label;
    let dto = EdgeDto::new(source, target, weight, label.as_deref());

    let edge = gs.create_edge(dto).await?;

    println!("{:?}", edge);

    Ok(())
}
