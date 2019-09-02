mod circular_binary_tree;
mod position;

use svg::Document;

use circular_binary_tree::*;
use docopt::Docopt;
use position::Position;
use serde_derive::Deserialize;

const USAGE: &'static str = "
Program to generate a circular binary tree (e.g. for family trees) as an svg.

Usage:
  circular-family-tree FILE RADIUS DEPTH [--inner-sector=<ir>, --margin=<m>, --x-offset=<xo>, --y-offset=<yo>, --disable-numbers, --line-width=<lw>, --font-size=<fs>, --font-family=<ff>]
  circular-family-tree (-h | --help)
  circular-family-tree --version

Options:
  -h --help               Show this screen.
  --version               Show version.
  --inner-sector=<ir>     Draw an inner ring for each sector at a percentage of the sector width [default: 0.0].
  --margin=<m>           Margin between tree and edge [default: 0.0].
  --x-offset=<xo>         x-offset of the numbers [default: 0.0].
  --y-offset=<yo>         y-offset of the numbers [default: 1.0].
  --disable-numbers=<dn>  Disable printing of numbers.  
  --line-width=<lw>       Line width [default: 1.0].  
  --font-size=<fs>        Font size [default: 10].  
  --font-family=<ff>      Font family [default: Arial].  
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_FILE: String,
    arg_RADIUS: f64,
    arg_DEPTH: u32,
    flag_margin: f64,
    flag_inner_sector: f64,
    flag_x_offset: f64,
    flag_y_offset: f64,
    flag_disable_numbers: bool,
    flag_line_width: f64,
    flag_font_size: u64,
    flag_font_family: String,
}

fn create_content(num_segments: u32) -> Vec<String> {
    let mut content = Vec::with_capacity(((1 - 2isize.pow(num_segments + 1)) / -1 - 1) as usize);
    for depth in 1..num_segments + 1 {
        let max = 2u64.pow(depth);
        for i in 0..max {
            content.push(format!("{}", 2u64.pow(depth) + (i as u64) - 1));
        }
    }
    content
}

fn main() {
    // let args: Args = Docopt::new(USAGE)
    //     .and_then(|d| d.deserialize())
    //     .unwrap_or_else(|e| e.exit());
    // let tree = CircularBinaryTree::new(
    //     args.arg_RADIUS * 2.0,
    //     args.arg_RADIUS,
    //     args.flag_margin,
    //     args.arg_DEPTH,
    //     args.flag_inner_sector,
    //     args.flag_x_offset,
    //     args.flag_y_offset,
    //     !args.flag_disable_numbers,
    //     args.flag_line_width,
    //     args.flag_font_size,
    //     args.flag_font_family,
    // );

    let height = 7000.0;
    let width = height * (2.0 as f64).sqrt();
    let radius = width/2.0;

    // let height = 2000.0;
    // let width = 2000.0;
    // let radius = 1000.0;
    let center = Position::new(width/2.0,0.0, height);
    let start_angle = 0.0;
    let end_angle = 180.0;

    //let left = true;

    // let (center, start_angle, end_angle) = if left {
    //     (Position::new(width, 0.0, height), 90.0, 180.0)
    // } else {
    //     (Position::new(0.0, 0.0, height), 0.0, 90.0)
    // };

    let num_segments = 9;
    let line_width = 1.0;

    let s = CircularBinaryTree::new(
        center,
        radius / (num_segments as f64),
        1.0,
        num_segments,
        line_width,
        start_angle,
        end_angle,
        -10.0,
        80.0,
        0.125,
        "Times New Roman".to_owned(),
        60.0,
        0.1,
    );
    let content = create_content(num_segments);
    //println!("{:?}", content);

    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .add(s.draw(&content));
    //svg::save(args.arg_FILE, &document).unwrap();
    svg::save("baum.svg", &document).unwrap();
}
