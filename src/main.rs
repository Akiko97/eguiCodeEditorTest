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
            code: "; Test
vextractf128 xmm0, ymm0, 1
vbroadcastsd ymm5, xmm0
shufpd xmm0, xmm0, 1
vbroadcastsd ymm4, xmm0
vextractf128 xmm0, ymm0, 0
vbroadcastsd ymm7, xmm0
shufpd xmm0, xmm0, 1
vbroadcastsd ymm6, xmm0
vmulpd ymm7, ymm3, ymm7
vfmadd213pd ymm6, ymm2, ymm7
vfmadd213pd ymm5, ymm1, ymm6
vfmadd213pd ymm4, ymm0, ymm5
vmovapd ymm8, ymm4\n".into(),
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
                .with_syntax(Syntax::asm())
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
