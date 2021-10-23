use connectorx::prelude::*;

#[test]
fn test_connection() {
    let mut destination = ArrowDestination::new();
    let source = SQLiteSource::new("sqlite:///path/to/db", 10).expect("cannot create the source");
    let queries = &[
        "SELECT * FROM db WHERE id < 100",
        "SELECT * FROM db WHERE id >= 100",
    ];
    let dispatcher = Dispatcher::<SQLiteSource, ArrowDestination, SQLiteArrowTransport>::new(
        source,
        &mut destination,
        queries,
    );
    dispatcher.run().expect("run failed");

    let data = destination.arrow();

    println!("{:?}", data);
}
