use crate::presets::_10_1021_jf903048p::MATURE_MILK_FAT;
use egui::{ComboBox, InnerResponse, Ui};
use lipid::fatty_acid::{
    FattyAcid,
    display::{COMMON, DisplayWithOptions},
    polars::DataFrameExt as _,
};
use polars::prelude::*;

/// Fatty acid widget
pub(crate) struct FattyAcidWidget<'a> {
    pub(crate) value: Box<dyn Fn() -> PolarsResult<Option<FattyAcid>> + 'a>,
    pub(crate) editable: bool,
    pub(crate) hover: bool,
}

impl<'a> FattyAcidWidget<'a> {
    pub(crate) fn new(value: impl Fn() -> PolarsResult<Option<FattyAcid>> + 'a) -> Self {
        Self {
            value: Box::new(value),
            editable: false,
            hover: false,
        }
    }

    pub(crate) fn editable(self, editable: bool) -> Self {
        Self { editable, ..self }
    }

    pub(crate) fn hover(self) -> Self {
        Self {
            hover: true,
            ..self
        }
    }

    pub(crate) fn try_ui(self, ui: &mut Ui) -> PolarsResult<InnerResponse<Option<FattyAcid>>> {
        let fatty_acid = (self.value)()?;
        let text = match &fatty_acid {
            Some(fatty_acid) => &format!("{:#}", fatty_acid.display(COMMON)),
            None => "",
        };
        let mut inner = None;
        let mut response = if self.editable {
            let current_value = &mut fatty_acid.unwrap_or_default();
            let response = ComboBox::from_id_salt(ui.next_auto_id())
                .width(ui.available_width())
                .selected_text(text)
                .show_ui(ui, |ui| -> PolarsResult<()> {
                    let mature_milk = MATURE_MILK_FAT.data.fatty_acid();
                    for index in 0..mature_milk.len() {
                        if let Some(selected_value) = mature_milk.get(index)? {
                            let text = format!("{:#}", (&selected_value).display(COMMON));
                            if ui
                                .selectable_value(current_value, selected_value, text)
                                .changed()
                            {
                                inner = Some(current_value.clone())
                            }
                        }
                    }
                    Ok(())
                });
            response.response
        } else {
            ui.label(text)
        };
        if self.hover {
            response = response.on_hover_text(text);
        }
        Ok(InnerResponse::new(inner, response))
    }

    pub(crate) fn ui(self, ui: &mut Ui) -> InnerResponse<Option<FattyAcid>> {
        self.try_ui(ui).expect("Fatty acid widget")
    }
}
