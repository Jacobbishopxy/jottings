//! Async Processor

use std::marker::PhantomData;

use std::future::Future;

#[allow(dead_code)]
pub struct AProcessor<F, Fut, I, O>
where
    F: Fn(I) -> Fut,
    Fut: Future<Output = O>,
{
    func: F,
    called: usize,
    f: PhantomData<Fut>,
    i: PhantomData<I>,
}

impl<F, Fut, I, O> AProcessor<F, Fut, I, O>
where
    F: Fn(I) -> Fut,
    Fut: Future<Output = O>,
{
    #[allow(dead_code)]
    pub fn new(func: F) -> Self {
        AProcessor {
            func,
            called: 0,
            f: PhantomData,
            i: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub async fn process(&mut self, i: I) -> O {
        self.called += 1;
        (self.func)(i).await
    }
}

#[allow(dead_code)]
async fn fake_process(i: i32) -> String {
    format!(">>> {:?} <<<", i + 1)
}

#[tokio::test]
async fn async_processor_success1() {
    let fp = |i| async move { fake_process(i).await };

    let mut processor = AProcessor::new(fp);

    let res = processor.process(5).await;

    println!("res: {:?}", res);
    println!("called: {:?}", processor.called);
}

#[allow(dead_code)]
pub struct AProcessor2 {
    called: usize,
}

impl AProcessor2 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AProcessor2 { called: 0 }
    }

    #[allow(dead_code)]
    pub async fn process<'a, F, Fut, O>(&'a mut self, f: F) -> O
    where
        F: Fn(&'a mut Self) -> Fut,
        Fut: Future<Output = O>,
    {
        self.called += 1;
        f(self).await
    }

    #[allow(dead_code)]
    pub async fn add(&mut self, a: usize) -> usize {
        self.called += a;
        self.called
    }
}

#[tokio::test]
async fn async_processor_success2() {
    let mut p2 = AProcessor2::new();

    let num = 2;

    let res = p2.process(|s| async { s.add(num).await }).await;

    println!("{:?}", res);
}
