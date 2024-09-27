pub fn request_focus_on_ctrl(i: usize, ui: &egui::Ui, id: egui::Id, rect: egui::Rect) {
    let keys = [
        egui::Key::Num0,
        egui::Key::Num1,
        egui::Key::Num2,
        egui::Key::Num3,
        egui::Key::Num4,
        egui::Key::Num5,
        egui::Key::Num6,
        egui::Key::Num7,
        egui::Key::Num8,
        egui::Key::Num9,
    ];
    if let Some(k) = keys.get(i) {
        if ui.input(|i| i.modifiers.ctrl && i.key_pressed(*k)) {
            ui.memory_mut(|m| m.request_focus(id));
            ui.scroll_to_rect(rect, None);
        }
    }
}
