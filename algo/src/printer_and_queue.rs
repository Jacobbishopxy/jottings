//! Printer and Queue
//!
//! https://blog.csdn.net/zzcwing/article/details/105984562?spm=1001.2014.3001.5502

use rand::Rng;

#[derive(Default)]
pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn size_of(&self) -> usize {
        self.data.len()
    }
}

#[allow(dead_code)]
struct Task {
    timestamp: i64,
    pages: i64,
}

#[allow(dead_code)]
impl Task {
    fn new(timestamp: i64) -> Self {
        Task {
            timestamp,
            pages: rand::thread_rng().gen_range(1..21),
        }
    }

    fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    fn get_pages(&self) -> i64 {
        self.pages
    }

    fn wait_time(&self, current_time: i64) -> i64 {
        current_time - self.timestamp
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct Printer {
    page_rate: i64,
    current_task: Option<Task>,
    time_remaining: i64,
}

#[allow(dead_code)]
impl Printer {
    fn new(page_rate: i64) -> Self {
        Printer {
            page_rate,
            current_task: None,
            time_remaining: 0,
        }
    }

    fn tick(&mut self) {
        if self.current_task.is_some() {
            self.time_remaining -= 1;
            if self.time_remaining <= 0 {
                self.current_task = None;
            }
        }
    }

    fn busy(&self) -> bool {
        self.current_task.is_some()
    }

    fn start_next(&mut self, task: Task) {
        self.time_remaining = (task.get_pages() * 60 / self.page_rate) as i64;
        self.current_task = Some(task);
    }
}

#[allow(dead_code)]
fn new_print_task() -> bool {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..181);
    num == 180
}

#[allow(dead_code)]
fn simulation(num_seconds: i64, pages_per_minute: i64) -> (f64, usize) {
    let mut printer = Printer::new(pages_per_minute);
    let mut queue = Queue::new();
    let mut waiting_times: Vec<i64> = Vec::new();

    for current_second in 0..num_seconds {
        if new_print_task() {
            let task = Task::new(current_second);
            queue.enqueue(task);
        }

        if !printer.busy() && !queue.is_empty() {
            let next_task = queue.dequeue().expect("not empty");
            waiting_times.push(next_task.wait_time(current_second));
            printer.start_next(next_task);
        }

        printer.tick();
    }

    let sum = waiting_times.iter().sum::<i64>();
    let average_wait = (sum / waiting_times.len() as i64) as f64;
    (average_wait, queue.size_of())
}

#[test]
fn test_simulation() {
    for _ in 0..10 {
        let (avg, size) = simulation(3600, 5);
        println!("average time: {:?}, {:?} tasks remaining", avg, size);
    }
}
