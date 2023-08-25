use eframe::{
    egui::{self, RichText, ScrollArea, Separator, TextEdit, Ui},
    epaint::Color32,
    App,
};
const PADDING: f32 = 5.0;

pub(crate) struct Seract {
    filename: Option<String>,
    content: Option<String>,
    error: Option<String>,
    // tess: leptess::LepTess
}

pub(crate) fn validate_file(file: String) -> bool {
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
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.ctx_get_filename(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_heading(ui);
            self.ui_get_filename(ui);
            if self.filename.is_some() {
                self.ui_get_content(ui);
            }
        });
    }
}

impl Seract {
    // new is declared as unused function by the analyzer
    #[allow(dead_code)]
    pub fn new() -> Self {
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

    fn ctx_get_filename(&mut self, _ctx: &eframe::egui::Context) {
        // let mut cb = arboard::Clipboard::new().unwrap();
        // dbg!(cb.get_text().unwrap());
    }

    fn ui_get_filename(&mut self, ui: &mut Ui) {
        ui.add_space(PADDING);

        ui.vertical_centered(|ui| {
            ui.add_space(PADDING);
            if self.filename.is_some() {
                ui.heading("Change Image");
            } else {
                ui.heading("Add Image");
            }
            ui.label("from");

            ui.horizontal(|ui| {
                ui.add_space(170.);
                let cpd = ui.button("Clipboard");
                if cpd.clicked() {
                    match arboard::Clipboard::new().unwrap().get_text() {
                        Ok(file) => {
                            self.set_filename(file);
                        }
                        _ => {
                            self.error = Some("invalid data!".to_owned());
                            // self.filename = None;
                            // self.content = None;
                        }
                    }
                }

                let dialog = ui.button("Explorer");
                if dialog.clicked() {
                    let file_path = tinyfiledialogs::open_file_dialog("Open Image", "~", None);
                    if let Some(file) = file_path {
                        self.set_filename(file);
                    } else {
                        self.filename = None;
                        self.content = None;
                    }
                }
            });

            ui.add_space(PADDING);

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

    fn set_filename(&mut self, file: String) {
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
