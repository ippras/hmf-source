use crate::{
    app::{ICON_SIZE, panes::Pane},
    presets::*,
};
use anyhow::Result;
use egui::{Response, RichText, ScrollArea, Separator, Ui, Widget};
use egui_phosphor::regular::DATABASE;
use egui_tiles::Tree;
use egui_tiles_ext::{TreeExt, VERTICAL};
use metadata::MetaDataFrame;
use ron::{extensions::Extensions, ser::PrettyConfig};
use std::fs::File;

/// Load
pub(crate) struct Load<'a> {
    tree: &'a mut Tree<Pane>,
}
impl<'a> Load<'a> {
    pub(crate) fn new(tree: &'a mut Tree<Pane>) -> Self {
        Self { tree }
    }
}

impl Load<'_> {
    fn presets(&mut self, ui: &mut Ui) {
        macro preset($frame:path) {
            if ui
                .button(RichText::new(format!("{DATABASE} {}", $frame.meta.title())).heading())
                .clicked()
            {
                self.tree.insert_pane::<VERTICAL>(Pane::new($frame.clone()));
            }
        }

        // <https://doi.org/10.1038/sj.ejcn.1601470>
        ui.add(doi_separator("10.1038/sj.ejcn.1601470"));
        ui.label(RichText::new("A López-López (2002)").small());
        preset!(_10_1038_sj_ejcn_1601470::CMF);
        preset!(_10_1038_sj_ejcn_1601470::TMF);
        preset!(_10_1038_sj_ejcn_1601470::MMF);
        // <https://doi.org/10.1021/jf903048p>
        ui.add(doi_separator("10.1021/jf903048p"));
        ui.label(RichText::new("Yong-Hua Wang (2009)").small());
        preset!(_10_1021_jf903048p::CMF_AF);
        preset!(_10_1021_jf903048p::CMF_AP);
        preset!(_10_1021_jf903048p::CMF_R);
        preset!(_10_1021_jf903048p::MMF_A);
        // <https://doi.org/10.1016/j.algal.2018.11.004>
        ui.add(doi_separator("10.1016/j.algal.2018.11.004"));
        ui.label(RichText::new("Yongjin He (2019)").small());
        preset!(_10_1016_j_algal_2018_11_004::CV_15);
        preset!(_10_1016_j_algal_2018_11_004::CZ_30412);
        preset!(_10_1016_j_algal_2018_11_004::CV_395);
        preset!(_10_1016_j_algal_2018_11_004::CP_9);
        preset!(_10_1016_j_algal_2018_11_004::SS);
        preset!(_10_1016_j_algal_2018_11_004::CS);
        preset!(_10_1016_j_algal_2018_11_004::NL_2047);
        preset!(_10_1016_j_algal_2018_11_004::PT_646);
        preset!(_10_1016_j_algal_2018_11_004::ISO_FJ);
        preset!(_10_1016_j_algal_2018_11_004::IG_2307);
        preset!(_10_1016_j_algal_2018_11_004::NO_IMET1);
        preset!(_10_1016_j_algal_2018_11_004::NS_537);
        // IPPRAS
        ui.horizontal(|ui| {
            ui.hyperlink_to(RichText::new("IPPRAS").heading(), "https://ippras.ru");
            ui.add(Separator::default().horizontal());
        });
        preset!(ippras::C70_CONTROL);
        preset!(ippras::C70_H2O2);
        preset!(ippras::C70_NACL);
        ui.separator();
        preset!(ippras::H242_N);
        preset!(ippras::H242_N_1);
        preset!(ippras::H242_N_2);
        preset!(ippras::H242_N_3);
        ui.separator();
    }
}

impl Widget for Load<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.menu_button(RichText::new(DATABASE).size(ICON_SIZE), |ui| {
            ScrollArea::new([false, true]).show(ui, |ui| self.presets(ui));
        })
        .response
    }
}

fn doi_separator(doi: &str) -> impl Fn(&mut Ui) -> Response {
    move |ui| {
        ui.horizontal(|ui| {
            ui.hyperlink_to(
                RichText::new(format!("DOI: {doi}")).heading(),
                format!("https://doi.org/{doi}"),
            );
            ui.add(Separator::default().horizontal());
        })
        .response
    }
}

fn ipc(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    let file = File::create(name)?;
    MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write(file)?;
    Ok(())
}

fn ron(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    let file = File::create(name)?;
    ron::ser::to_writer_pretty(
        file,
        &frame.data,
        PrettyConfig::default().extensions(Extensions::IMPLICIT_SOME),
    )?;
    Ok(())
}
// fn json(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
//     // let contents = ron::ser::to_string_pretty(
//     //     &frame.data,
//     //     PrettyConfig::new().extensions(Extensions::IMPLICIT_SOME | Extensions::UNWRAP_NEWTYPES),
//     // )?;
//     let contents = serde_json::to_string(&frame.data)?;
//     println!("contents: {contents}");
//     std::fs::write(name, contents)?;
//     Ok(())
// }
