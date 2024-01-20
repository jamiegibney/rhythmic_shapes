//! Module for individual shape sequencers.

use std::sync::{mpsc, Arc};

use super::*;
use crate::{audio::voice::NoteEventData, prelude::*};

mod node;
use node::Node;

const MIN_NUM_VERTICES: usize = 3;
const MAX_NUM_VERTICES: usize = 8;

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

    pub fn draw(&self, draw: &Draw) {
        draw.line()
            .start(self.start)
            .end(self.end)
            .color(BLACK)
            .weight(3.0);
    }
}

#[derive(Clone, Debug)]
pub struct Sequence {
    /// All the vertices of the shape.
    nodes: Vec<Node>,
    clicked_idx: Option<usize>,
    /// All the segments connecting each vertex.
    segments: Vec<Segment>,
    /// The length of the whole shape.
    length: f32,

    last_behind_idx: usize,

    /// The sequence's progress in the range `[0, 1)`.
    progress: f32,
    /// The position of the "playhead".
    progress_node: Vec2,
    /// The tempo of the sequence in beats per minute (BPM).
    tempo: f32,

    /// The number of vertices.
    num_nodes: usize,

    note_data_sender: mpsc::Sender<NoteEventData>,

    /// The sequence's bounding rect.
    rect: Rect,
}

impl Sequence {
    pub fn new(
        rect: Rect,
        note_data_sender: mpsc::Sender<NoteEventData>,
        num_init_nodes: usize,
        tempo: f32,
    ) -> Self {
        let mut s = Self {
            nodes: vec![Node::new(rect); MAX_NUM_VERTICES],
            clicked_idx: None,
            segments: vec![Segment::default(); MAX_NUM_VERTICES],
            length: 0.0,
            last_behind_idx: 0,

            progress: 0.0,
            progress_node: Vec2::ZERO,
            tempo,

            num_nodes: num_init_nodes,

            note_data_sender,

            rect,
        };

        s.emplace_nodes();
        s.update_segments();
        s.update_length();

        s.progress_node = s.nodes[0].pos;
        s.nodes[0].note_data_mut().note += 12.0;
        s.nodes[0].color = Rgba::new(1.0, 0.0, 1.0, 1.0);

        s
    }

    /// Sets the number of active nodes in the sequence (i.e., the number of vertices).
    ///
    /// Value is clamped between [`MIN_NUM_VERTICES`] and [`MAX_NUM_VERTICES`].
    pub fn set_num_nodes(&mut self, num: usize) {
        self.num_nodes = num.clamp(MIN_NUM_VERTICES, MAX_NUM_VERTICES);
        self.emplace_nodes();
        self.update_segments();
        self.update_length();
    }

    /// Sets the tempo (speed) of the sequencer.
    pub fn set_tempo(&mut self, bpm: f32) {
        self.tempo = bpm;
    }

    /// Resets the sequence — the position of nodes and its progress.
    ///
    /// Does not affect the number of nodes.
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.emplace_nodes();
        self.update_segments();
        self.update_length();
    }

    /// Updates the positions and lengths of each segment.
    fn update_segments(&mut self) {
        let num = self.num_nodes;

        for i in 0..num {
            let start = &self.nodes[i];
            let end = &self.nodes[(i + 1) % num];

            self.segments[i].start = start.pos;
            self.segments[i].end = end.pos;
            self.segments[i].calculate_distance();
        }
    }

    /// Updates the length of the entire shape.
    fn update_length(&mut self) {
        self.length = self
            .segments
            .iter()
            .take(self.num_nodes)
            .map(|sg| sg.length)
            .sum();
    }

    fn update_progress(&mut self, delta_time: f32) {
        let time_per_bar = 60.0 / self.tempo * 4.0;
        let time_increment = time_per_bar.recip() * delta_time;

        self.progress += time_increment;

        if self.progress > 1.0 {
            self.progress -= 1.0;
        }
    }

    fn update_progress_node(&mut self) {
        let progress = self.length * self.progress;
        let mut idx = 0;
        let mut lower_len = 0.0;
        let mut upper_len = 0.0;

        for i in 0..self.num_nodes {
            lower_len = upper_len;
            upper_len += self.segments[i].length;

            if (lower_len..=upper_len).contains(&progress) {
                idx = i;
                break;
            }
        }

        let interp = ilerp(lower_len, upper_len, progress);
        let index = |i| i % self.num_nodes;

        self.progress_node = self.nodes[index(idx)]
            .pos
            .lerp(self.nodes[index(idx + 1)].pos, interp);

        if idx != self.last_behind_idx {
            self.last_behind_idx = idx;
            self.tap();
        }
    }

    fn update_node_flash(&mut self, input_data: &InputData) {
        for i in 0..self.num_nodes {
            self.nodes[i].update_flash_timer(input_data);
        }
    }

    fn tap(&mut self) {
        self.nodes[self.last_behind_idx].tap();
        self.note_data_sender
            .send(self.nodes[self.last_behind_idx].note_data())
            .expect("failed to send note data");
    }

    fn find_lower_node_idx(&self) -> Option<usize> {
        let pos = self.length * self.progress;
        let mut lower_bound = 0.0;
        let mut upper_bound = 0.0;

        for i in 0..self.num_nodes {
            upper_bound += self.segments[i].length;

            if lower_bound <= pos && pos < upper_bound {
                return Some(i);
            }

            lower_bound = upper_bound;
        }

        None
    }

    fn emplace_nodes(&mut self) {
        let delta_angle = TAU / self.num_nodes as f32;
        let shape_radius = 250.0;

        for i in 0..self.num_nodes {
            let idx = self.num_nodes - i;
            let dlt_angle = idx as f32 * delta_angle + PI * 0.5;
            let x = shape_radius * dlt_angle.cos();
            let y = shape_radius * dlt_angle.sin();

            self.nodes[i].pos.x = x;
            self.nodes[i].pos.y = y;
        }
    }
}

impl Drawable for Sequence {
    fn update(&mut self, input_data: &InputData) {
        if !input_data.is_left_clicked {
            self.clicked_idx = None;
        }

        self.update_node_flash(input_data);

        'update_nodes: {
            if let Some(idx) = self.clicked_idx {
                self.nodes[idx].update(input_data);
                break 'update_nodes;
            }

            for (i, node) in
                self.nodes.iter_mut().enumerate().take(self.num_nodes)
            {
                node.update(input_data);

                if node.is_clicked() {
                    self.clicked_idx = Some(i);
                    break;
                }
            }
        }

        if self.clicked_idx.is_some() {
            self.update_segments();
            self.update_length();
        }

        self.update_progress(input_data.delta_time);
        self.update_progress_node();
    }

    fn draw(&self, draw: &Draw, frame: &Frame) {
        for segment in self.segments.iter().take(self.num_nodes) {
            segment.draw(draw);
        }

        for node in self.nodes.iter().take(self.num_nodes) {
            node.draw(draw, frame);
        }

        draw.ellipse().color(RED).radius(8.0).xy(self.progress_node);
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
