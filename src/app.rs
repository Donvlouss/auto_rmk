use std::sync::mpsc::Receiver;

use eframe::App;
use egui::Button;

use crate::{core::MKAction, record::Record};

#[derive(Debug)]
pub struct RmkApp {
    recorder: Record,
    is_recording: bool,
    actions: Vec<MKAction>,
    rx: Receiver<()>,
    focused: bool,
}

impl RmkApp {
    pub fn new(rx: Receiver<()>) -> Self {
        Self {
            recorder: Record::new(),
            is_recording: false,
            actions: vec![],
            rx,
            focused: true,
        }
    }
}

impl App for RmkApp {
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        _ctx.viewport(|ctx| {
            let input = &ctx.input;
            for e in input.events.iter() {
                match e {
                    egui::Event::WindowFocused(f) => {
                        self.focused = *f;
                    }
                    _ => {}
                }
            }
        });
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.rx.try_recv().ok().map(|_| {
            if !self.is_recording && !self.focused {
                self.recorder.start();
            } else {
                self.recorder.stop();
            }
            self.is_recording = !self.is_recording;
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                egui::Grid::new("status").num_columns(3).show(ui, |ui| {
                    ui.label("Status | ");
                    ui.label(format!(
                        "{}",
                        if self.is_recording {
                            "Recording"
                        } else {
                            "Stop"
                        }
                    ));
                    ui.add_enabled_ui(!self.is_recording, |ui| {
                        if ui.add(Button::new("Get")).clicked() {
                            self.actions = self.recorder.get_actions();
                            for action in self.actions.iter() {
                                println!("{:?}", action);
                            }
                        }
                    });
                });
                ui.add_enabled_ui(!self.is_recording, |ui| {
                    ui.horizontal(|ui| if ui.add(Button::new("Replay")).clicked() {});
                });
            });
        });
        ctx.request_repaint();
    }
}
