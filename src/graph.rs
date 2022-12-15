use rand::{Rng, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use rand_pcg::Pcg32;
use std::collections::HashMap;

pub struct Node<'a> {
    pub x: f32,
    pub y: f32,
    pub name: &'a str,
    visited: bool,
}

pub struct Graph<'a> {
    pub nodes: HashMap<&'a str, Node<'a>>,
    pub edges: Vec<(&'a str, &'a str, bool)>,
    rng: Lcg64Xsh32,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            rng: Pcg32::seed_from_u64(48),
        }
    }

    pub fn register_edge(&mut self, a: &'a str, b: &'a str, visible: bool) {
        let x1 = self.rng.gen_range(0.0..1.0);
        let x2 = self.rng.gen_range(0.0..1.0);
        let y1 = self.rng.gen_range(0.0..1.0);
        let y2 = self.rng.gen_range(0.0..1.0);

        self.nodes.insert(
            a,
            Node {
                x: x1,
                y: y1,
                name: a,
                visited: false,
            },
        );

        self.nodes.insert(
            b,
            Node {
                x: x2,
                y: y2,
                name: b,
                visited: false,
            },
        );

        self.edges.push((a, b, visible));
    }

    pub fn iterate(&mut self) {
        let mut node_data: Vec<(f32, f32, &str)> = Vec::new();

        for node in &self.nodes {
            node_data.push((node.1.x, node.1.y, node.1.name));
        }

        // Simulate repulsion
        for mut node in self.nodes.iter_mut() {
            for other in &node_data {
                if node.1.name == other.2 {
                    continue;
                }

                let xdist = node.1.x - other.0;
                let ydist = node.1.y - other.1;
                let dist_2 = xdist * xdist + ydist * ydist;
                let force = 0.0003 / dist_2;

                let x_component = xdist / dist_2.sqrt();
                let y_component = ydist / dist_2.sqrt();

                node.1.x += force * x_component;
                node.1.y += force * y_component;
            }
        }

        // Simulate Hooke's law
        for edge in self.edges.iter() {
            let length = 0.3;

            let a = self.nodes.get(edge.0).unwrap();
            let b = self.nodes.get(edge.1).unwrap();
            let xdelta = a.x - b.x;
            let ydelta = a.y - b.y;
            let dist = (xdelta * xdelta + ydelta * ydelta).sqrt();

            let x = dist - length;

            let f = -0.1 * x;

            let a = self.nodes.get_mut(edge.0).unwrap();

            a.x += xdelta * f;
            a.y += ydelta * f;

            let b = self.nodes.get_mut(edge.1).unwrap();

            b.x += -xdelta * f;
            b.y += -ydelta * f;
        }
    }
}
