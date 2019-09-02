use crate::position::*;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::node::element::Text as TextNode;
use svg::node::Text;

pub struct CircularBinaryTree {
    center: Position,
    segment_width: f64,
    segment_width_multipier: f64,
    num_segments: u32,
    line_width: f64,
    start_angle: f64,
    end_angle: f64,
    text_x_offset: f64,
    initial_text_y_offset: f64,
    text_y_modifier: f64,
    font_family: String,
    initial_font_size: f64,
    font_size_modifier: f64,
}

impl CircularBinaryTree {
    pub fn new(
        center: Position,
        segment_width: f64,
        segment_width_multipier: f64,
        num_segments: u32,
        line_width: f64,
        start_angle: f64,
        end_angle: f64,
        text_x_offset: f64,
        initial_text_y_offset: f64,
        text_y_modifier: f64,
        font_family: String,
        initial_font_size: f64,
        font_size_modifier: f64,
    ) -> Self {
        CircularBinaryTree {
            center,
            segment_width,
            segment_width_multipier,
            num_segments,
            line_width,
            start_angle: start_angle.to_radians(),
            end_angle: end_angle.to_radians(),
            text_x_offset,
            initial_text_y_offset,
            text_y_modifier,
            font_family,
            initial_font_size,
            font_size_modifier,
        }
    }
    pub fn draw(&self, content: &[String]) -> Group {
        let g = Group::new()
            .set("font-family", self.font_family.as_ref())
            .set("fill", "none")
            .set("stroke", "black")
            .set("style", "white-space: pre;")
            .set("stroke-width", self.line_width);
        g.add(self.draw_segments())
            .add(self.draw_sectors())
            .add(self.draw_content(content))
    }

    fn draw_segments(&self) -> Path {
        let path_data = self.draw_segments_inner(self.start_angle, self.end_angle, 0, Data::new());
        Path::new().set("d", path_data)
    }
    fn draw_segments_inner(
        &self,
        start_angle: f64,
        end_angle: f64,
        depth: u32,
        path_data: Data,
    ) -> Data {
        if depth >= self.num_segments {
            return path_data;
        }
        let distance_from_center =
            f64::from(depth) * self.segment_width * self.segment_width_multipier;
        let length = f64::from(self.num_segments - depth)
            * self.segment_width
            * self.segment_width_multipier;

        let angle = start_angle + (end_angle - start_angle) / 2.0;

        let from_pos = self.center
            + Position::new(
                distance_from_center * angle.cos(),
                distance_from_center * angle.sin(),
                self.center.height,
            );
        let to_pos = from_pos
            + Position::new(
                length * angle.cos(),
                length * angle.sin(),
                self.center.height,
            );

        let mut new_data = path_data.move_to(from_pos).line_to(to_pos);

        new_data = self.draw_segments_inner(start_angle, angle, depth + 1, new_data);
        new_data = self.draw_segments_inner(angle, end_angle, depth + 1, new_data);
        new_data
    }

    fn draw_sectors(&self) -> Path {
        let long_way = if (self.end_angle - self.start_angle).abs() > PI * 3.0 / 4.0 {
            1
        } else {
            0
        };

        let mut path_data = Data::new();
        for depth in 0..=self.num_segments {
            let distance_from_center =
                f64::from(depth) * self.segment_width * self.segment_width_multipier;
            let from_pos = self.center
                + Position::new(
                    distance_from_center * self.start_angle.cos(),
                    distance_from_center * self.start_angle.sin(),
                    self.center.height,
                );
            let to_pos = self.center
                + Position::new(
                    distance_from_center * self.end_angle.cos(),
                    distance_from_center * self.end_angle.sin(),
                    self.center.height,
                );

            path_data = path_data.move_to(from_pos).elliptical_arc_to((
                distance_from_center,
                distance_from_center,
                0,
                long_way,
                0,
                to_pos,
            ))
        }
        Path::new().set("d", path_data)
    }

    fn draw_content(&self, content: &[String]) -> Group {
        let mut g = Group::new().set("fill", "black").set("stroke", "none");

        assert_eq!(
            content.len() as isize,
            (1 - 2isize.pow(self.num_segments + 1)) / -1 - 1,
            "content does not have right size"
        );

        for depth in 1..=self.num_segments {
            let max = 2u64.pow(depth);
            for i in 0..max {
                let angle = self.start_angle
                    + i as f64 * ((self.end_angle - self.start_angle).abs() / (max as f64));

                let (angle, distance_from_center, anchor) = if angle.to_degrees() < 90.0 {
                    (
                        angle,
                        f64::from(depth) * self.segment_width * self.segment_width_multipier
                            + self.text_x_offset,
                        "end",
                    )
                } else {
                    (
                        self.end_angle
                            + (i + 1) as f64
                                * ((self.end_angle - self.start_angle).abs() / (max as f64)),
                        (f64::from(depth) * self.segment_width * self.segment_width_multipier)
                            * -1.0
                            - self.text_x_offset,
                        "start",
                    )
                };

                let rot_str = format!(
                    "rotate({},{},{})",
                    360.0 - angle.to_degrees(),
                    self.center.x(),
                    self.center.y()
                );

                g = g.add(
                    TextNode::new()
                        .set("text-anchor", anchor)
                        //.set("dominant-baseline", "text-top")
                        .set("x", self.center.x() + distance_from_center)
                        .set(
                            "y",
                            self.center.y()
                                - (self.initial_text_y_offset
                                    - (self.text_y_modifier * max as f64)),
                        )
                        .set("transform", rot_str)
                        .set(
                            "font-size",
                            self.initial_font_size - (self.font_size_modifier * max as f64),
                        )
                        .add(Text::new(&content[2usize.pow(depth) + (i as usize) - 2])),
                    //.add(Text::new(format!("{:width$}", 0, width = width))),
                )
            }
        }
        g
    }
}
