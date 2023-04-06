//! file: iter_filter_dev.rs
//! author: Jacob Xie
//! date: 2023/04/05 11:32:52 Wednesday
//! brief:

// ================================================================================================
// ADT
// ================================================================================================

use std::{borrow::Borrow, marker::PhantomData};

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

#[derive(Debug)]
pub struct ATaskUnitRef<'a, O: ?Sized> {
    pub dag_id: &'a str,
    pub dag_run_id: &'a str,
    pub task_id: &'a str,
    _o: PhantomData<O>,
}

impl<'a, O> ATaskUnitRef<'a, O> {
    #[allow(dead_code)]
    fn compare(&self, di: &str) -> bool {
        self.dag_id == di
    }
}

// ================================================================================================
// trait & impl
// ================================================================================================

pub trait UniversalTaskRef {
    fn dag_id(&self) -> &str;

    fn dag_run_id(&self) -> &str;

    fn task_id(&self) -> &str;

    // default impl

    fn to_tur(&self) -> ATaskUnitRef<'_, Self> {
        ATaskUnitRef {
            dag_id: self.dag_id(),
            dag_run_id: self.dag_run_id(),
            task_id: self.task_id(),
            _o: PhantomData,
        }
    }
}

impl UniversalTaskRef for TaskUnit {
    fn dag_id(&self) -> &str {
        &self.dag_id
    }

    fn dag_run_id(&self) -> &str {
        &self.dag_run_id
    }

    fn task_id(&self) -> &str {
        &self.task_id
    }
}

impl UniversalTaskRef for TaskInstance {
    fn dag_id(&self) -> &str {
        &self.dag_id
    }

    fn dag_run_id(&self) -> &str {
        &self.dag_run_id
    }

    fn task_id(&self) -> &str {
        &self.task_id
    }
}

#[test]
fn test_trait() {
    let tu = TaskUnit::new("di", "dri", "ti");

    let btu = tu.to_tur();
    println!("{:?}", btu);

    let btur = (&tu).to_tur();
    println!("{:?}", btur);
}

// ================================================================================================
// from_borrow
// ================================================================================================

impl<'a, T, O> From<&'a T> for ATaskUnitRef<'a, O>
where
    T: Borrow<O>,
    O: UniversalTaskRef + ?Sized + 'a,
{
    fn from(value: &'a T) -> Self {
        let rf = Borrow::<O>::borrow(value);
        rf.to_tur()
    }
}

#[test]
fn test_ref_from_generic_type() {
    let tu = TaskUnit::new("di", "dri", "ti");

    let tur = ATaskUnitRef::from((&tu).borrow());

    println!("{:?}", tur);
}

// ================================================================================================
// filter
// ================================================================================================

#[allow(dead_code)]
fn filter<'s, O, I>(data: I, di: &'s str) -> Vec<I::Item>
where
    I: IntoIterator,
    I::Item: Borrow<O>,
    O: UniversalTaskRef,
{
    data.into_iter()
        .filter_map(|e| {
            let rf = Borrow::<O>::borrow(&e);
            let r = ATaskUnitRef::<O>::from(rf);

            r.compare(di).then_some(e)
        })
        .collect::<Vec<_>>()
}

#[test]
fn test_filter() {
    let foos1 = vec![
        TaskUnit::new("di1", "dri", "ti"),
        TaskUnit::new("di2", "dri", "ti"),
    ];

    let f_foos1 = filter(foos1, "di1");
    println!("{:?}", f_foos1);

    let foos2 = vec![
        TaskUnit::new("di1", "dri", "ti"),
        TaskUnit::new("di2", "dri", "ti"),
    ];

    let f_foos2 = filter::<TaskUnit, _>(&foos2, "di1");
    println!("{:?}", f_foos2);

    let foos3 = vec![
        TaskInstance::new("di1", "dri", "ti"),
        TaskInstance::new("di2", "dri", "ti"),
    ];

    let f_foos3 = filter(foos3, "di1");
    println!("{:?}", f_foos3);

    let foos4 = vec![
        TaskInstance::new("di1", "dri", "ti"),
        TaskInstance::new("di2", "dri", "ti"),
    ];

    let f_foos4 = filter::<TaskInstance, _>(&foos4, "di1");
    println!("{:?}", f_foos4);
}
