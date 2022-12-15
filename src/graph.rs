use rand::SeedableRng;
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
}
