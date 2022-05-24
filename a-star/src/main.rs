#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use a_star::common::Point;
use a_star::gui::animation::Animation;
use a_star::gui::utils::maze_image;
use a_star::maze::Maze;
use a_star::maze::SparsePointSet;
use a_star::solvers::{AStarSolver, GreedySolver, Solver};

use a_star::traits::solver::SearchState;
use eframe::egui;
use rand::Rng;
use std::sync::{Arc, RwLock};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SolverAlgorithm {
    Greedy,
    AStar,
}

impl SolverAlgorithm {
    fn name(&self) -> &str {
        match self {
            SolverAlgorithm::Greedy => "Greedy",
            SolverAlgorithm::AStar => "A*",
        }
    }
    fn create(&self, maze: Maze) -> Arc<RwLock<dyn Solver + Send + Sync>> {
        match self {
            SolverAlgorithm::AStar => {
                let solver = AStarSolver::new(maze);
                Arc::new(RwLock::new(solver))
            }
            SolverAlgorithm::Greedy => {
                let solver = GreedySolver::new(maze);
                Arc::new(RwLock::new(solver))
            }
        }
    }
}

struct MyApp {
    algorithm: SolverAlgorithm,
    solver: Arc<RwLock<dyn Solver + Send + Sync>>,
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

        let points_set = SparsePointSet::new(walls);

        let maze = Maze::new(size, points_set, source, destination);

        let algorithm = SolverAlgorithm::Greedy;

        Self {
            algorithm: algorithm,
            solver: algorithm.create(maze),
            progress: Arc::new(RwLock::new(0)),
            animation: None,
            solver_thread: None,
            update_thread: None,
        }
    }
}

impl MyApp {
    fn set_alg(&mut self, algorithm: SolverAlgorithm) {
        let maze = self.solver.read().unwrap().maze().clone();

        self.solver = algorithm.create(maze);
        self.algorithm = algorithm;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = "WinDow";

        egui::CentralPanel::default()
            // .frame(egui::Frame::none())
            .show(ctx, |ui| {
                ui.heading(title);

                {
                    let last_alg = self.algorithm;
                    let mut selected_alg = self.algorithm;

                    egui::ComboBox::from_label("Select one!")
                        .selected_text(format!("{:?}", selected_alg.name()))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected_alg, SolverAlgorithm::AStar, "A*");
                            ui.selectable_value(
                                &mut selected_alg,
                                SolverAlgorithm::Greedy,
                                "Greedy",
                            );
                        });

                    if selected_alg != last_alg {
                        self.set_alg(selected_alg);
                    }
                }

                if ui.button("restart").clicked() {
                    let mut solver = self.solver.write().unwrap();
                    solver.restart();
                }

                let solver = self.solver.read().unwrap();
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

                let checked = *self.progress.read().unwrap();
                let total = animation.size().area();
                ui.add(egui::ProgressBar::new(checked as f32 / total as f32));

                // let pr_clone = self.progress.cl

                let current_progress = self.progress.clone();
                if self.solver_thread.is_none() {
                    let solver_guard = self.solver.clone();
                    self.solver_thread = Some(std::thread::spawn(move || loop {
                        // TODO: stop and join thread when other algorithm is selected
                        // and then recreate the thread
                        std::thread::sleep(std::time::Duration::from_millis(
                            1000 / a_star::SOLVER_TICKS_PER_SECOND,
                        ));

                        let mut solver = solver_guard.write().unwrap();

                        for _ in 0..a_star::SOLVER_STEPS_PER_TICK {
                            let solver_state = (&mut *solver).next();

                            if let SearchState::Progress(progress) = solver_state {
                                *current_progress.write().unwrap() = progress.checked;
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
