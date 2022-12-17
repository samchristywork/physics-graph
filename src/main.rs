use std::fs::File;
use std::io::Read;
use std::io::Write;

pub mod graph;
pub mod svg;
pub mod vector;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let mut g = graph::Graph::new();

    for line in contents.split("\n") {
        if line != "" {
            match line.split("-").collect::<Vec<&str>>()[..] {
                [a, b] => g.register_edge(a, b, true),
                _ => panic!("Failed to process text file."),
            }
        }
    }

    match g.is_fully_connected() {
        Ok(_) => {}
        Err(e) => panic!("Graph not fully connected! Missing {:?}.", e),
    }

    let mut output_file = File::create("out.svg").unwrap();

    let mut svg = String::new();
    svg += format!("{}\n", svg::start(0.0, 0.0, 1.0, 1.0)).as_str();

    svg += svg::draw_box(0.0, 0.0, 1.0, 1.0, 0.01, "red").as_str();

    svg += format!("{}\n", svg::end()).as_str();

    output_file.write(svg.as_bytes()).unwrap();
}
