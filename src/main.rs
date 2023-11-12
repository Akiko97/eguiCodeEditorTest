use eframe::egui::{self, CentralPanel};
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};

struct MyApp {
    code: String,
    highlight: usize,
    highlight_str: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            code: "// A very simple example\n\
fn main() {\n\
\tprintln!(\"Hello world!\");\n\
}\n\
".into(),
            highlight: 0,
            highlight_str: "0".into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            CodeEditor::default()
                .id_source("code_editor")
                .with_rows(12)
                .with_fontsize(14.0)
                .with_theme(ColorTheme::GRUVBOX)
                .with_syntax(Syntax::rust())
                .with_numlines(true)
                .show(ui, &mut self.code, &mut self.highlight);
            ui.text_edit_singleline(&mut self.highlight_str);
            if ui.button("Highlight").clicked() {
                if let Ok(line) = self.highlight_str.parse::<usize>() {
                    self.highlight = line;
                }
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Code Editor Test",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
