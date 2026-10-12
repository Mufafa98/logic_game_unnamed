mod app;
mod consts;
mod logic_item;
mod ports;
mod wire;

use app::MyApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logic Game Unnamed",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
