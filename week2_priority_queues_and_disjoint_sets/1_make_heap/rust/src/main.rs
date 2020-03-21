use std::io::stdin;

fn main() {
    let _n = read_line();
    let mut a = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let result = build_heap(&mut a);

    println!("{}", result.len());
    result.iter().for_each(|(a, b)| println!("{} {}", a, b))
}

fn build_heap(a: &mut Vec<u32>) -> Vec<(usize, usize)> {
    let mut swaps = Vec::new();

    for i in (0..a.len() / 2).rev() {
        let mut cur_i = i;
        loop {
            let left = 2 * cur_i + 1;
            if left >= a.len() { break; }

            let right = left + 1;
            let min = if right < a.len() && a[right] < a[left] { right } else { left };
            if a[min] < a[cur_i] {
                a.swap(cur_i, min);
                swaps.push((cur_i, min));
                cur_i = min;
            } else {
                break;
            }
        }
    }

    swaps
}

#[test]
fn test_cases() {
    let mut tests = vec![
        (vec![5, 4, 3, 2, 1], vec![(1_usize, 4_usize), (0, 1), (1, 3)]),
        (vec![1, 2, 3, 4, 5], vec![])
    ];

    tests.iter_mut()
        .for_each(|(d, e)| assert_eq!(build_heap(d), *e))
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}