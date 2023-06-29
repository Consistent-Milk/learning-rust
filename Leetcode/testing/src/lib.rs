use std::collections::VecDeque;

pub struct Solution;
pub struct Node {
    neighbours: Vec<u8>,
}

impl Node {
    fn new() -> Node {
        Node { neighbours: vec![] }
    }
}

impl Solution {
    // Find bomb's neighbours
    pub fn build_graph(nodes: &mut Vec<Node>, bombs: &[Vec<i32>]) {
        for (idx, bomb) in bombs.iter().enumerate() {
            let mut node = Node::new();
            for (idy, other_bomb) in bombs.iter().enumerate() {
                if idx == idy {
                    continue;
                }
                if ((bomb[0] - other_bomb[0]) as i64).pow(2)
                    + ((bomb[1] - other_bomb[1]) as i64).pow(2)
                    <= (bomb[2] as i64).pow(2)
                {
                    node.neighbours.push(idy as u8);
                }
            }
            nodes.insert(idx, node);
        }
    }

    // Run BFS on every bomb
    pub fn bfs(nodes: &Vec<Node>, mut v: u8) -> u8 {
        let mut marked: Vec<bool> = vec![false; nodes.len()];
        let mut queue: VecDeque<u8> = VecDeque::from([v]);
        let mut count: u8 = 0;
        while !queue.is_empty() {
            v = queue.pop_front().unwrap();
            if !marked[v as usize] {
                count += 1;
                marked[v as usize] = true;
                for w in &nodes[v as usize].neighbours {
                    if !marked[*w as usize] {
                        queue.push_back(*w);
                    }
                }
            }
        }
        count
    }

    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut nodes: Vec<Node> = Vec::with_capacity(bombs.len());
        let mut max_detonates: u8 = 0;
        Solution::build_graph(&mut nodes, &bombs);
        for idx in 0..bombs.len() {
            let detonates = Solution::bfs(&nodes, idx as u8);
            if detonates == bombs.len() as u8 {
                return detonates as i32;
            }
            if detonates > max_detonates {
                max_detonates = detonates;
            }
        }
        max_detonates as i32
    }
}
