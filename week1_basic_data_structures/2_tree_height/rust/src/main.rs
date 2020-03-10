use std::ops::{Index, IndexMut};

type NodeId = usize;

#[derive(Clone, Debug, PartialEq)]
struct Node {
    children: Vec<NodeId>
}

impl Node {
    fn new() -> Self {
        Node { children: Vec::new() }
    }
}

#[derive(Debug, PartialEq)]
struct Tree {
    root: Option<NodeId>,
    nodes: Vec<Node>,
}

impl Index<usize> for Tree {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<usize> for Tree {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Tree {
    fn new(size: usize) -> Self {
        Tree {
            root: None,
            nodes: vec![Node::new(); size],
        }
    }

    fn parse(&mut self, str: &str) -> &mut Tree {
        str
            .split_whitespace()
            .enumerate()
            .fold(
                self,
                |tree, (i, x)| {
                    match x.parse::<usize>() {
                        Ok(parent_id) => tree.nodes[parent_id].children.push(i),
                        Err(_) => tree.root = Some(i)
                    }

                    tree
                })
    }

    fn height(&self) -> u32 {
        fn height_rec(tree: &Tree, node_id: NodeId) -> u32 {
            tree[node_id].children
                .iter()
                .map(|&x| 1 + height_rec(tree, x))
                .max()
                .unwrap_or(1)
        }

        height_rec(self, self.root.unwrap())
    }
}

fn main() {
    let n: usize = read_line().trim().parse().unwrap();
    let input = read_line();

    println!("{}", Tree::new(n).parse(input.trim()).height())
}

#[test]
fn test_tree_parse() {
    let mut e = Tree::new(5);
    e.root = Some(1);
    e[1].children.push(3);
    e[1].children.push(4);
    e[4].children.push(0);
    e[4].children.push(2);

    assert_eq!(*Tree::new(5).parse("4 -1 4 1 1"), e);
}

#[test]
fn test_tree_height() {
    assert_eq!(Tree::new(5).parse("4 -1 4 1 1").height(), 3);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

