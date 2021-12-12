use advent_of_code_2021::read_file_to_string;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Debug)]
enum NodeType {
    Big,
    Small,
    Start,
    End,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node {
    name: String,
    node_type: NodeType,
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cave_type = if s == "start" {
            NodeType::Start
        } else if s == "end" {
            NodeType::End
        } else if s.chars().all(|c| c.is_uppercase()) {
            NodeType::Big
        } else {
            NodeType::Small
        };
        let name = s.to_string();
        Ok(Self {
            name,
            node_type: cave_type,
        })
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Edge {
    u: String,
    v: String,
}

impl Edge {
    pub fn new(u_name: &str, v_name: &str) -> Self {
        Self {
            u: u_name.to_string(),
            v: v_name.to_string(),
        }
    }

    pub fn get_other_side(&self, name: &str) -> Option<&str> {
        if name == self.u {
            Some(&self.v)
        } else if name == self.v {
            Some(&self.u)
        } else {
            None
        }
    }
}

fn format_path(path: &Vec<&Node>) -> String {
    path.iter().map(|n| n.name.clone()).join(",")
}

struct CaveSystem {
    nodes: HashSet<Node>,
    edges: HashSet<Edge>,
}

impl CaveSystem {
    pub fn parse(s: &str) -> Self {
        let mut nodes = HashSet::new();
        let mut edges = HashSet::new();
        for line in s.lines() {
            let (u_str, v_str) = line.split_once('-').unwrap();
            let u = Node::from_str(u_str).unwrap();
            let v = Node::from_str(v_str).unwrap();
            nodes.insert(u);
            nodes.insert(v);

            let edge = Edge::new(u_str, v_str);
            edges.insert(edge);
        }

        Self { nodes, edges }
    }

    pub fn find_num_of_all_paths(&self) -> u32 {
        let mut found_paths: HashSet<String> = HashSet::new();

        let start = self.get_node_by_name("start").unwrap();
        self.dfs_step(&start, Vec::new(), &mut found_paths);

        found_paths.len() as u32
    }

    fn dfs_step<'a>(
        &self,
        node: &'a Node,
        mut path: Vec<&'a Node>,
        found_paths: &mut HashSet<String>,
    ) {
        if node.node_type == NodeType::Small && path.contains(&node) {
            return;
        }
        if node.node_type == NodeType::Start && !path.is_empty() {
            return;
        }
        path.push(node);
        if node.node_type == NodeType::End {
            let path_string = format_path(&path);
            println!("Found a path: {}", &path_string);
            found_paths.insert(path_string);
            return;
        }

        for n in self.get_node_neighbours(node) {
            self.dfs_step(&n, path.clone(), found_paths);
        }
    }

    fn get_node_neighbours<'a>(&'a self, node: &'a Node) -> impl Iterator<Item = &'a Node> {
        self.edges.iter().filter_map(|e| {
            e.get_other_side(&node.name)
                .map(|name| self.get_node_by_name(name))
                .flatten()
        })
    }

    fn get_node_by_name(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }
}

fn main() {
    let input = read_file_to_string("input/day12.txt");
    let system = CaveSystem::parse(&input);
    let num_of_paths = system.find_num_of_all_paths();
    println!("The number of all paths found is {}", num_of_paths);
}
