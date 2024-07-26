use std::borrow::Cow;

use polars::prelude::*;

#[test]
fn with_column_success() {
    let mut df = df!(
        "ticker" => ["000001.SZ", "600001.SH", "000300.SZ"],
        "date" => ["2019-01-01", "2019-01-02", "2019-01-03"],
        "close" => [2u32, 5, 4],
    )
    .unwrap();

    let mut new_ticker = df
        .column("ticker")
        .unwrap()
        .str()
        .unwrap()
        .apply(|t| t.map(|s| Cow::from(&s[..6])));
    new_ticker.rename("new_ticker");

    df.with_column(new_ticker).unwrap();

    println!("{:?}", df);
}
