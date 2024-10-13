use clap::Parser;
use gif::{Encoder, Frame};
use indicatif::ProgressIterator;
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::borrow::Cow;
use std::fs::File;

use percolations::Percolation;

/// Grid percolation simulation
#[derive(clap::Parser)]
#[clap(version = "1.0", author = "Denis Dalecki (daletskidenis@gmail.com)")]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    /// visualize grid percolation
    Visualize(Visualize),

    /// calculate percolation thresholds for different grid sizes
    Threshold(Threshold),
}

#[derive(clap::Args)]
struct Visualize {
    /// percolation grid size
    #[clap(short, long)]
    size: u8,

    /// output gif path
    #[clap(short, long)]
    out: String,
}

#[derive(clap::Args)]
struct Threshold {
    /// percolation grid height
    #[clap(short, long)]
    rows: u16,

    /// percolation grid width
    #[clap(short, long)]
    cols: u16,

    /// sample size
    #[clap(long)]
    samples: u8,
}

fn grid_points_shuffled(rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut rng = rand::thread_rng();

    let mut all_points = (0..rows).cartesian_product(0..cols).collect_vec();
    all_points.shuffle(&mut rng);

    all_points
}

fn visualize(grid_size: usize, out_file: &str) {
    let color_map = &[0, 0, 0, 0xF0, 0xF0, 0xF0, 0, 0, 0xFF];

    let mut image = File::create(out_file).unwrap();
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

fn calc_percolation_threshold(rows: usize, cols: usize) -> f64 {
    let mut rng = rand::thread_rng();

    let mut perc = Percolation::new(rows, cols);
    let mut all_points = (0..rows).cartesian_product(0..cols).collect_vec();
    all_points.shuffle(&mut rng);

    for (y, x) in all_points {
        perc.open(y, x);

        if perc.percolates() {
            break;
        }
    }

    perc.count_open() as f64 / (rows * cols) as f64
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Visualize(v) => {
            let grid_size = v.size as usize;
            if grid_size == 0 || grid_size > 100 {
                println!("error: size should be in range[1, 100], got {}", grid_size);
                return;
            }

            visualize(grid_size, &v.out);
        }

        SubCommand::Threshold(t) => {
            if t.samples == 0 {
                println!("error: sample should be > 0");
                return;
            }

            let thresholds = (0..t.samples)
                .map(|_| calc_percolation_threshold(t.rows as usize, t.cols as usize))
                .progress()
                .collect_vec();

            let n = thresholds.len() as f64;

            let mean = thresholds.iter().sum::<f64>() / n;
            let stdev = (thresholds
                .iter()
                .map(|x| (x - mean) * (x - mean))
                .sum::<f64>()
                / (n - 1.0))
                .sqrt();
            let ci_range = 1.96 * stdev / n.sqrt();

            println!(
                "threshold for {}x{} grid = {{mean: {:.4}, stdev: {:.5}, ci(0.95): [{:.4}, {:.4}]}}",
                t.rows,
                t.cols,
                mean,
                stdev,
                mean - ci_range,
                mean + ci_range
            );
        }
    }
}
