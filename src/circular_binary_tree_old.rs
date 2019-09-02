use crate::position::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Circle;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::element::Text as TextNode;
use svg::node::Node;
use svg::node::Text;

pub struct CircularBinaryTree {
    center: Position,
    segment_width: f64,
    segment_width_multipier: f64,
    num_segments: u32,
    inner_segment_width: f64,
    number_x_offset: f64,
    number_y_offset: f64,
    draw_numbers: bool,
    line_width: f64,
    initial_font_size: f64,
    font_size_modifier: f64,
    font_family: String,
}

impl CircularBinaryTree {
    pub fn new(
        width: f64,
        height: f64,
        margin: f64,
        num_segments: u32,
        inner_segment_width_percentage: f64,
        number_x_offset: f64,
        number_y_offset: f64,
        draw_numbers: bool,
        line_width: f64,
    initial_font_size: u64,
    font_size_modifier: u64,
        font_family: String,
    ) -> Self {
        let segment_width = ((width / 2.0) - margin) / ((num_segments) as f64);
        CircularBinaryTree {
            center: Position::new(width / 2.0, 0.0, height),
            segment_width,
            segment_width_multipier: 1.0,
            num_segments,
            inner_segment_width: segment_width * inner_segment_width_percentage,
            number_x_offset,
            number_y_offset,
            draw_numbers,
            line_width,
    initial_font_size,
    font_size_modifier,
            font_family,
        }
    }
    pub fn draw(&self) -> Group {
        let segments = draw_segment_lines(
            self.center,
            self.segment_width,
            self.segment_width_multipier,
            self.num_segments,
        );
        let sectors = draw_sectors(
            self.center,
            self.segment_width,
            self.segment_width_multipier,
            self.inner_segment_width,
            self.num_segments,
        );
        let numbers = draw_numbers(
            self.center,
            self.segment_width,
            self.segment_width_multipier,
            self.number_x_offset,
            self.number_y_offset,
            self.num_segments,
        );

        let mut g = Group::new()
            .set("font-family", self.font_family.as_ref())
            .set("fill", "none")
            .set("stroke", "black")
            .set("style", "white-space: pre;")
            .set("stroke-width", self.line_width);
        g = g.add(segments).add(sectors);
        if self.draw_numbers {
            g = g.add(numbers)
        }
        g
    }
}

fn draw_sectors(
    center: Position,
    segment_width: f64,
    segment_width_multipier: f64,
    inner_segment_width: f64,
    num_segments: u32,
) -> impl Node {
    let mut g = Group::new();
    let mut length = segment_width;
    for _ in 0..num_segments {
        let outer = Circle::new()
            .set("cx", center.x())
            .set("cy", center.y())
            .set("r", length);
        let inner = Circle::new()
            .set("cx", center.x())
            .set("cy", center.y())
            .set("r", length - inner_segment_width);
        //let outer = Path::new().set("d", Data::new().elliptical_arc_to((length,length, 0, 0, 0, length/2.0, length/2.0)));
        g = g.add(outer).add(inner);
        length += segment_width * segment_width_multipier;
    }
    g
}
fn draw_numbers(
    center: Position,
    segment_width: f64,
    segment_width_multipier: f64,
    x_offset: f64,
    y_offset: f64,
    num_segments: u32,
) -> impl Node {
    let mut g = Group::new();
    let mut length = segment_width;
    let width = (num_segments as f64 * 2f64.ln() / (2f64.ln() + 5f64.ln())).ceil() as usize;
    for x in 0..num_segments {
        let curr_pos = center + Position::new(-length + x_offset, y_offset, center.height);
        let max = 2u64.pow(x + 1);
        let angle_modifier = PI / (max as f64);
        let mut angle = angle_modifier;
        for y in 0..max {
            let num = 2u64.pow(x + 1) + y - 1;
            let rot_str_left = format!(
                "rotate({},{},{})",
                angle * 180.0 / PI,
                center.x(),
                center.y()
            );
            g = g.add(
                TextNode::new()
                    .set("x", curr_pos.x())
                    .set("y", curr_pos.y())
                    .set("transform", rot_str_left)
                    .set("font-size", self.initial_font_size - (self.font_size_modifier * depth))
                    .add(Text::new(format!("{:width$}", num, width = width))),
            );
            angle += angle_modifier;
        }
        //g = g.add(outer).add(inner);
        length += segment_width * segment_width_multipier;
    }
    g.set("fill", "black").set("stroke", "none")
}

fn draw_segment_lines(
    center: Position,
    segment_width: f64,
    segment_width_multipier: f64,
    num_segments: u32,
) -> impl Node {
    let data = draw_segment_lines_inner(
        Data::new(),
        center,
        PI / 2.0,
        0,
        segment_width,
        segment_width_multipier,
        num_segments,
    )
    .close();
    let path = Path::new().set("d", data);
    path
}

fn draw_segment_lines_inner(
    segment_lines: Data,
    pos: Position,
    angle: f64,
    depth: u32,
    segment_width: f64,
    segment_width_multipier: f64,
    num_segments: u32,
) -> Data {
    if depth == num_segments {
        return segment_lines;
    }
    let distance_from_center = depth as f64 * segment_width * segment_width_multipier;
    let length = (num_segments - depth) as f64 * segment_width * segment_width_multipier;
    let angle_modifier = PI / (2 as f64).powf((depth + 2) as f64);

    let from_pos = pos
        + Position::new(
            distance_from_center * angle.cos(),
            distance_from_center * angle.sin(),
            pos.height,
        );
    let to_pos = from_pos + Position::new(length * angle.cos(), length * angle.sin(), pos.height);

    let mut new_data = segment_lines.move_to(from_pos).line_to(to_pos);

    new_data = draw_segment_lines_inner(
        new_data,
        pos,
        angle - angle_modifier,
        depth + 1,
        segment_width * segment_width_multipier,
        segment_width_multipier,
        num_segments,
    );
    new_data = draw_segment_lines_inner(
        new_data,
        pos,
        angle + angle_modifier,
        depth + 1,
        segment_width * segment_width_multipier,
        segment_width_multipier,
        num_segments,
    );
    new_data
}
