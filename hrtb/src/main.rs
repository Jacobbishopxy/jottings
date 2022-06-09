//! Higher-Rank Trait Bounds (HTRBs)
//!
//! use case

struct Processor<F, T>
where
    for<'a> F: Fn(&'a mut T),
{
    data: T,
    func: F,
}

impl<F, T> Processor<F, T>
where
    for<'a> F: Fn(&'a mut T),
{
    pub fn new(data: T, func: F) -> Self {
        Processor { data, func }
    }

    pub fn process(&mut self) {
        (self.func)(&mut self.data);
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug)]
enum Data {
    A(u32),
    B(i32),
}

fn add_one(data: &mut Data) {
    match data {
        Data::A(x) => *x += 1,
        Data::B(x) => *x += 1,
    }
}

fn minus_one(data: &mut Data) {
    match data {
        Data::A(x) => *x -= 1,
        Data::B(x) => *x -= 1,
    }
}

fn main() {
    let mut x = Processor::new(Data::A(1), add_one);
    x.process();

    println!("{:?}", x.data());

    let mut y = Processor::new(Data::B(1), minus_one);
    y.process();

    println!("{:?}", y.data());
}
