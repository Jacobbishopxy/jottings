//! file: iter_filter.rs
//! author: Jacob Xie
//! date: 2023/04/03 21:09:07 Monday
//! brief:

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

#[derive(Debug)]
struct TaskInstance {
    dag_id: String,
    dag_run_id: String,
    task_id: String,
    _a: usize,
    _b: usize,
    _c: usize,
}

impl TaskInstance {
    #[allow(dead_code)]
    fn new(di: &str, dri: &str, ti: &str) -> Self {
        Self {
            dag_id: di.to_owned(),
            dag_run_id: dri.to_owned(),
            task_id: ti.to_owned(),
            _a: 0,
            _b: 0,
            _c: 0,
        }
    }
}

#[allow(dead_code)]
struct ATaskUnitRef<'a> {
    dag_id: &'a str,
    dag_run_id: &'a str,
    task_id: &'a str,
}

impl<'a> ATaskUnitRef<'a> {
    #[allow(dead_code)]
    fn compare(&self, di: &str) -> bool {
        self.dag_id == di
    }
}

impl<'a> From<&'a TaskUnit> for ATaskUnitRef<'a> {
    fn from(value: &'a TaskUnit) -> Self {
        Self {
            dag_id: &value.dag_id,
            dag_run_id: &value.dag_run_id,
            task_id: &value.task_id,
        }
    }
}

impl<'a> From<&'a &TaskUnit> for ATaskUnitRef<'a> {
    fn from(value: &'a &TaskUnit) -> Self {
        Self {
            dag_id: &value.dag_id,
            dag_run_id: &value.dag_run_id,
            task_id: &value.task_id,
        }
    }
}

impl<'a> From<&'a TaskInstance> for ATaskUnitRef<'a> {
    fn from(value: &'a TaskInstance) -> Self {
        Self {
            dag_id: &value.dag_id,
            dag_run_id: &value.dag_run_id,
            task_id: &value.task_id,
        }
    }
}

impl<'a> From<&'a &TaskInstance> for ATaskUnitRef<'a> {
    fn from(value: &'a &TaskInstance) -> Self {
        Self {
            dag_id: &value.dag_id,
            dag_run_id: &value.dag_run_id,
            task_id: &value.task_id,
        }
    }
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

    let foos = vec![
        TaskInstance::new("di1", "dri", "ti"),
        TaskInstance::new("di2", "dri", "ti"),
    ];

    let f_foos = filter(foos, "di1");
    println!("{:?}", f_foos);

    let foos = vec![
        TaskInstance::new("di1", "dri", "ti"),
        TaskInstance::new("di2", "dri", "ti"),
    ];

    let f_foos = filter(foos.as_slice(), "di1");
    println!("{:?}", f_foos);
}
