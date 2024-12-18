use crate::{app::MAX_PRECISION, localization::localize};
use egui::{Grid, Slider, Ui, Widget, Window};
use egui_phosphor::regular::{
    ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, MINUS, PENCIL, PLUS, TAG, TRASH,
};
use serde::{Deserialize, Serialize};

/// Calculation control
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Control {
    pub(crate) settings: Settings,
    pub(crate) open: bool,
}

impl Control {
    pub(crate) const fn new() -> Self {
        Self {
            settings: Settings::new(),
            open: false,
        }
    }

    pub(crate) fn windows(&mut self, ui: &mut Ui) {
        Window::new(format!("{GEAR} Calculation settings"))
            .id(ui.next_auto_id())
            .default_pos(ui.next_widget_position())
            .open(&mut self.open)
            .show(ui.ctx(), |ui| self.settings.ui(ui));
    }
}

/// Calculation settings
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub(crate) struct Settings {
    #[serde(skip)]
    pub(crate) resizable: bool,
    #[serde(skip)]
    pub(crate) editable: bool,
    pub(crate) label: String,
    pub(crate) precision: usize,
    pub(crate) round: u32,
    pub(crate) sticky: usize,
    pub(crate) truncate: bool,

    pub(crate) names: bool,
    pub(crate) properties: bool,
}

impl Settings {
    pub(crate) const fn new() -> Self {
        Self {
            resizable: false,
            editable: false,
            label: String::new(),
            precision: 2,
            round: 0,
            sticky: 0,
            truncate: false,
            names: true,
            properties: true,
        }
    }

    pub(crate) fn ui(&mut self, ui: &mut Ui) {
        Grid::new(ui.next_auto_id()).show(ui, |ui| {
            // Precision
            ui.label(localize!("precision"));
            Slider::new(&mut self.precision, 0..=MAX_PRECISION).ui(ui);
            ui.end_row();

            ui.separator();
            ui.separator();
            ui.end_row();

            // Round
            ui.label(localize!("round"));
            Slider::new(&mut self.round, 0..=MAX_PRECISION as _)
                .ui(ui)
                .on_hover_text(localize!("round_description"));
            ui.end_row();

            // Properties
            ui.label(localize!("properties"));
            ui.checkbox(&mut self.properties, "")
                .on_hover_text(localize!("properties_description"));
            ui.end_row();

            // Names
            ui.label(localize!("names"));
            ui.checkbox(&mut self.names, "")
                .on_hover_text(localize!("names_description"));
        });
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
