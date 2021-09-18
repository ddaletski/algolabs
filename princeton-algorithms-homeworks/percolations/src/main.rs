use clap::{App, AppSettings, Clap};
use gif::{Encoder, Frame};
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::borrow::Cow;
use std::fs::File;

use percolations::Percolation;

#[derive(Clap)]
#[clap(version = "1.0", author = "Denis Dalecki")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Visualize(Visualize),
}

#[derive(Clap)]
struct Visualize {
    #[clap(short, long)]
    size: u8,
}

fn grid_points_shuffled(rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();

    let mut all_points = (0..rows).cartesian_product(0..cols).collect_vec();
    all_points.shuffle(&mut rng);

    all_points
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Visualize(v) => {
            let grid_size = v.size as usize;
            if grid_size == 0 || grid_size > 100 {
                println!("error: size should be in range[1, 100], got {}", grid_size);
                return;
            }

            let color_map = &[0, 0, 0, 0xF0, 0xF0, 0xF0, 0, 0, 0xFF];

            let mut image = File::create("target/vis.gif").unwrap();
            let mut encoder =
                Encoder::new(&mut image, grid_size as u16, grid_size as u16, color_map).unwrap();

            let mut perc = Percolation::new(grid_size, grid_size);

            for (y, x) in grid_points_shuffled(grid_size, grid_size).iter().progress() {
                perc.open(*y, *x);

                let grid = perc.grid();

                let mut frame = Frame::default();
                frame.width = grid_size as u16;
                frame.height = grid_size as u16;
                frame.buffer = Cow::Borrowed(grid.as_slice().unwrap());
                encoder.write_frame(&frame).unwrap();

                if perc.percolates() {
                    break;
                }
            }
        }
    }
}