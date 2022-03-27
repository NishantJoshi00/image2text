use eframe::{run_native, epi::App, egui::{self, Ui, Separator}, NativeOptions, epaint::Vec2};
const PADDING: f32 = 5.0;

struct Seract {
    filename: String,
    content: Option<String>
}


impl App for Seract {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_heading(ui);
            self.ui_drag_drop(ui);
        });
        
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

impl Seract {
    fn new() -> Self {
        Self {
            filename: String::new(),
            content: None
        }
    }
    fn ui_heading(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(PADDING);
            ui.heading("Image2Text");
            // ui.add_space(PADDING);

        });
        let sep = Separator::default().spacing(20.);
        ui.add(sep);
    }
    fn ui_drag_drop(&mut self, ui: &mut Ui) {
        ui.add(Separator::default().horizontal());
        
        ui.add(Separator::default().horizontal());

    }
}

fn main() {
    let app = Seract::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(Vec2::new(540., 480.));
    run_native(Box::new(app), win_options);
}