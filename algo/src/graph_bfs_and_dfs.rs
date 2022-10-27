//! Graph BFS and DFS
//!
//! https://blog.csdn.net/zzcwing/article/details/109233598

use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[allow(dead_code)]
type Link = Option<Rc<RefCell<Node>>>;

#[allow(dead_code)]
struct Node {
    x: usize,
    next: Link,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("x", &self.x)
            .field("next", &self.next)
            .finish()
    }
}

#[allow(dead_code)]
impl Node {
    fn new(x: usize) -> Self {
        Node { x, next: None }
    }
}

#[allow(dead_code)]
struct GraphLink {
    first: Link,
    last: Link,
}

impl Debug for GraphLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = self
            .first
            .as_ref()
            .map(|v| v.borrow().x.to_string())
            .unwrap_or_else(|| "null".to_owned());
        let last = self
            .last
            .as_ref()
            .map(|v| v.borrow().x.to_string())
            .unwrap_or_else(|| "null".to_owned());
        f.debug_struct("GraphLink")
            .field("first", &first)
            .field("last", &last)
            .finish()
    }
}

#[allow(dead_code)]
impl GraphLink {
    fn new() -> Self {
        GraphLink {
            first: None,
            last: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    fn print_node(&self) {
        let mut current = self.first.clone();
        while let Some(value) = current {
            print!("[{:?}]", value.borrow().x);
            current = value.borrow().next.clone();
        }
    }

    fn get_first(&self) -> Link {
        self.first.clone()
    }

    fn insert(&mut self, x: usize) {
        let node = Rc::new(RefCell::new(Node::new(x)));
        if self.is_empty() {
            self.first = Some(node.clone());
            self.last = Some(node);
        } else {
            self.last.as_mut().unwrap().borrow_mut().next = Some(node.clone());
            self.last = Some(node);
        }
    }
}

#[allow(dead_code)]
fn bfs(mut graphic: Vec<(GraphLink, usize)>) {
    let mut node_vec = Vec::new();
    graphic[1].1 = 1;

    let mut current = graphic[1].0.get_first();
    print!("{} -> ", 1);
    while let Some(nv) = current {
        node_vec.push(nv.borrow().x);
        current = nv.borrow().next.clone();
    }

    while !node_vec.is_empty() {
        let x = node_vec.remove(0);
        if graphic[x].1 == 0 {
            graphic[x].1 = 1;
            print!("{} -> ", x);
            let mut current = graphic[x].0.get_first().clone();
            while let Some(nv) = current {
                node_vec.push(nv.borrow().x);
                current = nv.borrow().next.clone();
            }
        }
    }

    println!();
}

#[allow(dead_code)]
fn dfs(mut graphic: Vec<(GraphLink, usize)>) {
    let mut node_vec = Vec::new();
    let mut temp_vec = Vec::new();

    graphic[1].1 = 1;
    let mut current = graphic[1].0.get_first();
    print!("{} -> ", 1);
    while let Some(nv) = current {
        node_vec.insert(0, nv.borrow().x);
        current = nv.borrow().next.clone();
    }

    while !node_vec.is_empty() {
        let x = node_vec.pop().unwrap();
        if graphic[x].1 == 0 {
            graphic[x].1 = 1;
            print!("{} -> ", x);
            let mut current = graphic[x].0.get_first().clone();
            while let Some(nv) = current {
                temp_vec.push(nv.borrow().x);
                current = nv.borrow().next.clone();
            }
            while !temp_vec.is_empty() {
                node_vec.push(temp_vec.pop().unwrap());
            }
        }
    }

    println!();
}

#[cfg(test)]
mod test_bfs_and_dfs {

    use super::*;

    // 1 => [2][3]
    // 2 => [1][4][5]
    // 3 => [1][6][7]
    // 4 => [2][5]
    // 5 => [2][4][8]
    // 6 => [3][7][8]
    // 7 => [3][6]
    // 8 => [5][6]
    fn create_graphic_arr() -> Vec<(GraphLink, usize)> {
        let data = [
            [1, 2],
            [2, 1],
            [1, 3],
            [3, 1],
            [2, 4],
            [4, 2],
            [2, 5],
            [5, 2],
            [3, 6],
            [6, 3],
            [3, 7],
            [7, 3],
            [4, 5],
            [5, 4],
            [6, 7],
            [7, 6],
            [5, 8],
            [8, 5],
            [6, 8],
            [8, 6],
        ];

        let mut arr = Vec::new();

        for _ in 0..9 {
            arr.push((GraphLink::new(), 0));
        }
        println!("directed graph: ");

        for (i, item) in arr.iter_mut().enumerate().take(9).skip(1) {
            print!("vertex {i} => ");
            for d in &data {
                if d[0] == i {
                    item.0.insert(d[1]);
                }
            }
            item.0.print_node();
            println!();
        }

        arr
    }

    #[test]
    fn test_create_graphic_arr() {
        let v = create_graphic_arr();

        for i in v {
            println!("{i:?}");
        }
    }

    #[test]
    fn test_bfs() {
        let v = create_graphic_arr();

        bfs(v);
    }

    #[test]
    fn test_dfs() {
        let v = create_graphic_arr();

        dfs(v);
    }
}
