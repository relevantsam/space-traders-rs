use egui::{Ui, TextEdit, TextStyle, Layout, Align, Button};

use crate::{constants::{strings::{SAMPLE_USER_NAME, CHOOSE_NAME_PROMPT, CHOOSE_NAME_SUBMISSION_LABEL}, limits::MAX_PLAYER_NAME_LEN}, models::{user_name::Name, user_state::UserState}};

pub fn registration_ui(ui: &mut Ui, user_state: &mut UserState)  {
    ui.group(|ui| {
        ui.set_max_width(
            if ui.available_width() < 400. {
                ui.available_width()
            } else {
                ui.available_width() / 3.
            }
        );
        ui.label(CHOOSE_NAME_PROMPT);
        ui.add_space(10.);
        let Name(name) = &mut user_state.name;
        let name_field = TextEdit::singleline( name)
            .desired_width(ui.available_width())
            .font(TextStyle::Monospace)
            .hint_text(SAMPLE_USER_NAME);
        let field = ui.add(name_field);
        ui.with_layout(Layout::right_to_left(Align::TOP), |label| {
            label.add_space(10.);
            label.label(format!("{} / {}", name.len(), MAX_PLAYER_NAME_LEN));
        });
    
        if field.changed() {
            let mut new_name = name.clone();
            user_state.name.set(&mut new_name);
        }
        ui.add_space(10.);
    
        let start_btn: Button = Button::new(CHOOSE_NAME_SUBMISSION_LABEL);
        let start = ui.add_enabled(user_state.name.is_valid(), start_btn);
        if start.clicked() {
            user_state.token = Some("Fake token".to_string());
        }
    });
}
