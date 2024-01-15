//! Module for individual shape sequencers.

use super::*;
use crate::prelude::*;

mod node;
use node::Node;

#[derive(Clone, Copy, Debug, Default)]
struct Segment {
    pub start: Vec2,
    pub end: Vec2,
    pub length: f32,
}

impl Segment {
    pub fn calculate_distance(&mut self) {
        self.length = self.start.distance(self.end);
    }
}

#[derive(Clone, Debug)]
pub struct Sequence {
    /// All the vertices of the shape.
    nodes: Vec<Node>,
    /// All the segments connecting each vertex.
    segments: Vec<Segment>,
    /// The length of the whole shape.
    length: f32,

    /// The sequence's progress in the range `[0, 1)`.
    progress: f32,

    /// The number of vertices.
    num_nodes: usize,

    /// The sequence's bounding rect.
    rect: Rect,
}

impl Sequence {
    pub fn new(rect: Rect, num_init_nodes: usize) -> Self {
        Self {
            nodes: vec![Node::new(rect); num_init_nodes],
            segments: vec![Segment::default(); num_init_nodes],
            length: 0.0,

            progress: 0.0,

            num_nodes: num_init_nodes,

            rect,
        }
    }

    /// Updates the positions and lengths of each segment.
    fn update_segments(&mut self) {
        let num = self.num_nodes;

        for i in 0..num {
            let start = self.nodes[i];
            let end = self.nodes[(i + 1) % num];

            self.segments[i].start = start.pos;
            self.segments[i].end = end.pos;
            self.segments[i].calculate_distance();
        }
    }

    /// Updates the length of the entire shape.
    fn update_length(&mut self) {
        self.length = self.segments.iter().map(|sg| sg.length).sum();
    }
}

impl Drawable for Sequence {
    fn update(&mut self, input_data: &InputData) {
        todo!();
        for node in &mut self.nodes {
            node.update(input_data);
        }
    }

    fn draw(&self, draw: &Draw, frame: &Frame) {
        todo!();
        for node in &self.nodes {
            node.draw(draw, frame);
        }
    }

    fn force_redraw(&self, draw: &Draw, frame: &Frame) {
        todo!();
        for node in &self.nodes {
            node.force_redraw(draw, frame);
        }
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }
}
