pub struct AddUi {
    named_pipe_path: String,
}

pub enum AddUiResult {
    Nop,
    Add(String),
}

const DEFAULT_PATH: &str = r"mpv\default";

impl Default for AddUi {
    fn default() -> Self {
        Self {
            named_pipe_path: DEFAULT_PATH.into(),
        }
    }
}

impl AddUi {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> AddUiResult {
        let mut result = AddUiResult::Nop;

        let mut enter = false;

        ui.horizontal(|ui| {
            ui.label("Named pipe path:");
        });
        ui.vertical_centered_justified(|ui| {
            let input = egui::TextEdit::singleline(&mut self.named_pipe_path)
                .hint_text(DEFAULT_PATH)
                .show(ui);

            enter |= input.response.lost_focus() && ui.input(|i| i.key_down(egui::Key::Enter));
        });
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("add").clicked() {
                enter |= true;
            }
        });

        if enter {
            result = AddUiResult::Add(self.named_pipe_path.clone());
        }

        result
    }
}
