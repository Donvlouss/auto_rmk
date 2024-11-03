use eframe::App;
use egui::Button;

use crate::{core::MKAction, record::Record};

#[derive(Debug)]
pub struct RmkApp {
    recorder: Record,
    actions: Vec<MKAction>,
}

impl RmkApp {
    pub fn new() -> Self {
        Self {
            recorder: Record::new(),
            actions: vec![],
        }
    }
}

impl App for RmkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Control Panel")
            .default_pos([1000., 10.])
            .default_width(250.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.add(Button::new("Start")).clicked() {
                        self.recorder.start();
                    }
                    if ui.add(Button::new("Stop")).clicked() {
                        self.recorder.stop();
                    }
                    if ui.add(Button::new("Get")).clicked() {
                        self.actions = self.recorder.get_actions();
                        for action in self.actions.iter() {
                            println!("{:?}", action);
                        }
                    }
                });
            });
    }
}
