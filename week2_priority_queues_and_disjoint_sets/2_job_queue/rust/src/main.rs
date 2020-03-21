use std::io::stdin;
use std::fmt::{self, Display, Formatter};

type ThreadId = u32;
type ThreadTime = u64;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Thread {
    time_idle: ThreadTime,
    thread_id: ThreadId,
}

impl Thread {
    fn new(thread_id: ThreadId, time_idle: ThreadTime) -> Self {
        Thread { thread_id, time_idle }
    }
}

impl Display for Thread {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.thread_id, self.time_idle)
    }
}

#[derive(Debug)]
struct ThreadQueue {
    data: Vec<Thread>
}

impl ThreadQueue {
    fn new(n: usize) -> Self {
        ThreadQueue {
            data: (0..n as ThreadId)
                .fold(Vec::with_capacity(n),
                      |mut acc, i| {
                          acc.push(Thread::new(i, 0));
                          acc
                      })
        }
    }

    fn peak(&self) -> &Thread {
        &self.data[0]
    }

    fn exec_task(&mut self, time: u32) {
        self.data[0].time_idle += time as u64;
        self.sift_down(0);
    }

    fn sift_down(&mut self, i: usize) {
        let mut cur_i = i;

        loop {
            let left = 2 * cur_i + 1;
            if left >= self.data.len() { break; }

            let right = 2 * cur_i + 2;
            let min = if right < self.data.len() && self.data[right] < self.data[left] {
                right
            } else {
                left
            };

            if self.data[min] < self.data[cur_i] {
                self.data.swap(cur_i, min);
                cur_i = min;
            } else {
                break;
            }
        }
    }
}

fn main() {
    let n = read_line()
        .split_whitespace()
        .nth(0).unwrap()
        .parse().unwrap();
    let t = read_line().trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    solve(n, t).iter().for_each(|x| println!("{}", x));
}

fn solve(n: usize, tasks: Vec<u32>) -> Vec<Thread> {
    let mut threads = ThreadQueue::new(n);
    let mut result = Vec::with_capacity(tasks.len());

    for time in tasks {
        result.push(*threads.peak());
        threads.exec_task(time);
    }

    result
}

#[test]
fn test_case_1() {
    let e = vec![
        Thread::new(0, 0),
        Thread::new(1, 0),
        Thread::new(0, 1),
        Thread::new(1, 2),
        Thread::new(0, 4),
    ];

    assert_eq!(solve(2, vec![1, 2, 3, 4, 5]), e);
}

#[test]
fn test_case_2() {
    let e = vec![
        Thread::new(0, 0),
        Thread::new(1, 0),
        Thread::new(2, 0),
        Thread::new(3, 0),
        Thread::new(0, 1),
        Thread::new(1, 1),
        Thread::new(2, 1),
        Thread::new(3, 1),
        Thread::new(0, 2),
        Thread::new(1, 2),
        Thread::new(2, 2),
        Thread::new(3, 2),
        Thread::new(0, 3),
        Thread::new(1, 3),
        Thread::new(2, 3),
        Thread::new(3, 3),
        Thread::new(0, 4),
        Thread::new(1, 4),
        Thread::new(2, 4),
        Thread::new(3, 4),
    ];

    assert_eq!(solve(4, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), e);
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}