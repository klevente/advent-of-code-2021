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

fn can_step_on_node(node: &Node, path: &Vec<&Node>, node_visitable_twice: &Option<&Node>) -> bool {
    return if node.node_type == NodeType::Start && !path.is_empty() {
        // cannot step on `start`
        false
    } else if node.node_type == NodeType::Small {
        // check if there is a `Node` that can be visited twice
        if let Some(node_visitable_twice) = node_visitable_twice {
            if node == *node_visitable_twice {
                // if the current `Node` is the one that can be visited twice, count its occurrences, and return `true` if it is less than 2
                path.iter().filter(|&n| n == node_visitable_twice).count() < 2
            } else {
                // if the current `Node` is not visitable twice, check if it has already been visited, return `false` if it has been
                !path.contains(&node)
            }
        } else {
            // if there is no twice-visitable `Node`, check if the `Node` has already been visited, return `false` if it has been
            !path.contains(&node)
        }
    } else {
        // if no other conditions are true, the `Node` can be stepped on
        true
    };
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

    pub fn find_num_of_all_paths_small_caves_count_once(&self) -> u32 {
        let mut found_paths: HashSet<String> = HashSet::new();

        let start = self.get_node_by_name("start").unwrap();
        self.dfs_step(&start, Vec::new(), &mut found_paths, None);

        found_paths.len() as u32
    }

    pub fn find_num_of_all_paths_one_small_cave_counts_twice(&self) -> u32 {
        let mut found_paths: HashSet<String> = HashSet::new();

        let start = self.get_node_by_name("start").unwrap();
        let small_nodes = self.nodes.iter().filter(|n| n.node_type == NodeType::Small);

        for small_node in small_nodes {
            // select each small `Node` to be visitable twice for generating paths
            self.dfs_step(&start, Vec::new(), &mut found_paths, Some(small_node));
        }

        found_paths.len() as u32
    }

    fn dfs_step<'a>(
        &self,
        node: &'a Node,
        mut path: Vec<&'a Node>,
        found_paths: &mut HashSet<String>,
        node_visitable_twice: Option<&Node>,
    ) {
        if !can_step_on_node(node, &path, &node_visitable_twice) {
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
            self.dfs_step(&n, path.clone(), found_paths, node_visitable_twice);
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
    let num_of_paths_small_caves_count_once = system.find_num_of_all_paths_small_caves_count_once();
    println!(
        "The number of all paths found when small caves count only once is {}",
        num_of_paths_small_caves_count_once
    );

    let num_of_paths_one_small_cave_counts_twice =
        system.find_num_of_all_paths_one_small_cave_counts_twice();
    println!(
        "The number of all paths found when one small cave counts twice is {}",
        num_of_paths_one_small_cave_counts_twice
    );
}
