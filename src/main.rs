use std::fs;

#[derive(Debug)]
struct InputLine {
    source: String,
    sinks: Vec<String>,
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

fn main() {
    let filename= "example.txt".to_string();
    let input_lines = read_input(filename);
    // Find the vertices that point to each vertex in our digraph.
    // At each step, consider a vertex that has no more paths leading to it.
    // Add the number of paths to this vertex to all its children.
    
    // println!("Hello, world! {:?}", input_lines);
}
