use eframe::{
    egui::{self, RichText, ScrollArea, Separator, TextEdit, Ui},
    epaint::{Color32, Vec2},
    epi::App,
    run_native, NativeOptions,
};
const PADDING: f32 = 5.0;

struct Seract {
    filename: Option<String>,
    content: Option<String>,
    error: Option<String>,
    // tess: leptess::LepTess
}

fn validate_file(file: String) -> bool {
    let path = std::path::Path::new(&file);
    if path.exists() && path.is_file() {
        let ext = path.extension().unwrap().to_str().unwrap();
        if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "bmp" {
            return true;
        }
    }
    false
}

impl App for Seract {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_heading(ui);
            self.ui_get_filename(ui);
            if self.filename.is_some() {
                self.ui_get_content(ui);
            }
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

impl Seract {
    fn new() -> Self {
        Self {
            filename: None,
            content: None,
            error: None,
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
    fn ui_get_filename(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);

        ui.vertical_centered(|ui| {
            ui.add_space(PADDING);
            let btn = if self.filename.is_some() {
                ui.button("Change Image")
            } else {
                ui.button("Open Image")
            };
            ui.add_space(PADDING);

            if btn.clicked() {
                let file_path = tinyfiledialogs::open_file_dialog("Open Image", "~", None);
                if let Some(file) = file_path {
                    if validate_file(file.clone()) {
                        self.filename = Some(file);
                        self.error = None;
                        let mut tess = leptess::LepTess::new(None, "eng").unwrap();
                        tess.set_image(self.filename.clone().unwrap()).unwrap();
                        self.content = Some(tess.get_utf8_text().unwrap());
                        let mut cb = arboard::Clipboard::new().unwrap();
                        cb.set_text(self.content.clone().unwrap()).unwrap();
                    } else {
                        self.error = Some("invalid file!".to_string());
                        self.filename = None;
                        self.content = None;
                    }
                } else {
                    self.filename = None;
                    self.content = None;
                }
            }

            if let Some(file) = self.filename.clone() {
                ui.label(RichText::new(file).color(Color32::GREEN));
            } else if let Some(error) = self.error.clone() {
                ui.label(RichText::new(error).color(Color32::RED));
            }

            ui.add_space(PADDING);
            let sep = Separator::default().spacing(20.);
            ui.add(sep);
        });
    }

    fn ui_get_content(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);
        let mut value = self.content.clone().unwrap();
        // ui.label(value);
        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                TextEdit::multiline(&mut value)
                    .desired_width(f32::INFINITY)
                    .code_editor(),
            );
        });
    }
}

fn main() {
    let args = std::env::args();
    if args.len() != 1 {
        let filename = args.collect::<Vec<String>>();
        let filename = filename[1].clone();
        if validate_file(filename.clone()) {
            let mut tess = leptess::LepTess::new(None, "eng").unwrap();
            tess.set_variable(leptess::Variable::UserDefinedDpi, "70")
                .unwrap();
            tess.set_image(filename).unwrap();
            let text = tess.get_utf8_text().unwrap();
            let mut cb = arboard::Clipboard::new().unwrap();
            cb.set_text(text.clone()).unwrap();
            println!("{}", text);
            return;
        }
    }

    let app = Seract::new();
    let win_options = NativeOptions {
        initial_window_size: Some(Vec2::new(480., 640.)),
        ..NativeOptions::default()
    };
    run_native(Box::new(app), win_options);
}
