use clap::Parser;
use p3m::{GraphService, Pyo3MongoResult};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    uri: String,

    #[clap(short, long, default_value = "graph")]
    database: String,

    #[clap(short, long, default_value = "dev")]
    category: String,
}

#[tokio::main]
async fn main() -> Pyo3MongoResult<()> {
    let args = Args::parse();

    let gs = GraphService::new(&args.uri, &args.database, &args.category).await?;

    Ok(gs.truncate_all().await?)
}
