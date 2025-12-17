use std::{collections::HashMap, fs, path};

#[derive(Debug)]
struct InputLine {
    source: String,
    sinks: Vec<String>,
}

#[derive(Debug)]
struct Graph{
    vertices: Vec<String>,
    forward_adjacency: HashMap<String, Vec<String>>,
    backward_adjacency: HashMap<String, Vec<String>>,
}

fn read_input(filename: String) -> Vec<InputLine> {
    let contents = fs::read_to_string(filename).expect("file exists.");
    let lines = contents.trim().split("\n").map(
        |l| l.split(":")
    ).map(
        |p| InputLine{
                source: p.clone().nth(0).unwrap().to_string(),
                sinks: p.clone().nth(1).unwrap().trim().split(" ").map(|s| s.trim().to_string()).collect::<Vec<String>>()
            }
    );
    return lines.collect();
}

fn build_graph(input_lines: &Vec<InputLine>) -> Graph {
    let vertices: Vec<String> = input_lines.iter().map(|l| l.source.clone()).collect();
    let mut vertices_map: HashMap<String, Vec<String>> = HashMap::new();
    for vertex in vertices.iter(){
        vertices_map.insert(vertex.to_string(), Vec::new());
    };
    let mut graph = Graph{
        vertices: vertices,
        forward_adjacency: vertices_map.clone(),
        backward_adjacency: vertices_map,
    };
    for line in input_lines.iter(){
        let source = &line.source;
        let sinks = &line.sinks;
        for sink in sinks.iter(){
            if let Some(fvs) = graph.forward_adjacency.get_mut(source){
                fvs.push(sink.to_string());
            }
            else{
                println!("source |{}| sink |{}|", source, sink);
            }
            if let Some(bvs) = graph.backward_adjacency.get_mut(sink){
                bvs.push(source.to_string());
            }
        }
    }
    return graph;
}

fn find_nice_vertex(graph: &Graph, path_counts: &HashMap<String, i32>) -> String{
    for vertex in graph.vertices.iter(){
        if path_counts.contains_key(vertex){
            continue;
        }
        let incoming = &(graph.backward_adjacency[vertex]);
        let mut nice: bool = true;
        for prev in incoming.iter(){
            if !path_counts.contains_key(prev){
                nice = false;
            }
        }
        if nice {
            return vertex.to_string();
        } 
    }
    panic!()
}

fn count_paths(graph: &Graph) -> HashMap<String, i32> {
    let mut path_counts: HashMap<String, i32> = HashMap::new();
    path_counts.insert("you".to_string(), 1);
    // We need to find a vertex such that all of its incoming edges already have a path_count
    while path_counts.len() < graph.vertices.len() {
        let nice_vertex = find_nice_vertex(graph, &path_counts);
        println!("L {} nv {}", path_counts.len(), nice_vertex);
        let count = graph.backward_adjacency[&nice_vertex].iter().map(|v| path_counts[v]).sum();
        path_counts.insert(nice_vertex, count);
    }
    return path_counts;
}

fn main() {
    // let filename= "example.txt".to_string();
    let filename= "input.txt".to_string();
    let input_lines = read_input(filename);
    // Find the vertices that point to each vertex in our digraph.
    // At each step, consider a vertex that has no more paths leading to it.
    // Add the number of paths to this vertex to all its children.
    let graph = build_graph(&input_lines);

    let path_counts = count_paths(&graph);
    // println!("Graph: {:?}", graph);
    println!("Paths: {:?}", path_counts["out"])
}
