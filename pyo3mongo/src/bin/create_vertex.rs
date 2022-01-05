use clap::Parser;
use p3m::{GraphService, Pyo3MongoResult, VertexDto};

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
    name: String,
}

#[tokio::main]
async fn main() -> Pyo3MongoResult<()> {
    let args = Args::parse();

    let gs = GraphService::new(&args.uri, &args.database, &args.category).await?;

    let vertex = gs.create_vertex(VertexDto::new(&args.name)).await?;

    println!("{:?}", vertex);

    Ok(())
}
