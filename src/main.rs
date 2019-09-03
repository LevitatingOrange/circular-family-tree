mod circular_binary_tree;
mod position;

use svg::Document;

use circular_binary_tree::*;
use position::Position;
use resvg::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum DIN {
    A0_4,
    A0_2,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    A10,
}

impl DIN {
    fn size(&self) -> (u64, u64) {
        match self {
            Self::A0_4 => (1682, 2378),
            Self::A0_2 => (1189, 1682),
            Self::A0 => (841, 1189),
            Self::A1 => (594, 841),
            Self::A2 => (420, 594),
            Self::A3 => (297, 420),
            Self::A4 => (210, 297),
            Self::A5 => (148, 210),
            Self::A6 => (105, 148),
            Self::A7 => (74, 105),
            Self::A8 => (52, 74),
            Self::A9 => (37, 52),
            Self::A10 => (26, 37),
        }
    }
}

impl FromStr for DIN {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.to_uppercase() == "4A0" => Ok(Self::A0_4),
            s if s.to_uppercase() == "2A0" => Ok(Self::A0_2),
            s if s.to_uppercase() == "A0" => Ok(Self::A0),
            s if s.to_uppercase() == "A1" => Ok(Self::A1),
            s if s.to_uppercase() == "A2" => Ok(Self::A2),
            s if s.to_uppercase() == "A3" => Ok(Self::A3),
            s if s.to_uppercase() == "A4" => Ok(Self::A4),
            s if s.to_uppercase() == "A5" => Ok(Self::A5),
            s if s.to_uppercase() == "A6" => Ok(Self::A6),
            s if s.to_uppercase() == "A7" => Ok(Self::A7),
            s if s.to_uppercase() == "A8" => Ok(Self::A8),
            s if s.to_uppercase() == "A9" => Ok(Self::A9),
            s if s.to_uppercase() == "A10" => Ok(Self::A10),
            s => Err(format!("{} is not a valid DIN paper size", s)),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "circular-binary-tree",
    about = "A tool to generate a circular binary tree as an svg."
)]
struct Args {
    #[structopt(name = "FILE", parse(from_os_str))]
    output: PathBuf,
    #[structopt(name = "NUM_SEGMENTS")]
    num_segments: u32,
    /// not needed when using -d/--din
    #[structopt(name = "WIDTH", required_unless("din"))]
    width: Option<f64>,
    /// not needed when using -d/--din
    #[structopt(name = "HEIGHT", required_unless("din"))]
    height: Option<f64>,
    /// Use DIN (A[0-10]) format
    #[structopt(short, long)]
    din: Option<DIN>,
    #[structopt(short, long, default_value = "1.0")]
    line_width: f64,
    #[structopt(short, long, default_value = "0.0")]
    x_offset: f64,
    #[structopt(short, long, default_value = "0.0")]
    y_offset: f64,
    #[structopt(short = "m", long, default_value = "0.0")]
    y_offset_modifier: f64,
    #[structopt(short = "s", long, default_value = "16.0")]
    font_size: f64,
    #[structopt(short = "n", long, default_value = "0.0")]
    font_size_modifier: f64,
    #[structopt(short, long, default_value = "Times New Roman")]
    font_family: String,
    #[structopt(short="p", long, default_value = "0.0")]
    margin: f64,
}

fn create_content(num_segments: u32) -> Vec<String> {
    let mut content = Vec::with_capacity(((1 - 2isize.pow(num_segments + 1)) / -1 - 1) as usize);
    for depth in 1..=num_segments {
        let max = 2u64.pow(depth);
        for i in 0..max {
            content.push(format!("{}", 2u64.pow(depth) + (i as u64) - 1));
        }
    }
    content
}

fn main() {
    env_logger::init();
    let args = Args::from_args();

    let (width, height) = if let Some(d) = args.din {
        (d.size().1 as f64, d.size().0 as f64)
    } else {
        (args.width.unwrap(), args.height.unwrap())
    };

    let radius = width / 2.0 - args.margin;
    let center = Position::new(width / 2.0, args.margin, height);
    let start_angle = 0.0;
    let end_angle = 180.0;

    let s = CircularBinaryTree::new(
        center,
        radius / f64::from(args.num_segments),
        1.0,
        args.num_segments,
        args.line_width,
        start_angle,
        end_angle,
        args.x_offset * -1.0,   //-10.0,
        args.y_offset,          // 80.0,
        args.y_offset_modifier, //0.125,
        args.font_family,
        args.font_size,          //Times New Roman
        args.font_size_modifier, //0.1
    );
    let content = create_content(args.num_segments);

    let document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", format!("{}mm", width))
        .set("height", format!("{}mm", height))
        .add(s.draw(&content));

    svg::save(args.output.clone(), &document).unwrap();

    // let mut opt = resvg::Options::default();
    // opt.usvg.path = Some(args.output.clone().into());
    // let rtree = usvg::Tree::from_file(args.output.clone(), &opt.usvg).unwrap();

    // let backend = resvg::default_backend();
    // let mut img = backend.render_to_image(&rtree, &opt).unwrap();
    // let mut png_out = args.output.clone();
    // png_out.set_extension("png");
    // img.save_png(&png_out);
}
