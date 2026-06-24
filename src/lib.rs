#[cfg(target_arch = "wasm32")]
mod web;

use eframe::egui;
use eframe::{App, Frame};
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Hello;

impl Hello {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.storage
            .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl App for Hello {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        ui.heading("Hello, World!");
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
