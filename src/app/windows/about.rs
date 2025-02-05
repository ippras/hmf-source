use crate::app::ICON_SIZE;
use egui::{Context, Id, Label, RichText, Sense, Widget, Window};
use egui_phosphor::regular::{COPYRIGHT, GITHUB_LOGO, GLOBE, INFO, WARNING};

/// About
#[derive(Debug, Default)]
pub(crate) struct About {
    pub(crate) open: bool,
}

impl About {
    pub(crate) fn window(&mut self, ctx: &Context) {
        Window::new(format!("{INFO} About"))
            .open(&mut self.open)
            .show(ctx, |ui| {
                // let color = ui.visuals().text_color();
                // let mut text = LayoutJob::default();
                // text.append(
                //     &format!(
                //         "HMF {version}\n\
                //         Ultimate TAG Calculation Application\n\
                //         © 2023 Giorgi Kazakov & Roman Sidorov"
                //     ),
                //     0.0,
                //     TextFormat {
                //         color,
                //         ..Default::default()
                //     },
                // );
                // ctx.frame_nr()
                ui.vertical_centered(|ui| {
                    let version = env!("CARGO_PKG_VERSION");
                    ui.label(format!("HMF {version}"));
                    ui.label("Human Milk Fat matching");
                    ui.label(COPYRIGHT);
                    Label::new("Giorgi Kazakov").sense(Sense::click()).ui(ui);
                    let id = Id::new("counter");
                    Label::new("Roman Sidorov").sense(Sense::click()).ui(ui);
                    ui.label("2024");
                    ui.separator();
                    ui.collapsing(RichText::new("Links").heading(), |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GLOBE).size(ICON_SIZE))
                                .on_hover_text("web");
                            ui.hyperlink_to(
                                "https://ippras.github.io/hmf",
                                "https://ippras.github.io/hmf",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(GITHUB_LOGO).size(ICON_SIZE))
                                .on_hover_text("github.com");
                            ui.hyperlink_to(
                                "https://github.com/ippras/hmf",
                                "https://github.com/ippras/hmf",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(WARNING).size(ICON_SIZE))
                                .on_hover_text("report an issue");
                            ui.hyperlink_to(
                                "https://github.com/ippras/hmf/issues",
                                "https://github.com/ippras/hmf/issues",
                            );
                        });
                    });
                });
            });
    }
}
