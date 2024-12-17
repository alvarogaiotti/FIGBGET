#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use chrono::prelude::*;
use eframe::egui::{self, Button};
use egui_extras::DatePickerButton;
use figbget::download_report;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Option::from(egui::vec2(400., 400.)),
        ..Default::default()
    };
    eframe::run_native(
        "Scarica Report Tornei",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

struct MyEguiApp {
    start: NaiveDate,
    end: NaiveDate,
    channel: (Sender<f32>, Receiver<f32>),
    percentuale: f32,
    available: AtomicBool,
    failure: bool,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let today = Utc::now();
        let (send, recv) = std::sync::mpsc::channel::<f32>();
        Self {
            start: NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap(),
            end: today.date_naive(),
            channel: (send, recv),
            percentuale: 0.0,
            available: AtomicBool::new(true),
            failure: false,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Seleziona date per il report");
            ui.label("Dal:");
            ui.add_space(2.0);
            if ui.add(DatePickerButton::new(&mut self.start)).clicked() {
                self.percentuale = 0.0;
            };
            ui.add_space(2.0);
            ui.label("Al:");
            ui.add_space(2.0);
            let rect = ui.available_rect_before_wrap();
            if ui
                .child_ui_with_id_source(
                    rect,
                    eframe::egui::Layout::left_to_right(eframe::egui::Align::Min),
                    "Second date",
                )
                .add(DatePickerButton::new(&mut self.end))
                .clicked()
            {
                self.percentuale = 0.0;
            };
            let start = self.start;
            let end = self.end;
            ui.add_space(20.0);
            if self.available.load(Ordering::Relaxed) {
                if ui.button("Download report").clicked() {
                    self.available.store(false, Ordering::Relaxed);
                    self.failure = false;
                    let new_sender = self.channel.0.clone();
                    std::thread::spawn(move || {
                        download_report(start, end, new_sender);
                    });
                }
            } else {
                ui.add_enabled(false, Button::new("Download report"));
            }
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
            match self.channel.1.try_recv() {
                Ok(t) => {
                    if (1.0 + t).abs() < 1.0e-6 {
                        let channel = std::sync::mpsc::channel::<f32>();
                        self.channel = channel;
                        self.failure = true;
                        self.available = true;
                    } else {
                        self.percentuale = t;
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    let channel = std::sync::mpsc::channel::<f32>();
                    self.channel = channel;
                    self.available = true;
                }
                Err(_e) => {}
            };
            if self.failure {
                ui.add(
                    eframe::egui::widgets::ProgressBar::new(self.percentuale)
                        .fill(eframe::egui::Color32::RED),
                );
            } else {
                ui.add(eframe::egui::widgets::ProgressBar::new(self.percentuale));
            }
            if (1.0 - self.percentuale).abs() < 1.0e-6 {
                ui.label(
                    eframe::egui::RichText::new("Completato").color(eframe::egui::Color32::GREEN),
                );
                self.available.store(true, Ordering::Relaxed);
                self.percentuale = 0.0;
            }
            if self.failure {
                ui.label(
                    eframe::egui::RichText::new("Errore: controlla la tua connessione")
                        .color(eframe::egui::Color32::RED),
                );
            }
        });
    }
}
