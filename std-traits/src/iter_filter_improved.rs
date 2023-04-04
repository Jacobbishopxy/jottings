//! file: iter_filter2.rs
//! author: Jacob Xie
//! date: 2023/04/03 22:04:19 Monday
//! brief:

//! file: iter_filter.rs
//! author: Jacob Xie
//! date: 2023/04/03 21:09:07 Monday
//! brief:

use std::borrow::Borrow;

#[derive(Debug)]
struct TaskUnit {
    dag_id: String,
    dag_run_id: String,
    task_id: String,
}

impl TaskUnit {
    #[allow(dead_code)]
    fn new(di: &str, dri: &str, ti: &str) -> Self {
        Self {
            dag_id: di.to_owned(),
            dag_run_id: dri.to_owned(),
            task_id: ti.to_owned(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct ATaskUnitRef<'a> {
    dag_id: &'a str,
    dag_run_id: &'a str,
    task_id: &'a str,
}

impl<'a> ATaskUnitRef<'a> {
    fn compare(&self, di: &str) -> bool {
        self.dag_id == di
    }
}

impl<'a, T> From<&'a T> for ATaskUnitRef<'a>
where
    T: Borrow<TaskUnit> + ?Sized,
{
    fn from(value: &'a T) -> Self {
        let rf = Borrow::<TaskUnit>::borrow(value);

        ATaskUnitRef {
            dag_id: &rf.dag_id,
            dag_run_id: &rf.dag_run_id,
            task_id: &rf.task_id,
        }
    }
}

#[test]
fn test_from() {
    let tu = TaskUnit::new("di", "dri", "ti");

    let tu_ref = &tu;

    let atur = ATaskUnitRef::from(&tu);
    let aturr = ATaskUnitRef::from(tu_ref);
    let aturrr = ATaskUnitRef::from(&tu_ref);

    println!("{:?}", atur);
    println!("{:?}", aturr);
    println!("{:?}", aturrr);
}

#[allow(dead_code)]
fn filter<'s, I>(data: I, di: &'s str) -> Vec<I::Item>
where
    I: IntoIterator,
    for<'a> ATaskUnitRef<'a>: From<&'a I::Item>,
{
    data.into_iter()
        .filter_map(|e| {
            let r = ATaskUnitRef::from(&e);

            r.compare(di).then_some(e)
        })
        .collect::<Vec<_>>()
}

#[test]
fn test_foo() {
    let foos = vec![
        TaskUnit::new("di1", "dri", "ti"),
        TaskUnit::new("di2", "dri", "ti"),
    ];

    let f_foos = filter(foos, "di1");
    println!("{:?}", f_foos);

    let foos = vec![
        TaskUnit::new("di1", "dri", "ti"),
        TaskUnit::new("di2", "dri", "ti"),
    ];

    let f_foos = filter(&foos, "di1");
    println!("{:?}", f_foos);
}
