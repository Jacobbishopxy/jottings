//! file: join_types.rs
//! author: Jacob Xie
//! date: 2023/10/25 10:59:53 Wednesday
//! brief:

#[allow(unused_imports)]
use polars::prelude::*;

#[test]
fn join_test_case() {
    let df1 = df!("Fruit" => &["Apple", "Banana", "Pear"], "Phosphorus (mg/100g)" => &[11,22,12])
        .unwrap();
    let df2 = df!("Name" => &["Apple", "Banana", "Pear"], "Potassium (mg/100g)" => &[107,358,115])
        .unwrap();

    let df3 = df1
        .join(&df2, ["Fruit"], ["Name"], JoinArgs::new(JoinType::Left))
        .unwrap();

    println!("{:?}", df3);
}

#[test]
fn join_test_case2() {
    let df1 =
        df!("Fruit" => &["Pear", "Apple", "Pear"], "Phosphorus (mg/100g)" => &[11,22,12]).unwrap();

    let df2 = df!("Name" => &["Apple", "Pear", "Pear", "Pear"], "Potassium (mg/100g)" => &[107,358,115, 116])
        .unwrap();

    let df_left = df1
        .join(&df2, ["Fruit"], ["Name"], JoinArgs::new(JoinType::Left))
        .unwrap();
    println!("left:\n{:?}", df_left);

    let df_inner = df1
        .join(&df2, ["Fruit"], ["Name"], JoinArgs::new(JoinType::Inner))
        .unwrap();
    println!("inner:\n{:?}", df_inner);

    let df_outer = df1
        .join(&df2, ["Fruit"], ["Name"], JoinArgs::new(JoinType::Outer))
        .unwrap();
    println!("outer:\n{:?}", df_outer);
}
