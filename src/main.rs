mod circular_binary_tree;
mod position;

use circular_binary_tree::CircularBinaryTree;
use svg::Document;

use docopt::Docopt;
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

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    let tree = CircularBinaryTree::new(
        args.arg_RADIUS * 2.0,
        args.arg_RADIUS,
        args.flag_margin,
        args.arg_DEPTH,
        args.flag_inner_sector,
        args.flag_x_offset,
        args.flag_y_offset,
        !args.flag_disable_numbers,
        args.flag_line_width,
        args.flag_font_size,
        args.flag_font_family,
    );
    let document = Document::new()
        .set("viewBox", (0, 0, args.arg_RADIUS * 2.0, args.arg_RADIUS))
        .add(tree.draw());
    svg::save(args.arg_FILE, &document).unwrap();
}
