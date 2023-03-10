use rand::{Rng, SeedableRng};
use rand_pcg::Lcg64Xsh32;
use rand_pcg::Pcg32;
use std::collections::HashMap;

use crate::node::Node;

pub struct Edge<'a> {
    pub a: &'a str,
    pub b: &'a str,
    pub visible: bool,
    length: f32,
}

pub struct Graph<'a> {
    pub nodes: HashMap<&'a str, Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    rng: Lcg64Xsh32,
}

impl<'a> Default for Graph<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Graph<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            rng: Pcg32::seed_from_u64(48),
        }
    }

    fn retrieve_bool(&self, style: &str, key: &str, default: bool) -> bool {
        let json: serde_json::Value = serde_json::from_str(style).expect("Could not parse JSON.");

        json.get(key).map_or(default, |x| match x.as_bool() {
            Some(x) => x,
            None => default,
        })
    }

    fn retrieve_string(&self, style: &str, key: &str, default: &'a str) -> String {
        let json: serde_json::Value = serde_json::from_str(style).expect("Could not parse JSON.");

        let ret = json.get(key).map_or(default, |x| match x.as_str() {
            Some(x) => x,
            None => default,
        });

        ret.to_string()
    }

    pub fn register_node(&mut self, a: &'a str, style: &'a str) {
        self.nodes
            .get_mut(a)
            .expect("Could not find node. Edges must be defined before nodes can be configured.")
            .visible = self.retrieve_bool(style, "visible", true);

        self.nodes
            .get_mut(a)
            .expect("Could not find node. Edges must be defined before nodes can be configured.")
            .color = self.retrieve_string(style, "color", "green");
    }

    pub fn register_edge(&mut self, a: &'a str, b: &'a str, style: &'a str) {
        let mut visible = true;
        let mut length = 0.3;

        if !style.is_empty() {
            let json: serde_json::Value =
                serde_json::from_str(style).expect("Could not parse JSON.");

            json.get("visible").map_or({}, |x| match x.as_bool() {
                Some(x) => visible = x,
                None => {}
            });

            json.get("length").map_or({}, |x| match x.as_f64() {
                Some(x) => length = x,
                None => {}
            });
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
                color: "black".to_string(),
                visited: false,
                visible: true,
            },
        );

        self.nodes.insert(
            b,
            Node {
                x: x2,
                y: y2,
                name: b,
                color: "black".to_string(),
                visited: false,
                visible: true,
            },
        );

        self.edges.push(Edge {
            a,
            b,
            visible,
            length: length as f32,
        });
    }

    pub fn iterate(&mut self) {
        let mut node_data: Vec<(f32, f32, &str)> = Vec::new();

        for node in &self.nodes {
            node_data.push((node.1.x, node.1.y, node.1.name));
        }

        // Simulate repulsion
        node_data.sort_by(|a, b| {
            a.partial_cmp(b)
                .expect("Could not compare node coordinates.")
        });

        for mut node in &mut self.nodes {
            for other in &node_data {
                if node.1.name == other.2 {
                    continue;
                }

                let xdist = node.1.x - other.0;
                let ydist = node.1.y - other.1;
                let dist_2 = xdist.mul_add(xdist, ydist * ydist);
                let force = 0.0003 / dist_2;

                let x_component = xdist / dist_2.sqrt();
                let y_component = ydist / dist_2.sqrt();

                node.1.x += force * x_component;
                node.1.y += force * y_component;
            }
        }

        // Simulate Hooke's law
        for edge in &self.edges {
            let length = edge.length;

            let a = self.nodes.get(edge.a).expect("Could not get named node.");
            let b = self.nodes.get(edge.b).expect("Could not get named node.");
            let xdelta = a.x - b.x;
            let ydelta = a.y - b.y;
            let dist = xdelta.hypot(ydelta);

            let x = dist - length;

            let f = -0.1 * x;

            let a = self
                .nodes
                .get_mut(edge.a)
                .expect("Could not get named node.");

            a.x += xdelta * f;
            a.y += ydelta * f;

            let b = self
                .nodes
                .get_mut(edge.b)
                .expect("Could not get named node.");

            b.x += -xdelta * f;
            b.y += -ydelta * f;
        }
    }

    pub fn normalize(&mut self) {
        let mut minx = self
            .nodes
            .iter()
            .next()
            .expect("Could not get first node.")
            .1
            .x;
        let mut miny = self
            .nodes
            .iter()
            .next()
            .expect("Could not get first node.")
            .1
            .y;
        let mut maxx = self
            .nodes
            .iter()
            .next()
            .expect("Could not get first node.")
            .1
            .x;
        let mut maxy = self
            .nodes
            .iter()
            .next()
            .expect("Could not get first node.")
            .1
            .y;

        for node in &self.nodes {
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

        for node in &mut self.nodes {
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
        for node in &mut self.nodes {
            node.1.visited = false;
        }

        self.nodes
            .iter_mut()
            .next()
            .expect("Could not get first node.")
            .1
            .visited = true;

        loop {
            let mut cont = false;

            for edge in &mut self.edges {
                let a = self.nodes.get(edge.a).expect("Could not get named node.");
                let b = self.nodes.get(edge.b).expect("Could not get named node.");

                if a.visited && b.visited {
                    continue;
                }

                if !a.visited && !b.visited {
                    continue;
                }

                let a = self
                    .nodes
                    .get_mut(edge.a)
                    .expect("Could not get named node.");
                a.visited = true;

                let b = self
                    .nodes
                    .get_mut(edge.b)
                    .expect("Could not get named node.");
                b.visited = true;
                cont = true;
            }

            if !cont {
                break;
            }
        }

        let mut unconnected = Vec::new();
        for node in &self.nodes {
            if !node.1.visited {
                unconnected.push(*node.0);
            }
        }

        let mut connected = Vec::new();
        for node in &self.nodes {
            if node.1.visited {
                connected.push(*node.0);
            }
        }

        if !unconnected.is_empty() {
            return std::result::Result::Err((connected, unconnected));
        }

        std::result::Result::Ok(())
    }
}
