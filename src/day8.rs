use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

struct Node {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Node {}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Copy for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i64(self.x);
        state.write_i64(self.y);
        state.write_i64(self.z);
    }
}

struct Edge<'a> {
    a: &'a Node,
    b: &'a Node,
    len: u64,
}

impl PartialEq<Self> for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Eq for Edge<'_> {}

pub fn part1(puzzle: &str) -> u64 {
    let mut sum = 1u64;

    let mut nodes: Vec<Node> = vec![];

    for line in puzzle.lines() {
        let mut parts = line.split(',');
        nodes.push(Node {
            x: parts.next().unwrap().parse::<i64>().unwrap(),
            y: parts.next().unwrap().parse::<i64>().unwrap(),
            z: parts.next().unwrap().parse::<i64>().unwrap(),
        })
    }

    let mut edges: Vec<Edge> = vec![];
    // let (tx, rx) = mpsc::channel();

    let mut map = HashMap::new();

    for a in &nodes {
        for b in &nodes {
            if a == b {
                continue;
            }

            let mut edge = Edge { a, b, len: 0 };

            if map.contains_key(&(a, b)) || map.contains_key(&(b, a)) {
                continue;
            }
            map.insert((a, b), true);

            edge.len = get_distance(&a, &b);
            edges.push(edge);
            // let index = edges.len() - 1;
            //
            // let tx = tx.clone();
            // let a = a.clone();
            // let b = b.clone();
            // thread::spawn(move || {
            //     tx.send((index, get_distance(&a, &b))).unwrap();
            // });
            //
            // if index % 1000 == 0 {
            //     println!("Edges temp: {}", edges.len());
            // }
        }
    }

    // println!("Edges: {}", edges.len());
    //
    // for (i, len) in rx {
    //     edges[i].len = len;
    // }

    println!("Edges calculated!");

    edges.sort_by(|a, b| a.len.cmp(&b.len));

    let mut circuits: Vec<Vec<&Node>> = vec![];

    for edge in edges.iter().take(1000) {
        let index_a = circuits.iter().position(|c| c.iter().any(|n| *n == edge.a));
        let index_b = circuits.iter().position(|c| c.iter().any(|n| *n == edge.b));

        if index_a.is_none() && index_b.is_none() {
            // println!("{}: {}", circuits.len(), edge.a);
            // println!("{}: {}", circuits.len(), edge.b);
            circuits.push(vec![edge.a, edge.b]);

            continue;
        }

        if index_b.is_none()
            && let Some(i) = index_a
        {
            circuits[i].push(edge.b);
            // println!("{}: {}", i, edge.b);

            continue;
        }

        if index_a.is_none()
            && let Some(i) = index_b
        {
            circuits[i].push(edge.a);
            // println!("{}: {}", i, edge.a);

            continue;
        }

        let i_a = index_a.unwrap();
        let i_b = index_b.unwrap();

        if i_a == i_b {
            continue;
        }

        // println!("merge {} with {}", i_a, i_b);
        for node in circuits[i_b].clone() {
            if circuits[i_a].iter().any(|n| *n == node) {
                continue;
            }

            circuits[i_a].push(node);
        }
        circuits.remove(i_b);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    for circuit in circuits.iter().take(3) {
        // println!("Circuit: {}", circuit.len());
        // for node in circuit {
        //     println!("  {}", node);
        // }
        sum *= circuit.len() as u64;
    }

    sum
}

fn get_distance(a: &Node, b: &Node) -> u64 {
    let sum = (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2);

    (sum as f64).sqrt().round() as u64
}

pub fn part2(puzzle: &str) -> u64 {
    let mut sum = 1u64;

    let mut nodes: Vec<Node> = vec![];

    for line in puzzle.lines() {
        let mut parts = line.split(',');
        nodes.push(Node {
            x: parts.next().unwrap().parse::<i64>().unwrap(),
            y: parts.next().unwrap().parse::<i64>().unwrap(),
            z: parts.next().unwrap().parse::<i64>().unwrap(),
        })
    }

    let mut edges: Vec<Edge> = vec![];
    let mut map = HashMap::new();

    for a in &nodes {
        for b in &nodes {
            if a == b {
                continue;
            }

            let mut edge = Edge { a, b, len: 0 };

            if map.contains_key(&(a, b)) || map.contains_key(&(b, a)) {
                continue;
            }
            map.insert((a, b), true);

            edge.len = get_distance(&a, &b);
            edges.push(edge);
        }
    }

    println!("Edges calculated!");

    edges.sort_by(|a, b| a.len.cmp(&b.len));

    let mut circuits: Vec<Vec<&Node>> = vec![];

    for edge in edges.iter() {
        let index_a = circuits.iter().position(|c| c.iter().any(|n| *n == edge.a));
        let index_b = circuits.iter().position(|c| c.iter().any(|n| *n == edge.b));

        let mut index = 0usize;

        if index_a.is_none() && index_b.is_none() {
            index = circuits.len();
            circuits.push(vec![edge.a, edge.b]);
            // println!("{}: {}", index, edge.a);
            // println!("{}: {}", index, edge.b);
        } else if index_b.is_none()
            && let Some(i) = index_a
        {
            index = i;
            circuits[i].push(edge.b);
            // println!("{}: {}", i, edge.b);
        } else if index_a.is_none()
            && let Some(i) = index_b
        {
            index = i;
            circuits[i].push(edge.a);
            // println!("{}: {}", i, edge.a);
        } else {
            let i_a = index_a.unwrap();
            let i_b = index_b.unwrap();

            index = i_a;

            if i_a == i_b {
                continue;
            }

            // println!("merge {} with {}", i_a, i_b);
            for node in circuits[i_b].clone() {
                if circuits[i_a].iter().any(|n| *n == node) {
                    continue;
                }

                circuits[i_a].push(node);
            }
            circuits.remove(i_b);
            if i_a > i_b {
                index -= 1;
            }
        }

        if nodes.len() == circuits[index].len() {
            return (edge.a.x * edge.b.x) as u64;
        }
    }

    sum
}
