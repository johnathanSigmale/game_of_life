mod species;
mod ecosystem;
mod mutations;

use ecosystem::Ecosystem;
use game_of_life::*;
use eframe::egui;
use egui::{color_picker::Alpha, Vec2, Window};
use core::f32;
use std::{collections::HashSet, time::{Duration, Instant}};

struct MyApp {
    ecosystem: Ecosystem,
    history: Vec<Vec<HashSet<(isize, isize)>>>,
    grid_size: (isize, isize),
    last_update: Instant,
    update_interval: Duration,
    playing : bool,
    infinite: bool,
    grid_offset: Vec2,
    zoom: f32,
    drag_speed: f32,
    focus: usize,
    hide_gui: bool,
}

impl MyApp {
    fn step(&mut self) {
        if self.ecosystem.species.len() == 1 {
            let species = &mut self.ecosystem.species[0];
            species.live_cells = step_infinite(&species.live_cells, &species.neighbours_survival,
                &species.neighbours_spawn);
        }
        else {
            self.ecosystem.step_infinite();
        }
    }

    // fn back(&mut self) {
    //     if !self.history.is_empty() {
    //         self.black_cells = self.history.pop().unwrap();
    //     }
    // }

    fn handle_click_cell(&mut self, cell: (isize, isize)) {
        for species in &mut self.ecosystem.species {
            if species.live_cells.contains(&cell) {
                species.live_cells.remove(&cell);
                return;
            }
        }
        self.ecosystem.species.get_mut(self.focus as usize).unwrap().live_cells.insert(cell);
    }

    fn clear(&mut self) {
        for species in &mut self.ecosystem.species {
            species.live_cells.clear();
        }
        self.history.clear();
    }

    fn randomize_cells(&mut self) {
        self.ecosystem.randomize_cells(self.grid_size);
    }
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ecosystem: Ecosystem::default(),
            history: vec![],
            grid_size: (60, 30),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(1),
            playing: false,
            infinite: true,
            grid_offset: Vec2::ZERO,
            zoom: 0.01,
            drag_speed: 1.,
            focus: 0,
            hide_gui: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Step interval
        if (self.last_update.elapsed() >= self.update_interval) && self.playing {
            self.step();
            self.last_update = Instant::now();
            ctx.request_repaint(); // Ensure the UI updates
        }

        ctx.request_repaint_after(self.update_interval); // Repaints every interval

    { // input handling
        if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
            self.playing = !self.playing;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::H)) {
            self.hide_gui = !self.hide_gui;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            self.step();
        }

        // if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
        //     self.back();
        // }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
            self.update_interval += Duration::from_millis(10);
        }

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
            self.update_interval = self.update_interval.checked_sub(Duration::from_millis(10)).unwrap_or(Duration::from_millis(0));
        }
    }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle zoom input
            let zoom = ctx.input(|i| i.zoom_delta());
            if zoom.is_normal() && zoom != 1. {
                self.zoom *= zoom;
                self.zoom = self.zoom.clamp(f32::MIN_POSITIVE, 2.);
            }

            // Handle dragging
            if ctx.input(|i| i.pointer.any_down()) {
                if ctx.input(|i| i.pointer.is_decidedly_dragging() && !i.pointer.primary_down()) {
                    self.grid_offset += ctx.input(|i| i.pointer.delta()) * self.drag_speed ;
                }
            }
            // Calculate cell size
            let cell_size = ui.available_width().min(ui.available_height()) * self.zoom;

            let (rows, cols) = (ui.available_size() / cell_size).into();
            
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click(),
            );

            // Draw white background
            painter.rect_filled(response.rect, 0.0, egui::Color32::WHITE);

            // Draw grid lines
            for i in 0..=rows as isize + 1 {
                let x = ((i as f32) * cell_size) + self.grid_offset.x.rem_euclid(cell_size);
                painter.line_segment(
                    [egui::pos2(x, response.rect.min.y),
                        egui::pos2(x, response.rect.max.y)],
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                );
            }

            for i in 0..=cols as isize + 1 {
                let y = ((i as f32 ) * cell_size) + self.grid_offset.y.rem_euclid(cell_size);
                painter.line_segment(
                    [egui::pos2(response.rect.min.x, y),
                        egui::pos2(response.rect.max.x, y)],
                    egui::Stroke::new(1.0, egui::Color32::GRAY),
                );
            }

            // Draw black cells
            for species in &self.ecosystem.species {    
                for &(x, y) in &species.live_cells {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2((x as f32 * cell_size) + self.grid_offset.x, (y as f32 * cell_size) + self.grid_offset.y),
                        egui::vec2(cell_size, cell_size),
                    );
                    painter.rect_filled(rect, 0.0, species.color);
                }
            }

            // Handle clicks
            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let x = (pos.x - self.grid_offset.x) / cell_size;
                    let x = x as isize - if x > 0. {0} else {1} ;
                    let y = (pos.y - self.grid_offset.y) / cell_size;
                    let y = y as isize - if y > 0. {0} else {1} ;
                    self.handle_click_cell((x, y))
                }
            }
        });

        if self.hide_gui {return}

        // General menu
        Window::new("Configuration")
            .resizable(true)
            .collapsible(true)
            .show(ctx, |ui| {
                if ui.button("Randomize Cells").clicked() {
                    self.randomize_cells();
                }

                if ui.button("Clear").clicked() {
                    self.clear();
                }

                if ui.button("Center").clicked() {
                    self.grid_offset = Vec2::ZERO;
                    self.zoom = 0.01
                }

                if ui.button("Add Species").clicked() {
                    self.ecosystem.add_species();
                }
    
                if ui.button(if self.playing {"Stop"} else {"Play"}).clicked() {
                    self.playing = !self.playing;
                }

                ui.horizontal(|ui| {
                    // if ui.button("< - -").clicked() {
                    //     self.back();
                    // }
                    if ui.button("- - >").clicked() {
                        self.step();
                    }
                });
    
                ui.horizontal(|ui| {
                    ui.label("Grid Size: ");
                    if ui.add(egui::DragValue::new(&mut self.grid_size.0).range(0..=isize::MAX).speed(1)).changed() {
                        if !self.infinite {
                            for species in &mut self.ecosystem.species {
                                species.live_cells.retain(|&(x, _)| x < self.grid_size.0);
                            }
                        }
                    }
                    ui.label("x");
                    if ui.add(egui::DragValue::new(&mut self.grid_size.1).range(0..=isize::MAX).speed(1)).changed() {
                        if !self.infinite {
                            for species in &mut self.ecosystem.species {
                                species.live_cells.retain(|&(_, y)| y < self.grid_size.1);
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Step Interval: ");
                    let mut update_interval_f = self.update_interval.as_secs_f64();
                    if ui.add(egui::DragValue::new(&mut update_interval_f).range(0..=isize::MAX).speed(0.01).suffix(" secs")).changed() {
                        self.update_interval = Duration::from_secs_f64(update_interval_f);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Drag Speed: ");
                    ui.add(egui::DragValue::new(&mut self.drag_speed).range(0..=isize::MAX).speed(0.1));
                })
            });


        // Species menu
        for species in &mut self.ecosystem.species {
            Window::new(&species.id.to_string())
                .resizable(true)
                .collapsible(true)
                .show(ctx, |ui| {

                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    ui.text_edit_singleline(&mut species.name);
                });

                ui.label("Cell Survival: ");
                ui.horizontal(|ui| {
                    ui.label("Minimum Neighbors: ");
                    ui.add(egui::DragValue::new(&mut species.neighbours_survival.0).range(0..=species.neighbours_survival.1).speed(0.1));
                    ui.label("Maximum Neighbors: ");
                    ui.add(egui::DragValue::new(&mut species.neighbours_survival.1).range(species.neighbours_survival.0..=8).speed(0.1));
                });

                ui.label("Cell Spawn: ");
                ui.horizontal(|ui| {
                    ui.label("Minimum Neighbors: ");
                    ui.add(egui::DragValue::new(&mut species.neighbours_spawn.0).range(0..=species.neighbours_spawn.1).speed(0.1));
                    ui.label("Maximum Neighbors: ");
                    ui.add(egui::DragValue::new(&mut species.neighbours_spawn.1).range(species.neighbours_spawn.0..=8).speed(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Change Color:");
                    egui::color_picker::color_edit_button_srgba(ui, &mut species.color, Alpha::OnlyBlend);
                });

                if self.focus == species.id {
                    ui.label("Focused");
                }
                else if ui.button("Focus").clicked() {
                    self.focus = species.id;
                }

                ui.horizontal(|ui| {
                    ui.label("Random Fill Percentage:");
                    let mut sum : f32 = self.ecosystem.rand_fill_values.iter().sum();
                    let rand_fill_value = self.ecosystem.rand_fill_values.get_mut(species.id).unwrap();
                    sum -= *rand_fill_value;
                    ui.add(egui::DragValue::new(rand_fill_value).range((0.)..=(1. - sum)).speed(0.01));
                });

                ui.horizontal(|ui| {
                    ui.label("Change Priority:");
                    ui.add(egui::DragValue::new(&mut species.priority).range(isize::MIN..=isize::MAX).speed(0.01));
                });

                ui.label("Effects on : ");
                for (receiver_id, effect) in self.ecosystem.dynamics.get_mut(species.id).unwrap().iter_mut().enumerate() {
                    ui.label(receiver_id.to_string());
                    ui.add(egui::DragValue::new(effect).range(isize::MIN..=isize::MAX).speed(0.1));
                }
            });
        }
    }
}

// #[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}