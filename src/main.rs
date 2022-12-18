use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub mod graph;
pub mod node;
pub mod svg;
pub mod vector;

fn draw_graph(g: &graph::Graph) -> String {
    let mut svg = String::new();

    for edge in &g.edges {
        if edge.visible == false {
            continue;
        }

        let a = g.nodes.get(edge.a).unwrap();
        let b = g.nodes.get(edge.b).unwrap();

        svg += svg::draw_line(
            vector::Vector { x: a.x, y: a.y },
            vector::Vector { x: b.x, y: b.y },
        )
        .as_str();
    }

    for key in &g.nodes {
        let node = key.1;
        svg += node.draw().as_str();
    }

    svg
}

fn main() {
    let filename = env::args().nth(1).unwrap();

    let mut input_file = File::open(filename).unwrap();
    let mut contents = String::new();
    input_file.read_to_string(&mut contents).unwrap();

    let mut g = graph::Graph::new();

    for line in contents.split("\n") {
        if line != "" {
            let (content, style) = match line.split("|").collect::<Vec<&str>>()[..] {
                [content] => (content, ""),
                [content, style] => (content, style),
                _ => panic!("Failed to process text file."),
            };

            match content.split("-").collect::<Vec<&str>>()[..] {
                [a] => g.register_node(a, style),
                [a, b] => g.register_edge(a, b, style),
                _ => panic!("Failed to process text file."),
            }
        }
    }

    match g.is_fully_connected() {
        Ok(_) => {}
        Err(e) => eprintln!(
            "WARNING: Graph not fully connected!\n\
            \n\
            Connected nodes: {:?}\n\
            Disconnected nodes:{:?}",
            e.0, e.1
        ),
    };

    let mut output_file = File::create("out.svg").unwrap();

    let mut svg = String::new();
    svg += format!("{}\n", svg::start(0.0, 0.0, 1.0, 1.0)).as_str();

    for _ in 1..2000 {
        g.iterate();
    }

    g.normalize();

    svg += svg::draw_box(0.0, 0.0, 1.0, 1.0, 0.01, "white", "black").as_str();

    svg += draw_graph(&g).as_str();

    svg += format!("{}\n", svg::end()).as_str();

    output_file.write(svg.as_bytes()).unwrap();
}
