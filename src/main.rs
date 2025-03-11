use eframe::*;
use egui::*;

struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World 🚀");
        });
    }
}

fn main() {
    println!("Launching 🚀");
}
