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

#[cfg(target_arch = "wasm32")]
mod web {
    use super::Hello;
    use wasm_bindgen::prelude::*;

    #[derive(Clone)]
    #[wasm_bindgen]
    pub struct WebHandle {
        runner: eframe::WebRunner,
    }

    #[wasm_bindgen]
    impl WebHandle {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Result<Self, String> {
            match eframe::WebLogger::init(log::LevelFilter::Debug) {
                Ok(()) => Ok(Self {
                    runner: eframe::WebRunner::new(),
                }),
                Err(err) => Err(format!("failed to initialize WebLogger: {err}")),
            }
        }

        #[wasm_bindgen]
        pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
            self.runner
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Ok(Box::new(Hello::new(cc)))),
                )
                .await
        }
    }
}
