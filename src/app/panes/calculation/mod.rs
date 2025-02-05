use self::{settings::Settings, state::State, table::TableView};
use crate::{localization::localize, utils::save};
use anyhow::Result;
use egui::{CursorIcon, Response, RichText, ScrollArea, Ui, Window, menu::bar, util::hash};
use egui_phosphor::regular::{
    ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, ERASER, FLOPPY_DISK, GEAR, NOTE_PENCIL, PENCIL, TAG,
};
use metadata::MetaDataFrame;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use tracing::error;

const ID_SOURCE: &str = "Calculation";

/// Calculation pane
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Pane {
    pub(crate) frame: MetaDataFrame,
    pub(crate) settings: Settings,
    state: State,
}

impl Pane {
    pub(crate) const fn new(frame: MetaDataFrame) -> Self {
        Self {
            frame,
            settings: Settings::new(),
            state: State::new(),
        }
    }

    pub(crate) const fn icon() -> &'static str {
        NOTE_PENCIL
    }

    pub(crate) fn title(&self) -> String {
        self.frame.meta.title()
    }

    pub(crate) fn header(&mut self, ui: &mut Ui) -> Response {
        bar(ui, |ui| {
            ScrollArea::horizontal()
                .show(ui, |ui| {
                    ui.visuals_mut().button_frame = false;
                    self.header_content(ui)
                })
                .inner
        })
        .inner
    }

    fn header_content(&mut self, ui: &mut Ui) -> Response {
        let mut response = ui
            .heading(Self::icon())
            .on_hover_text(localize!("configuration"));
        response |= ui.heading(self.title());
        response = response
            .on_hover_text(format!("{:x}", self.hash()))
            .on_hover_cursor(CursorIcon::Grab);
        ui.separator();
        // Reset
        if ui
            .button(RichText::new(ARROWS_CLOCKWISE).heading())
            .clicked()
        {
            self.state.reset_table_state = true;
        }
        // Resize
        ui.toggle_value(
            &mut self.settings.resizable,
            RichText::new(ARROWS_HORIZONTAL).heading(),
        )
        .on_hover_text(localize!("resize"));
        // Edit
        ui.toggle_value(&mut self.settings.editable, RichText::new(PENCIL).heading())
            .on_hover_text(localize!("edit"));
        ui.separator();
        // Clear
        ui.add_enabled_ui(
            self.settings.editable && self.frame.data.height() > 0,
            |ui| {
                if ui
                    .button(RichText::new(ERASER).heading())
                    .on_hover_text(localize!("clear"))
                    .clicked()
                {
                    self.frame.data = self.frame.data.clear();
                }
            },
        );
        ui.separator();
        // Settings
        ui.toggle_value(
            &mut self.state.open_settings_window,
            RichText::new(GEAR).heading(),
        )
        .on_hover_text(localize!("settings"));
        ui.separator();
        if ui
            .button(RichText::new(FLOPPY_DISK).heading())
            .on_hover_text(localize!("save"))
            .on_hover_text(&self.settings.label)
            .clicked()
        {
            if let Err(error) = self.save() {
                error!(%error);
            }
        }
        response
    }

    pub(crate) fn body(&mut self, ui: &mut Ui) {
        self.windows(ui);
        if self.settings.editable {
            self.body_content_meta(ui);
        }
        self.body_content_data(ui);
    }

    fn body_content_meta(&mut self, ui: &mut Ui) {
        ui.style_mut().visuals.collapsing_header_frame = true;
        ui.collapsing(RichText::new(format!("{TAG} Metadata")).heading(), |ui| {
            self.frame.meta.show(ui);
        });
    }

    fn body_content_data(&mut self, ui: &mut Ui) {
        TableView::new(&mut self.frame.data, &self.settings, &mut self.state).show(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        hash(&self.frame)
    }

    fn save(&mut self) -> Result<()> {
        let mut name = self.frame.meta.name.replace(" ", "_");
        if let Some(version) = &self.frame.meta.version {
            write!(name, ".{version}")?;
        }
        name.push_str(".hmf.ipc");
        save(&name, &mut self.frame)?;
        Ok(())
    }

    pub(crate) fn windows(&mut self, ui: &mut Ui) {
        Window::new(format!("{GEAR} Settings"))
            .id(ui.auto_id_with(ID_SOURCE))
            .open(&mut self.state.open_settings_window)
            .show(ui.ctx(), |ui| self.settings.show(ui));
    }

    // fn save(&self) -> Result<()> {
    //     let contents = ron::ser::to_string_pretty(
    //         &self.frame.data,
    //         PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_NEWTYPES),
    //     )?;
    //     std::fs::write(format!("{}.hmf.ron", self.settings.label), contents)?;
    //     Ok(())
    // }
}

pub(crate) mod settings;

mod state;
mod table;
