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
}
