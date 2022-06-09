//! Async Processor

use std::marker::PhantomData;

use std::future::Future;

pub struct AProcessor<F, Fut, I, O>
where
    for<'a> F: Fn(I) -> Fut,
    Fut: Future<Output = O>,
{
    func: F,
    called: usize,
    f: PhantomData<Fut>,
    i: PhantomData<I>,
    o: PhantomData<O>,
}

impl<F, Fut, I, O> AProcessor<F, Fut, I, O>
where
    for<'a> F: Fn(I) -> Fut,
    Fut: Future<Output = O>,
{
    pub fn new(func: F) -> Self {
        AProcessor {
            func,
            called: 0,
            f: PhantomData,
            i: PhantomData,
            o: PhantomData,
        }
    }

    pub async fn process(&mut self, i: I) -> O {
        self.called += 1;
        (self.func)(i).await
    }
}

async fn fake_process(i: i32) -> String {
    format!(">>> {:?} <<<", i + 1)
}

#[tokio::test]
async fn async_processor_success() {
    let mut processor = AProcessor::new(fake_process);

    let res = processor.process(5).await;

    println!("res: {:?}", res);
    println!("called: {:?}", processor.called);
}
