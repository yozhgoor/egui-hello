#[cfg(not(target_arch = "wasm32"))]
use egui_hello::Hello;

fn main() -> eframe::Result {
    #[cfg(not(target_arch = "wasm32"))]
    eframe::run_native(
        "hello",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Hello::new(cc)))),
    )?;

    Ok(())
}
