#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use a_star::common::{Point, Size};
use a_star::gui::animation::Animation;
use a_star::gui::utils::maze_image;
use a_star::maze::Maze;
use a_star::maze::SparsePointSet;
use a_star::solvers::{AStarSolver, Solver};

use a_star::traits::solver::SearchState;
use eframe::egui;
use eframe::epaint::mutex::RwLock;
use rand::Rng;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    solver: Arc<Mutex<AStarSolver>>,
    progress: Arc<RwLock<u32>>,
    animation: Option<Animation>,
    solver_thread: Option<std::thread::JoinHandle<()>>,
    update_thread: Option<std::thread::JoinHandle<()>>,
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
        let solver = AStarSolver::new(maze, source, destination);

        Self {
            solver: Arc::new(Mutex::new(solver)),
            progress: Arc::new(RwLock::new(0)),
            animation: None,
            solver_thread: None,
            update_thread: None,
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = "WinDow";

        egui::CentralPanel::default()
            // .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.heading(title);

                let mut solver = self.solver.lock().unwrap();

                if ui.button("restart").clicked() {
                    solver.restart();
                }

                let maze_img = maze_image(&*solver);

                let animation = if self.animation.is_none() {
                    self.animation = Some(Animation::new("maze", ctx, &maze_img));
                    self.animation.as_ref().unwrap()
                } else {
                    let animation = self.animation.as_mut().unwrap();
                    animation.update(&maze_img);
                    animation
                };

                let egui_img =
                    egui::Image::new(animation.texture(), animation.texture().size_vec2());

                ui.add(egui_img);

                let checked = *self.progress.read();
                let total = animation.size().area();
                ui.add(egui::ProgressBar::new(checked as f32 / total as f32));

                // let pr_clone = self.progress.cl

                let current_progress = self.progress.clone();
                if self.solver_thread.is_none() {
                    let mutex = self.solver.clone();
                    self.solver_thread = Some(std::thread::spawn(move || loop {
                        std::thread::sleep(std::time::Duration::from_millis(
                            1000 / a_star::SOLVER_STEPS_PER_SECOND,
                        ));

                        let mut solver = mutex.lock().unwrap();

                        for _ in 0..10 {
                            let solver_state = (&mut *solver).next();

                            if let SearchState::Progress(progress) = solver_state {
                                *current_progress.write() = progress.checked;
                            } else {
                                break;
                            }
                        }
                    }));
                }

                if self.update_thread.is_none() {
                    let ctx_clone = ctx.clone();
                    self.update_thread = Some(std::thread::spawn(move || loop {
                        std::thread::sleep(std::time::Duration::from_millis(30));
                        ctx_clone.request_repaint();
                    }));
                }
            });
    }
}
