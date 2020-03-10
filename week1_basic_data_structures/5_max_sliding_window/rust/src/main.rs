use std::io::stdin;
use std::collections::VecDeque;

fn main() {
    let _n = read_line();
    let a = read_line().trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let m = read_line().trim().parse().unwrap();

    let result = sliding_max(a, m)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", result);
}

fn sliding_max(vec: Vec<u32>, size: usize) -> Vec<u32> {
    let mut max = Vec::with_capacity(vec.len() - size + 1);
    let mut window = VecDeque::with_capacity(size);

    // init window
    for i in 0..size {
        while window.back().map_or(false, |&x| vec[i] >= vec[x]) {
            window.pop_back();
        }

        window.push_back(i);
    }

    max.push(vec[*window.front().unwrap()]);

    // slide
    for i in size..vec.len() {
        if window.front().map_or(false, |&x| x <= i - size) {
            window.pop_front();
        }

        let new_val = vec[i];
        while window.back().map_or(false, |&x| vec[x] <= new_val) {
            window.pop_back();
        }

        window.push_back(i);
        max.push(vec[*window.front().unwrap()]);
    }

    max
}

#[test]
fn test_case_1() {
    let vec = vec![2, 7, 3, 1, 5, 2, 6, 2];
    assert_eq!(sliding_max(vec, 4), vec![7, 7, 5, 6, 6])
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}