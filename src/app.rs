use egui::Button;

use crate::{models::user_state::{UserState}, components::user_registration::registration_ui};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SpaceTradersApp {
    user_state: UserState,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for SpaceTradersApp {
    fn default() -> Self {
        Self {
            user_state: UserState::default(),
            value: 0.
        }
    }
}

impl SpaceTradersApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for SpaceTradersApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    let clear_btn = Button::new("Clear Data");
                    let clear_data = ui.add_enabled(self.user_state.token.is_some(), clear_btn);
                    if clear_data.clicked() {
                        self.user_state = UserState::default();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.user_state.token.is_some() {
                ui.heading(format!("Welcome back to Space Traders, {} 🚀", self.user_state.name));
                return;
            }
            ui.heading("Welcome to Space Traders 🚀");
            ui.add_space(10.);
            registration_ui(ui, &mut self.user_state);
        });

        egui::TopBottomPanel::bottom("debug_panel").show(ctx, |ui| {
            ui.add_space(5.);
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);
                let quit = ui.button("Quit");
                if quit.clicked() {
                    frame.close();
                }
            });
            ui.add_space(3.);
        });
    }
}
