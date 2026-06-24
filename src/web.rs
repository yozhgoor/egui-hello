
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
