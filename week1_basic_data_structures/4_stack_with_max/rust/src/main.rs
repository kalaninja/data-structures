use std::cmp::max;

struct Stack<T> {
    buf: Vec<(T, T)>,
}

impl<T: Ord + Copy> Stack<T> {
    fn new() -> Self {
        Stack { buf: Vec::new() }
    }

    fn push(&mut self, value: T) {
        let cur_max = self.buf
            .last()
            .map_or(value, |(_, cur_max)| max(value, *cur_max));
        self.buf.push((value, cur_max));
    }

    fn pop(&mut self) -> Option<T> {
        self.buf.pop().map(|(val, _)| val)
    }

    fn max(&self) -> Option<T> {
        self.buf.last().map(|(_, max)| *max)
    }
}

fn main() {
    let n = read_line().trim().parse().unwrap();

    let mut stack: Stack<u32> = Stack::new();
    for _ in 0..n {
        let command = read_line();
        match &command[..2] {
            "pu" => stack.push(command.trim()[5..].parse().unwrap()),
            "po" => { stack.pop(); }
            "ma" => println!("{}", stack.max().unwrap()),
            _ => unreachable!()
        }
    }
}

#[test]
fn test_case_1() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.max(), Some(2));
    stack.pop();
    assert_eq!(stack.max(), Some(1));
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}