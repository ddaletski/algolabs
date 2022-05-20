#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use a_star::common::{Point, Rect, Size};
use a_star::maze::Maze;
use a_star::maze::{PointSet, SparsePointSet};
use a_star::{AStarSolver, Solver};

use eframe::egui::{self, Image};
use eframe::epaint::ColorImage;
use image::buffer::ConvertBuffer;
use image::{GrayImage, RgbaImage};
use rand::Rng;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    solver: AStarSolver,
}

impl Default for MyApp {
    fn default() -> Self {
        let size = Size {
            width: 20,
            height: 20,
        };

        let source = Point { x: 0, y: 0 };
        let destination = Point {
            x: (size.width - 1) as i32,
            y: (size.height - 1) as i32,
        };

        let mut rng = rand::thread_rng();
        let walls = (0..size.width * size.height / 10)
            .map(|_| {
                let x = rng.gen_range(0..size.width) as i32;
                let y = rng.gen_range(0..size.height) as i32;

                Point { x, y }
            })
            .filter(|&p| p != source && p != destination);

        let points_set = Box::new(SparsePointSet::new(walls));

        let maze = Maze::new(size, points_set);

        Self {
            solver: AStarSolver::new(maze, source, destination),
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = "WinDow";

        let stroke_color = ctx.style().visuals.text_color();

        // Height of the visualization area
        let canvas_height = 480.0;

        egui::CentralPanel::default()
            // .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.heading(title);

                if ui.button("restart").clicked() {
                    self.solver.restart();
                }

                let solver_state = self.solver.next();

                let solver_grid = self.solver.inspect();
                let size = self.solver.maze().size;
                let gray_img = GrayImage::from_raw(size.width, size.height, solver_grid).unwrap();

                let mut palette: Vec<(u8, u8, u8)> = Vec::new();
                palette.resize(256, (0, 0, 0));
                palette[0] = (0, 0, 0); // free is black
                palette[1] = (255, 0, 0); // wall is red
                palette[2] = (0, 255, 0); // checked is green
                palette[3] = (255, 255, 0); // queued is yellow
                palette[4] = (0, 255, 255); // source is cyan
                palette[5] = (255, 0, 255); // destination is purple

                let mut rgba_img: RgbaImage = gray_img.expand_palette(&palette[..], None);

                rgba_img = image::imageops::resize(
                    &rgba_img,
                    size.width * 8,
                    size.height * 8,
                    image::imageops::FilterType::Nearest,
                );

                let img = ColorImage::from_rgba_unmultiplied(
                    [rgba_img.height() as usize, rgba_img.width() as usize],
                    &rgba_img.as_raw().as_slice(),
                );

                let texture = ctx.load_texture("vis-area", img);
                ui.add(egui::Image::new(&texture, texture.size_vec2()));

                // ui.add(egui::Slider::new(&mut self.progress, 0..=100).text("progress"));

                // let desired_size = egui::vec2(canvas_height, canvas_height);
                // let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

                // if response.hovered() {}

                // let painter = ui.painter();

                // // Paint the frame:
                // painter.rect(
                //     rect,
                //     5.0,
                //     ctx.style().visuals.window_fill(),
                //     egui::Stroke::new(1.0, stroke_color),
                // );

                // let stroke = egui::Stroke::new(1.0, stroke_color);
                // for _ in 0..=self.progress {
                //     let progress = self.progress as f32 / 100.0;
                //     let shift = egui::vec2(progress * rect.width(), 0.0);
                //     painter.line_segment(
                //         [rect.left_top() + shift, rect.left_bottom() + shift],
                //         stroke,
                //     )
                // }
            });
    }
}
