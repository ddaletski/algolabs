#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use a_star::common::{Point, Size};
use a_star::gui::animation::Animation;
use a_star::gui::utils::maze_image;
use a_star::maze::Maze;
use a_star::maze::SparsePointSet;
use a_star::solvers::{AStarSolver, Solver};

use eframe::egui;
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
    animation: Option<Animation>,
}

impl Default for MyApp {
    fn default() -> Self {
        let size = a_star::MAZE_SIZE;

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
            animation: None,
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = "WinDow";

        // let stroke_color = ctx.style().visuals.text_color();

        // Height of the visualization area
        // let canvas_height = 480.0;

        egui::CentralPanel::default()
            // .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.heading(title);

                if ui.button("restart").clicked() {
                    self.solver.restart();
                }

                let start = std::time::Instant::now();
                for _ in 0..a_star::STEPS_PER_FRAME {
                    let _solver_state = self.solver.next();
                }
                let duration = start.elapsed();
                println!("update: {}", duration.as_millis());

                let maze_img = maze_image(&self.solver);

                let start = std::time::Instant::now();
                let animation = if self.animation.is_none() {
                    self.animation = Some(Animation::new("maze", ctx, &maze_img));
                    self.animation.as_ref().unwrap()
                } else {
                    let animation = self.animation.as_mut().unwrap();
                    animation.update(&maze_img);
                    animation
                };
                let duration = start.elapsed();
                println!("animation: {}", duration.as_millis());

                let start = std::time::Instant::now();
                let egui_img =
                    egui::Image::new(animation.texture(), animation.texture().size_vec2());
                let duration = start.elapsed();
                println!("add img: {}", duration.as_millis());

                ui.add(egui_img);

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
