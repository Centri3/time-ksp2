#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;

use app::CountdownApp;
use eframe::NativeOptions;
use eframe::Result;

fn main() -> Result<()> {
    eframe::run_native(
        "Time until KSP2",
        NativeOptions::default(),
        Box::new(|cc| Box::new(CountdownApp::new(cc))),
    )
}
