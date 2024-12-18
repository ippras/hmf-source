use self::{
    control::{Control, Settings},
    table::TableView,
};
use crate::localization::localize;
use anyhow::Result;
use egui::{RichText, ScrollArea, TextEdit, Ui, menu::bar};
use egui_phosphor::regular::{
    ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, MINUS, PENCIL, PLUS, TAG, TRASH,
};
use polars::prelude::*;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use tracing::error;
use tracing_subscriber::registry::Data;

/// Calculation pane
#[derive(Default, Deserialize, Serialize)]
pub(crate) struct Pane {
    pub(crate) data_frame: DataFrame,
    pub(crate) control: Control,
}

impl Pane {
    pub(crate) const fn new() -> Self {
        Self {
            data_frame: DataFrame::empty(),
            control: Control::new(),
        }
    }

    pub(crate) fn init(data_frame: DataFrame, label: impl Into<String>) -> Self {
        Self {
            data_frame,
            control: Control {
                settings: Settings {
                    label: label.into(),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    pub(crate) fn title(&self) -> &str {
        if self.control.settings.label.is_empty() {
            "HMF"
        } else {
            &self.control.settings.label
        }
    }

    pub(crate) fn header(&mut self, ui: &mut Ui) {
        ui.separator();
        bar(ui, |ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                ui.menu_button(RichText::new(TAG).heading(), |ui| {
                    ui.horizontal(|ui| {
                        ui.add(
                            TextEdit::singleline(&mut self.control.settings.label)
                                .hint_text("File name"),
                        );
                    });
                })
                .response
                .on_hover_text(localize!("label"));
                ui.toggle_value(
                    &mut self.control.settings.resizable,
                    RichText::new(ARROWS_HORIZONTAL).heading(),
                )
                .on_hover_text(localize!("resize"));
                ui.toggle_value(
                    &mut self.control.settings.editable,
                    RichText::new(PENCIL).heading(),
                )
                .on_hover_text(localize!("edit"));
                ui.separator();
                // Add
                if ui
                    .button(RichText::new(PLUS).heading())
                    .on_hover_text(localize!("add"))
                    .clicked()
                {
                    self.add_row().unwrap();
                }
                // Delete
                ui.add_enabled_ui(!self.data_frame.is_empty(), |ui| {
                    ui.menu_button(RichText::new(MINUS).heading(), |ui| {
                        for index in 0..self.data_frame.height() {
                            if ui.button(format!("{MINUS} with index {index}")).clicked() {
                                self.delete_row(index).unwrap();
                                if self.data_frame.is_empty() {
                                    ui.close_menu();
                                }
                            }
                        }
                    });
                });
                // Clear
                if ui
                    .button(RichText::new(TRASH).heading())
                    .on_hover_text(localize!("clear"))
                    .clicked()
                {
                    self.data_frame = DataFrame::empty();
                }
                ui.separator();
                ui.toggle_value(&mut self.control.open, RichText::new(GEAR).heading())
                    .on_hover_text(localize!("settings"));
                ui.separator();
                if ui
                    .button(RichText::new(FLOPPY_DISK).heading())
                    .on_hover_text(localize!("save"))
                    .on_hover_text(&self.control.settings.label)
                    .clicked()
                {
                    if let Err(error) = self.save() {
                        error!(%error);
                    }
                }
            });
        });
    }

    pub(crate) fn content(&mut self, ui: &mut Ui) {
        ui.separator();
        self.control.windows(ui);
        TableView::new(&mut self.data_frame, &self.control.settings).ui(ui);
    }

    pub(super) fn hash(&self) -> u64 {
        // hash(&self.data_frame)
        0
    }

    pub(crate) fn add_row(&mut self) -> PolarsResult<()> {
        self.data_frame = concat(
            [
                self.data_frame.clone().lazy(),
                df! {
                    "FattyAcid" => df! {
                        "Carbons" => &[0u8],
                        "Unsaturated" => &[
                            df! {
                                "Index" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                                "Isomerism" => Series::new_empty(PlSmallStr::EMPTY, &DataType::Int8),
                                "Unsaturation" => Series::new_empty(PlSmallStr::EMPTY, &DataType::UInt8),
                            }?.into_struct(PlSmallStr::EMPTY).into_series(),
                        ],
                    }?.into_struct(PlSmallStr::EMPTY),
                    "TAG" => [0f64],
                    "MAG" => [0f64],
                }?
                .lazy(),
            ],
            UnionArgs {
                rechunk: true,
                diagonal: true,
                ..Default::default()
            },
        )?
        .collect()?;
        Ok(())
    }

    // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
    // https://stackoverflow.com/a/71495211/1522758
    pub(crate) fn delete_row(&mut self, row: usize) -> PolarsResult<()> {
        self.data_frame = self
            .data_frame
            .slice(0, row)
            .vstack(&self.data_frame.slice((row + 1) as _, usize::MAX))?;
        self.data_frame.as_single_chunk_par();
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let contents = ron::ser::to_string_pretty(
            &self.data_frame,
            PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_NEWTYPES),
        )?;
        std::fs::write(format!("{}.hmf.ron", self.control.settings.label), contents)?;
        Ok(())
    }
}

pub(crate) mod control;

mod table;
