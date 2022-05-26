#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use a_star::common::Point;
use a_star::gui::animation::Animation;
use a_star::gui::utils::maze_image;
use a_star::maze::Maze;
use a_star::maze::SparsePointSet;
use a_star::solvers::{AStarSolver, BFSSolver, DFSSolver, GreedySolver, Solver};

use a_star::traits::solver::SearchState;
use eframe::egui;
use image::RgbaImage;
use rand::Rng;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
    BFS,
    DFS,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Stop,
    Restart,
}

impl SolverAlgorithm {
    fn name(&self) -> &str {
        match self {
            SolverAlgorithm::Greedy => "Greedy",
            SolverAlgorithm::AStar => "A*",
            SolverAlgorithm::BFS => "BFS",
            SolverAlgorithm::DFS => "DFS",
        }
    }

    fn create(&self, maze: Maze) -> Box<dyn Solver + Send + Sync> {
        match self {
            SolverAlgorithm::AStar => {
                let solver = AStarSolver::new(maze);
                Box::new(solver)
            }
            SolverAlgorithm::Greedy => {
                let solver = GreedySolver::new(maze);
                Box::new(solver)
            }
            SolverAlgorithm::BFS => {
                let solver = BFSSolver::new(maze);
                Box::new(solver)
            }
            SolverAlgorithm::DFS => {
                let solver = DFSSolver::new(maze);
                Box::new(solver)
            }
        }
    }
}

struct MyApp {
    maze: Maze,
    algorithm: SolverAlgorithm,
    animation: Option<Animation>,
    solver_thread: Option<std::thread::JoinHandle<()>>,
    update_thread: Option<std::thread::JoinHandle<()>>,
    cmd_sender: Option<crossbeam_channel::Sender<Command>>,
    img_receiver: Option<crossbeam_channel::Receiver<RgbaImage>>,
    last_update: std::time::Instant,
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

        let mut app = Self {
            maze: maze,
            algorithm: algorithm,
            animation: None,
            solver_thread: None,
            update_thread: None,
            img_receiver: None,
            cmd_sender: None,
            last_update: std::time::Instant::now(),
        };

        app.set_alg(algorithm);

        app
    }
}

impl MyApp {
    fn set_alg(&mut self, algorithm: SolverAlgorithm) {
        // stop the old thread if it's running
        if let Some(old_thread) = self.solver_thread.take() {
            let sender = self.cmd_sender.as_ref().unwrap();
            sender.send(Command::Stop).unwrap();
            old_thread.join().unwrap();
        }

        let maze = self.maze.clone();
        self.algorithm = algorithm;

        let (img_sender, img_receiver) = crossbeam_channel::bounded(3);
        let (cmd_sender, cmd_receiver) = crossbeam_channel::bounded(0);

        self.img_receiver = Some(img_receiver);
        self.cmd_sender = Some(cmd_sender);

        self.solver_thread = Some(std::thread::spawn(move || {
            let mut solver = algorithm.create(maze);

            let mut reached_end = false;
            loop {
                match cmd_receiver.try_recv() {
                    Ok(Command::Restart) => {
                        solver.restart();
                    }
                    Ok(Command::Stop) => {
                        return;
                    }
                    Err(_) => {
                        if reached_end {
                            continue;
                        }

                        let solver_state = {
                            (0..a_star::SOLVER_STEPS_PER_TICK)
                                .map(|_| solver.next())
                                .last()
                                .unwrap()
                        };

                        let maze_img = maze_image(&*solver);
                        img_sender.send(maze_img).unwrap();

                        match solver_state {
                            SearchState::Found | SearchState::NotFound => {
                                reached_end = true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }));
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let title = "Maze Solver";

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(title);

            let last_alg = self.algorithm;
            let mut selected_alg = last_alg;

            egui::ComboBox::from_label("algorithm")
                .selected_text(format!("{:?}", selected_alg.name()))
                .show_ui(ui, |ui| {
                    let mut add_alg = |alg| {
                        ui.selectable_value(&mut selected_alg, alg, alg.name());
                    };
                    add_alg(SolverAlgorithm::AStar);
                    add_alg(SolverAlgorithm::Greedy);
                    add_alg(SolverAlgorithm::BFS);
                    add_alg(SolverAlgorithm::DFS);
                });

            if selected_alg != last_alg {
                self.set_alg(selected_alg);
            }

            if ui.button("restart").clicked() {
                if let Some(sender) = self.cmd_sender.as_ref() {
                    sender.send(Command::Restart).unwrap();
                }
            }

            let current_time = std::time::Instant::now();
            let update_threshold = 1.0 / a_star::SOLVER_TICKS_PER_SECOND as f32;
            if current_time.duration_since(self.last_update).as_secs_f32() > update_threshold {
                self.last_update = current_time;

                let receiver = self.img_receiver.as_ref().unwrap();
                if let Ok(maze_img) = receiver.try_recv() {
                    if self.animation.is_none() {
                        self.animation = Some(Animation::new("maze", ctx, &maze_img));
                    } else {
                        let animation = self.animation.as_mut().unwrap();
                        animation.update(&maze_img);
                    };
                }
            }

            if let Some(animation) = &self.animation {
                let egui_img =
                    egui::Image::new(animation.texture(), animation.texture().size_vec2());
                ui.add(egui_img);
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
