use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_graph(path: &str) -> Vec<Vec<usize>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let vertex_count = first_line.parse::<usize>().unwrap();

    let mut graph = vec![Vec::new(); vertex_count];

    for line in lines {
        let unwrapped_line = line.unwrap(); 
        let edge = unwrapped_line.split_whitespace().collect::<Vec<&str>>();
        let from = edge[0].parse::<usize>().unwrap(); 
        let to = edge[1].parse::<usize>().unwrap();
        graph[from].push(to);
    }

    graph
}
