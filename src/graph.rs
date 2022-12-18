use rand::{Rng, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use rand_pcg::Pcg32;
use std::collections::HashMap;

use crate::node::Node;

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

    pub fn register_edge(&mut self, a: &'a str, b: &'a str, c: &'a str) {
        let mut visible = true;

        if c != "" {
            let json: serde_json::Value = serde_json::from_str(c).unwrap();

            match json.get("visible") {
                Some(x) => match x.as_bool() {
                    Some(x) => visible = x,
                    None => {}
                },
                None => {}
            }

            println!("{:?}", json);
        }

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
        node_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

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

    pub fn normalize(&mut self) {
        let mut minx = self.nodes.iter().next().unwrap().1.x;
        let mut miny = self.nodes.iter().next().unwrap().1.y;
        let mut maxx = self.nodes.iter().next().unwrap().1.x;
        let mut maxy = self.nodes.iter().next().unwrap().1.y;

        for node in self.nodes.iter() {
            if node.1.x < minx {
                minx = node.1.x;
            }
            if node.1.y < miny {
                miny = node.1.y;
            }
            if node.1.x > maxx {
                maxx = node.1.x;
            }
            if node.1.y > maxy {
                maxy = node.1.y;
            }
        }

        for node in self.nodes.iter_mut() {
            node.1.x -= minx;
            node.1.y -= miny;
            node.1.x *= 1.0 / (maxx - minx);
            node.1.y *= 1.0 / (maxy - miny);

            node.1.x *= 0.8;
            node.1.y *= 0.8;
            node.1.x += 0.1;
            node.1.y += 0.1;
        }
    }

    pub fn is_fully_connected(&mut self) -> Result<(), (Vec<&str>, Vec<&str>)> {
        for node in self.nodes.iter_mut() {
            node.1.visited = false;
        }

        self.nodes.iter_mut().next().unwrap().1.visited = true;

        loop {
            let mut cont = false;

            for edge in self.edges.iter_mut() {
                let a = self.nodes.get(edge.0).unwrap();
                let b = self.nodes.get(edge.1).unwrap();

                if a.visited && b.visited {
                    continue;
                }

                if !a.visited && !b.visited {
                    continue;
                }

                let a = self.nodes.get_mut(edge.0).unwrap();
                a.visited = true;

                let b = self.nodes.get_mut(edge.1).unwrap();
                b.visited = true;
                cont = true;
            }

            if !cont {
                break;
            }
        }

        let mut unconnected = Vec::new();
        for node in &self.nodes {
            if node.1.visited == false {
                unconnected.push(*node.0);
            }
        }

        let mut connected = Vec::new();
        for node in &self.nodes {
            if node.1.visited == true {
                connected.push(*node.0);
            }
        }

        if unconnected.len() > 0 {
            return std::result::Result::Err((connected, unconnected));
        }

        std::result::Result::Ok(())
    }
}
