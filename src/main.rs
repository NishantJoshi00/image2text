use eframe::{egui::TextBuffer, epaint::Vec2, run_native, App, CreationContext, NativeOptions};
mod seract;

type AppFactory = Box<dyn FnOnce(&CreationContext) -> Box<dyn App>>;

fn main() {
    let args = std::env::args();
    if args.len() != 1 {
        let filename = args.collect::<Vec<String>>();
        let filename = filename[1].clone();
        if seract::validate_file(filename.clone()) {
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

    let app_factory: AppFactory = Box::new(|_ctx| Box::new(seract::Seract::new()));
    let win_options = NativeOptions {
        initial_window_size: Some(Vec2::new(480., 640.)),
        resizable: false,
        ..NativeOptions::default()
    };
    let _ = run_native("Headlines".as_str(), win_options, app_factory);
}
