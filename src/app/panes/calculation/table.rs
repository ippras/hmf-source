use super::control::Settings;
use crate::app::{
    MARGIN,
    computers::{CalculationComputed, CalculationKey},
    widgets::{FloatWidget, new_fatty_acid::FattyAcidWidget},
};
use egui::{Frame, Id, Margin, Response, TextStyle, TextWrapMode, Ui};
use egui_phosphor::regular::MINUS;
use egui_table::{AutoSizeMode, CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate};
use lipid::fatty_acid::{
    FattyAcid,
    polars::{DataFrameExt as _, SeriesExt as _},
};
use polars::{chunked_array::builder::AnonymousOwnedListBuilder, prelude::*};
use std::ops::Range;

const ID: Range<usize> = 0..2;
const EXPERIMENTAL: Range<usize> = ID.end..ID.end + 2;
const CALCULATED: Range<usize> = EXPERIMENTAL.end..EXPERIMENTAL.end + 11;
const LEN: usize = CALCULATED.end;

const TOP: &[Range<usize>] = &[ID, EXPERIMENTAL, CALCULATED];

const MIDDLE: &[Range<usize>] = &[
    id::INDEX,
    id::FA,
    experimental::TAG123,
    experimental::MAG2,
    calculated::SN123,
    calculated::SN2,
    calculated::F,
];

/// Table view
pub(super) struct TableView<'a> {
    source: &'a mut DataFrame,
    target: DataFrame,
    settings: &'a Settings,
    event: Option<Event>,
}

impl<'a> TableView<'a> {
    pub(super) fn new(data_frame: &'a mut DataFrame, settings: &'a Settings) -> Self {
        Self {
            source: data_frame,
            target: DataFrame::empty(),
            settings,
            event: None,
        }
    }
}

impl TableView<'_> {
    pub(super) fn ui(&mut self, ui: &mut Ui) -> Option<Event> {
        self.target = ui.memory_mut(|memory| {
            memory
                .caches
                .cache::<CalculationComputed>()
                .get(CalculationKey {
                    data_frame: self.source,
                    settings: self.settings,
                })
        });
        let id_salt = Id::new("CalculationDataTable");
        let height = ui.text_style_height(&TextStyle::Heading);
        let num_rows = self.source.height() as u64 + 1;
        let num_columns = LEN;
        Table::new()
            .id_salt(id_salt)
            .num_rows(num_rows)
            .columns(vec![
                Column::default().resizable(self.settings.resizable);
                num_columns
            ])
            .num_sticky_cols(self.settings.sticky)
            .headers([
                HeaderRow {
                    height,
                    groups: TOP.to_vec(),
                },
                HeaderRow {
                    height,
                    groups: MIDDLE.to_vec(),
                },
                HeaderRow::new(height),
            ])
            .auto_size_mode(AutoSizeMode::OnParentResize)
            .show(ui, self);
        self.event
    }

    fn header_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) {
        if self.settings.truncate {
            ui.style_mut().wrap_mode = Some(TextWrapMode::Truncate);
        }
        match (row, column) {
            // Top
            (0, 0) => {
                ui.heading("ID");
            }
            (0, 1) => {
                ui.heading("Experimental");
            }
            (0, 2) => {
                ui.heading("Calculated");
            }
            // Middle
            (1, 0) => {
                ui.heading("Index");
            }
            (1, 1) => {
                ui.heading("FA");
            }
            (1, 2) => {
                ui.heading("TAG");
            }
            (1, 3) => {
                ui.heading("MAG2");
            }
            (1, 4) => {
                ui.heading("SN123");
            }
            (1, 5) => {
                ui.heading("SN2");
            }
            // Bottom
            (2, 4 | 9) => {
                ui.heading("A");
            }
            (2, 5 | 10) => {
                ui.heading("B");
            }
            (2, 6 | 11) => {
                ui.heading("C");
            }
            (2, 7 | 12) => {
                ui.heading("D");
            }
            (2, 8 | 13) => {
                ui.heading("E");
            }
            (2, 14) => {
                ui.heading("F");
            }
            _ => {} // _ => unreachable!(),
        };
    }

    fn cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) -> PolarsResult<()> {
        if !self.source.is_empty() {
            if row == self.source.height() {
                self.footer_cell_content_ui(ui, column)?;
            } else {
                self.body_cell_content_ui(ui, row, column)?;
            }
        }
        Ok(())
    }

    fn body_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) -> PolarsResult<()> {
        match (row, column..column + 1) {
            (row, id::INDEX) => {
                if self.settings.editable {
                    if ui.button(MINUS).clicked() {
                        self.event = Some(Event::DeleteRow(row));
                    }
                }
                let indices = self.target["Index"].u32()?;
                let index = indices.get(row).unwrap();
                ui.label(index.to_string());
            }
            (row, id::FA) => {
                let inner_response = FattyAcidWidget::new(|| self.source.fatty_acid().get(row))
                    .editable(self.settings.editable)
                    .hover()
                    .ui(ui)?;
                if let Some(value) = inner_response.inner {
                    self.source
                        .try_apply("FattyAcid", change_fatty_acid(row, &value))?;
                }
                // let changed = FattyAcidWidget::new(|| self.source.fatty_acid().get(row))
                //     .editable(self.settings.editable)
                //     .hover()
                //     .ui(ui)
                //     .inner;
                // if let Some(value) = changed {
                //     self.source
                //         .try_apply("FattyAcid", change_fatty_acid(row, &value))?;
                // }
            }
            (row, experimental::TAG123) => {
                self.rw(ui, row, "TAG")?;
            }
            (row, experimental::MAG2) => {
                self.rw(ui, row, "MAG")?;
            }
            (row, calculated::sn123::A) => {
                self.ro(ui, || {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("A")?
                        .struct_()?
                        .field_by_name("Value")?
                        .f64()?
                        .get(row))
                })?
                .on_hover_ui(|ui| {
                    ui.horizontal(|ui| {
                        FloatWidget::new(|| {
                            Ok(self.target["SN123"]
                                .struct_()?
                                .field_by_name("A")?
                                .struct_()?
                                .field_by_name("Min")?
                                .f64()?
                                .get(row))
                        })
                        .ui(ui);
                        ui.label("-");
                        FloatWidget::new(|| {
                            Ok(self.target["SN123"]
                                .struct_()?
                                .field_by_name("A")?
                                .struct_()?
                                .field_by_name("Max")?
                                .f64()?
                                .get(row))
                        })
                        .ui(ui);
                    });
                });
            }
            (row, calculated::sn123::B) => {
                self.ro(ui, || {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("B")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn123::C) => {
                self.ro(ui, || {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("C")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn123::D) => {
                self.ro(ui, || {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("D")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn123::E) => {
                self.ro(ui, || {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn2::A) => {
                self.ro(ui, || {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("A")?
                        .struct_()?
                        .field_by_name("Value")?
                        .f64()?
                        .get(row))
                })?
                .on_hover_ui(|ui| {
                    ui.horizontal(|ui| {
                        FloatWidget::new(|| {
                            Ok(self.target["SN2"]
                                .struct_()?
                                .field_by_name("A")?
                                .struct_()?
                                .field_by_name("Min")?
                                .f64()?
                                .get(row))
                        })
                        .ui(ui);
                        ui.label("-");
                        FloatWidget::new(|| {
                            Ok(self.target["SN2"]
                                .struct_()?
                                .field_by_name("A")?
                                .struct_()?
                                .field_by_name("Max")?
                                .f64()?
                                .get(row))
                        })
                        .ui(ui);
                    });
                });
            }
            (row, calculated::sn2::B) => {
                self.ro(ui, || {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("B")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn2::C) => {
                self.ro(ui, || {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("C")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn2::D) => {
                self.ro(ui, || {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("D")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::sn2::E) => {
                self.ro(ui, || {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .get(row))
                })?;
            }
            (row, calculated::F) => {
                self.ro(ui, || Ok(self.target["F"].f64()?.get(row)))?;
            }
            _ => {} // _ => unreachable!(),
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: usize) -> PolarsResult<()> {
        match column..column + 1 {
            experimental::TAG123 => {
                FloatWidget::new(|| Ok(self.source["TAG"].f64()?.sum()))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui)
                    .response
                    .on_hover_text("∑TAG");
            }
            experimental::MAG2 => {
                FloatWidget::new(|| Ok(self.source["MAG"].f64()?.sum()))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui)
                    .response
                    .on_hover_text("∑MAG");
            }
            calculated::sn123::E => {
                FloatWidget::new(|| {
                    Ok(self.target["SN123"]
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .sum()
                        .map(|e| 50.0 - e))
                })
                .precision(Some(self.settings.precision))
                .hover()
                .ui(ui)
                .response
                .on_hover_text("50 - ∑E");
            }
            calculated::sn2::E => {
                FloatWidget::new(|| {
                    Ok(self.target["SN2"]
                        .struct_()?
                        .field_by_name("E")?
                        .f64()?
                        .sum()
                        .map(|e| 50.0 - e))
                })
                .precision(Some(self.settings.precision))
                .hover()
                .ui(ui)
                .response
                .on_hover_text("50 - ∑E");
            }
            calculated::F => {
                FloatWidget::new(|| Ok(self.target["F"].f64()?.sum().map(|f| 100.0 - f)))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui)
                    .response
                    .on_hover_text("100 - ∑F");
            }
            _ => {} // _ => unreachable!(),
        }
        Ok(())
    }

    fn rw(&mut self, ui: &mut Ui, row: usize, column: &str) -> PolarsResult<Response> {
        let inner_response = FloatWidget::new(|| Ok(self.source[column].f64()?.get(row)))
            .editable(self.settings.editable)
            .precision(Some(self.settings.precision))
            .hover()
            .ui(ui);
        if let Some(value) = inner_response.inner {
            self.source
                .try_apply(column, change_experimental(row, value))?;
        }
        Ok(inner_response.response)
    }

    fn ro(&self, ui: &mut Ui, f: impl Fn() -> PolarsResult<Option<f64>>) -> PolarsResult<Response> {
        Ok(FloatWidget::new(f)
            .precision(Some(self.settings.precision))
            .hover()
            .ui(ui)
            .response)
    }
}

impl TableDelegate for TableView<'_> {
    fn header_cell_ui(&mut self, ui: &mut Ui, cell: &HeaderCellInfo) {
        Frame::none()
            .inner_margin(Margin::symmetric(MARGIN.x, MARGIN.y))
            .show(ui, |ui| {
                self.header_cell_content_ui(ui, cell.row_nr, cell.group_index)
            });
    }

    fn cell_ui(&mut self, ui: &mut Ui, cell: &CellInfo) {
        if cell.row_nr % 2 == 0 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
        Frame::none()
            .inner_margin(Margin::symmetric(MARGIN.x, MARGIN.y))
            .show(ui, |ui| {
                self.cell_content_ui(ui, cell.row_nr as _, cell.col_nr)
                    .unwrap()
            });
    }
}

/// Event
#[derive(Clone, Copy, Debug)]
pub(super) enum Event {
    DeleteRow(usize),
}

fn change_fatty_acid(
    row: usize,
    new: &FattyAcid,
) -> impl FnMut(&Series) -> PolarsResult<Series> + '_ {
    move |series| {
        let fatty_acid_series = series.fatty_acid();
        let mut carbons = PrimitiveChunkedBuilder::<UInt8Type>::new(
            fatty_acid_series.carbons.name().clone(),
            fatty_acid_series.len(),
        );
        let mut unsaturated = AnonymousOwnedListBuilder::new(
            fatty_acid_series.unsaturated.name().clone(),
            fatty_acid_series.len(),
            fatty_acid_series.unsaturated.dtype().inner_dtype().cloned(),
        );
        for index in 0..fatty_acid_series.len() {
            let mut fatty_acid = fatty_acid_series.get(index)?;
            if index == row {
                fatty_acid = Some(new.clone());
            }
            let fatty_acid = fatty_acid.as_ref();
            // Carbons
            carbons.append_option(fatty_acid.map(|fatty_acid| fatty_acid.carbons));
            // Unsaturated
            if let Some(fatty_acid) = fatty_acid {
                // let mut fields = Vec::with_capacity(fatty_acid.unsaturated.len());
                // if let Some(unsaturated_series) = fatty_acid_series.unsaturated(index)? {
                //     fields.push(unsaturated_series.index);
                //     fields.push(unsaturated_series.isomerism);
                //     fields.push(unsaturated_series.unsaturation);
                // }
                // unsaturated.append_series(
                //     &StructChunked::from_series(
                //         PlSmallStr::EMPTY,
                //         fatty_acid.unsaturated.len(),
                //         fields.iter(),
                //     )?
                //     .into_series(),
                // )?;
                let mut index = PrimitiveChunkedBuilder::<UInt8Type>::new(
                    "Index".into(),
                    fatty_acid.unsaturated.len(),
                );
                let mut isomerism = PrimitiveChunkedBuilder::<Int8Type>::new(
                    "Isomerism".into(),
                    fatty_acid.unsaturated.len(),
                );
                let mut unsaturation = PrimitiveChunkedBuilder::<UInt8Type>::new(
                    "Unsaturation".into(),
                    fatty_acid.unsaturated.len(),
                );
                for unsaturated in &fatty_acid.unsaturated {
                    index.append_option(unsaturated.index);
                    isomerism.append_option(unsaturated.isomerism.map(|isomerism| isomerism as _));
                    unsaturation.append_option(
                        unsaturated
                            .unsaturation
                            .map(|unsaturation| unsaturation as _),
                    );
                }
                unsaturated.append_series(
                    &StructChunked::from_series(
                        PlSmallStr::EMPTY,
                        fatty_acid.unsaturated.len(),
                        [
                            index.finish().into_series(),
                            isomerism.finish().into_series(),
                            unsaturation.finish().into_series(),
                        ]
                        .iter(),
                    )?
                    .into_series(),
                )?;
            } else {
                println!("HERE1");
                unsaturated.append_opt_series(None)?;
            }
        }
        Ok(StructChunked::from_series(
            series.name().clone(),
            fatty_acid_series.len(),
            [
                carbons.finish().into_series(),
                unsaturated.finish().into_series(),
            ]
            .iter(),
        )?
        .into_series())
    }
}

fn change_experimental(row: usize, new: f64) -> impl FnMut(&Series) -> PolarsResult<Series> {
    move |series| {
        Ok(series
            .f64()?
            .iter()
            .enumerate()
            .map(|(index, mut value)| {
                if index == row {
                    value = Some(new);
                }
                Ok(value)
            })
            .collect::<PolarsResult<Float64Chunked>>()?
            .into_series())
    }
}

mod id {
    use super::*;

    pub(super) const INDEX: Range<usize> = ID.start..ID.start + 1;
    pub(super) const FA: Range<usize> = INDEX.end..INDEX.end + 1;
}

mod experimental {
    use super::*;

    pub(super) const TAG123: Range<usize> = EXPERIMENTAL.start..EXPERIMENTAL.start + 1;
    pub(super) const MAG2: Range<usize> = TAG123.end..TAG123.end + 1;
}

mod calculated {
    use super::*;

    pub(super) const SN123: Range<usize> = CALCULATED.start..CALCULATED.start + 5;
    pub(super) const SN2: Range<usize> = SN123.end..SN123.end + 5;
    pub(super) const F: Range<usize> = SN2.end..SN2.end + 1;

    pub(super) mod sn123 {
        use super::*;

        pub(in super::super) const A: Range<usize> = SN123.start..SN123.start + 1;
        pub(in super::super) const B: Range<usize> = A.end..A.end + 1;
        pub(in super::super) const C: Range<usize> = B.end..B.end + 1;
        pub(in super::super) const D: Range<usize> = C.end..C.end + 1;
        pub(in super::super) const E: Range<usize> = D.end..D.end + 1;
    }

    pub(super) mod sn2 {
        use super::*;

        pub(in super::super) const A: Range<usize> = SN2.start..SN2.start + 1;
        pub(in super::super) const B: Range<usize> = A.end..A.end + 1;
        pub(in super::super) const C: Range<usize> = B.end..B.end + 1;
        pub(in super::super) const D: Range<usize> = C.end..C.end + 1;
        pub(in super::super) const E: Range<usize> = D.end..D.end + 1;
    }
}
