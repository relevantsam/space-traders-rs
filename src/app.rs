use egui::Button;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SpaceTradersApp {
    user_name: String,
    user_token: Option<String>,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

const DEFAULT_USER_NAME: &'static str = "SpaceCowboy3000";

impl Default for SpaceTradersApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            user_name: DEFAULT_USER_NAME.to_string(),
            user_token: None,
            value: 2.7,
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

    pub fn is_entered_username_valid(user_name: &str) -> bool {
        !(user_name == DEFAULT_USER_NAME || user_name.is_empty())
    }
}

impl eframe::App for SpaceTradersApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { user_name, user_token, .. } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            if let None = *user_token {
                ui.heading("Welcome to Space Traders 🚀");
                ui.group(|registration| {
                    registration.label("Let's register you for your new trading adventure");
                    registration.text_edit_multiline(user_name);
                    let start_buttom = Button::new("Space Country, let's ride");
                    let is_valid = Self::is_entered_username_valid(user_name);
                    let start = registration.add_enabled(is_valid, start_buttom);
                    if start.clicked() {
                        *user_token = Some("Fake token".to_string());
                    }
                });
            } else { 
                ui.heading(format!("Welcome back to Space Traders, {} 🚀", user_name));
            }

            egui::warn_if_debug_build(ui);
        });

        egui::TopBottomPanel::bottom("debug_panel").show(ctx, |ui| {
            let clear_state = ui.button("Clear State");
            if clear_state.clicked() {
                *user_token = None;
            }
        });
    }
}
