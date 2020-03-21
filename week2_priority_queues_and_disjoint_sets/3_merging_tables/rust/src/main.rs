use std::io::stdin;

struct DisjointSet {
    parents: Vec<usize>,
    table_sizes: Vec<u64>,
    max_table_size: u64,
}

impl DisjointSet {
    fn new(table_sizes: Vec<u32>) -> Self {
        DisjointSet {
            parents: (0..table_sizes.len()).collect(),
            table_sizes: table_sizes.iter().map(|&x| x as u64).collect(),
            max_table_size: table_sizes.iter().max().map(|&x| x as u64).unwrap(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parents[i] != i {
            let parent_i = self.parents[i];
            let root_i = self.find(parent_i);
            self.parents[i] = root_i;
            root_i
        } else {
            i
        }
    }

    fn merge(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            self.parents[root_j] = root_i;
            self.table_sizes[root_i] += self.table_sizes[root_j];
            if self.table_sizes[root_i] > self.max_table_size {
                self.max_table_size = self.table_sizes[root_i];
            }
        }
    }
}

fn main() {
    let m = read_line().trim()
        .split_whitespace()
        .nth(1).unwrap()
        .parse().unwrap();

    let table_sizes = read_line().trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let merges = (0..m).map(|_| {
        let vec = read_line().trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        (vec[0], vec[1])
    }).collect();

    solve(table_sizes, merges).iter()
        .for_each(|x| println!("{}", x));
}

fn solve(table_sizes: Vec<u32>, merges: Vec<(usize, usize)>) -> Vec<u64> {
    let mut result = Vec::with_capacity(merges.len());

    let mut disjoint_set = DisjointSet::new(table_sizes);
    for (table_a, table_b) in merges {
        disjoint_set.merge(table_a, table_b);
        result.push(disjoint_set.max_table_size)
    }

    result
}

#[test]
fn test_case_1() {
    let merges = vec![(2, 4), (1, 3), (0, 3), (4, 3), (4, 2)];
    assert_eq!(solve(vec![1, 1, 1, 1, 1], merges), vec![2, 2, 3, 5, 5])
}

#[test]
fn test_case_2() {
    let merges = vec![(5, 5), (5, 4), (4, 3), (3, 2)];
    assert_eq!(solve(vec![10, 0, 5, 0, 3, 3], merges), vec![10, 10, 10, 11])
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}