use super::control::Settings;
use crate::{
    app::{
        computers::{CalculationComputed, CalculationKey},
        widgets::{FattyAcidWidget, FloatWidget},
        MARGIN,
    },
    special::{
        fatty_acid::FattyAcid,
        polars::{series::SeriesExt as _, DataFrameExt as _},
    },
};
use egui::{Frame, Id, Margin, TextStyle, TextWrapMode, Ui};
use egui_phosphor::regular::MINUS;
use egui_table::{AutoSizeMode, CellInfo, Column, HeaderCellInfo, HeaderRow, Table, TableDelegate};
use polars::{chunked_array::builder::AnonymousOwnedListBuilder, prelude::*};
use std::ops::Range;

const ID: Range<usize> = 0..2;
const EXPERIMENTAL: Range<usize> = ID.end..ID.end + 2;
const CALCULATED: Range<usize> = EXPERIMENTAL.end..EXPERIMENTAL.end + 11;
// const CONTROL: Range<usize> = CALCULATED.end..CALCULATED.end + 1;
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

    pub(super) const _A: Range<usize> = CALCULATED.start..CALCULATED.start + 1;
    pub(super) const _B: Range<usize> = _A.end.._A.end + 1;
    pub(super) const _C: Range<usize> = _B.end.._B.end + 1;
    pub(super) const _D: Range<usize> = _C.end.._C.end + 1;
    pub(super) const _E: Range<usize> = _D.end.._D.end + 1;
}

/// Table view
pub(crate) struct TableView<'a> {
    source: &'a mut DataFrame,
    target: DataFrame,
    settings: &'a Settings,
    changed: bool,
}

impl<'a> TableView<'a> {
    pub(crate) fn new(data_frame: &'a mut DataFrame, settings: &'a Settings) -> Self {
        Self {
            source: data_frame,
            target: DataFrame::empty(),
            settings,
            changed: false,
        }
    }
}

impl TableView<'_> {
    pub(crate) fn ui(&mut self, ui: &mut Ui) {
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
        let mut num_columns = LEN;
        if self.settings.editable {
            num_columns += 1;
        }
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

    fn body_cell_content_ui(&mut self, ui: &mut Ui, row: usize, column: usize) -> PolarsResult<()> {
        if self.changed {
            return Ok(());
        }
        match (row, column) {
            (row, 0) => {
                let indices = self.target["Index"].u32()?;
                let index = indices.get(row).unwrap();
                ui.label(index.to_string());
            }
            (row, 1) => {
                let changed = FattyAcidWidget::new(|| self.source.fatty_acid().get(row))
                    .editable(self.settings.editable)
                    .hover()
                    .ui(ui)
                    .inner;
                if let Some(value) = changed {
                    self.source
                        .try_apply("FattyAcid", fatty_acid_change(row, &value))?;
                }
            }
            (row, 2) => self.experimental(ui, row, "TAG")?,
            (row, 3) => self.experimental(ui, row, "MAG")?,
            (row, 4) => self.calculated(ui, row, ["SN123", "A"])?,
            (row, 5) => self.calculated(ui, row, ["SN123", "B"])?,
            (row, 6) => self.calculated(ui, row, ["SN123", "C"])?,
            (row, 7) => self.calculated(ui, row, ["SN123", "D"])?,
            (row, 8) => self.calculated(ui, row, ["SN123", "E"])?,
            (row, 9) => self.calculated(ui, row, ["SN2", "A"])?,
            (row, 10) => self.calculated(ui, row, ["SN2", "B"])?,
            (row, 11) => self.calculated(ui, row, ["SN2", "C"])?,
            (row, 12) => self.calculated(ui, row, ["SN2", "D"])?,
            (row, 13) => self.calculated(ui, row, ["SN2", "E"])?,
            (row, 14) => {
                FloatWidget::new(|| Ok(self.target["F"].f64()?.get(row)))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui);
            }
            (row, 15) => {
                // Delete row
                if self.settings.editable {
                    if ui.button(MINUS).clicked() {
                        self.delete(row)?;
                        self.changed = true;
                    }
                }
            }
            _ => {} // _ => unreachable!(),
        }
        Ok(())
    }

    fn footer_cell_content_ui(&mut self, ui: &mut Ui, column: usize) -> PolarsResult<()> {
        match column {
            2 => {
                FloatWidget::new(|| Ok(self.source["TAG"].f64()?.sum()))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui)
                    .response
                    .on_hover_text("∑TAG");
            }
            3 => {
                FloatWidget::new(|| Ok(self.source["MAG"].f64()?.sum()))
                    .precision(Some(self.settings.precision))
                    .hover()
                    .ui(ui)
                    .response
                    .on_hover_text("∑MAG");
            }
            8 => {
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
            13 => {
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
            14 => {
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

    // https://stackoverflow.com/questions/71486019/how-to-drop-row-in-polars-python
    // https://stackoverflow.com/a/71495211/1522758
    pub(crate) fn delete(&mut self, row: usize) -> PolarsResult<()> {
        *self.source = self
            .source
            .slice(0, row)
            .vstack(&self.source.slice((row + 1) as _, usize::MAX))?;
        self.source.as_single_chunk_par();
        Ok(())
    }

    fn experimental(&mut self, ui: &mut Ui, row: usize, column: &str) -> PolarsResult<()> {
        let changed = FloatWidget::new(|| Ok(self.source[column].f64()?.get(row)))
            .editable(self.settings.editable)
            .precision(Some(self.settings.precision))
            .hover()
            .ui(ui)
            .inner;
        if let Some(value) = changed {
            self.source
                .try_apply(column, experimental_change(row, value))?;
        }
        Ok(())
    }

    fn calculated(&mut self, ui: &mut Ui, row: usize, column: [&str; 2]) -> PolarsResult<()> {
        FloatWidget::new(|| {
            Ok(self.target[column[0]]
                .struct_()?
                .field_by_name(column[1])?
                .f64()?
                .get(row))
        })
        .precision(Some(self.settings.precision))
        .hover()
        .ui(ui);
        Ok(())
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
        if cell.row_nr % 2 == 1 {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, ui.visuals().faint_bg_color);
        }
        Frame::none()
            .inner_margin(Margin::symmetric(MARGIN.x, MARGIN.y))
            .show(ui, |ui| {
                if !self.source.is_empty() {
                    if cell.row_nr == self.source.height() as _ {
                        self.footer_cell_content_ui(ui, cell.col_nr).unwrap()
                    } else {
                        self.body_cell_content_ui(ui, cell.row_nr as _, cell.col_nr)
                            .unwrap()
                    }
                }
            });
    }
}

fn fatty_acid_change(
    row: usize,
    changed: &FattyAcid,
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
                fatty_acid = Some(changed.clone());
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

fn experimental_change(row: usize, changed: f64) -> impl FnMut(&Series) -> PolarsResult<Series> {
    move |series| {
        Ok(series
            .f64()?
            .iter()
            .enumerate()
            .map(|(index, mut value)| {
                if index == row {
                    value = Some(changed);
                }
                Ok(value)
            })
            .collect::<PolarsResult<Float64Chunked>>()?
            .into_series())
    }
}
