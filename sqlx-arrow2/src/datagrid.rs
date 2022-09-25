//! Datagrid

use std::sync::Arc;

use arrow2::array::*;
use arrow2::datatypes::{DataType, Field, Schema};

pub struct Datagrid(Vec<Arc<dyn Array>>);

#[test]
fn name() {
    let a = Int32Vec::from([Some(1), None, Some(3)]).as_arc();
    let b = Int32Vec::from([Some(2), None, Some(6)]).as_arc();

    let fo = Datagrid(vec![a, b]);

    println!("{:?}", fo.0[1].data_type());
}
