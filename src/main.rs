use eframe::*;

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello World 🚀");
            let btn_response = ui.button("I'm button 🐥");
            if btn_response.clicked() {
                println!("We was clicked 🌍");
            }
        });
    }
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

fn main() -> eframe::Result {
    println!("Launching 🚀");
    let native_options = NativeOptions::default();
    run_native(
        "Joint Sketch 🦀",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
