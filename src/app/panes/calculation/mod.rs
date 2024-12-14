use self::control::Control;
use crate::localization::localize;
use anyhow::Result;
use egui::{menu::bar, RichText, ScrollArea, TextEdit, Ui};
use egui_phosphor::regular::{ARROWS_HORIZONTAL, FLOPPY_DISK, GEAR, PENCIL, PLUS, TAG};
use polars::prelude::*;
use ron::{extensions::Extensions, ser::PrettyConfig};
use serde::{Deserialize, Serialize};
use table::TableView;
use tracing::error;

/// Configuration pane
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
                if ui
                    .button(RichText::new(PLUS).heading())
                    .on_hover_text(localize!("add"))
                    .clicked()
                {
                    self.add().unwrap();
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

    pub(crate) fn add(&mut self) -> PolarsResult<()> {
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
        // .with_row_index("Index", Some(self.data_frame.height() as _))
        .collect()?;
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

// fn fatty_acid_add() -> impl FnMut(&Series) -> PolarsResult<Series> {
//     move |series| {
//         let fatty_acid_series = series.fatty_acid();
//         let carbons = fatty_acid_series
//             .carbons
//             .u8()?
//             .iter()
//             .chain(None)
//             .collect::<UInt8Chunked>();
//         let unsaturated = fatty_acid_series
//             .unsaturated
//             .list()?
//             .iter()
//             .chain(None)
//             .collect::<ListChunked>();
//         Ok(StructChunked::from_series(
//             series.name().clone(),
//             fatty_acid_series.len(),
//             [carbons.into_series(), unsaturated.into_series()].iter(),
//         )?
//         .into_series())
//     }
// }

pub(crate) mod control;

mod table;
