use self::{
    panes::{Pane, behavior::Behavior},
    windows::About,
};
use crate::{
    localization::{UiExt, localize},
    presets::{
        C70_CONTROL, C70_H2O2, C70_NACL, CP_9, CV_15, CV_395, CZ_30412, HMF_1, HMF_2, HMF_3, HMF_4,
        ISO_FJ,
    },
};
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, FontDefinitions, Id, LayerId, Layout, Order,
    RichText, ScrollArea, Sides, TextStyle, TopBottomPanel, Vec2, Visuals, menu::bar, vec2,
    warn_if_debug_build,
};
use egui_ext::{DroppedFileExt, HoveredFileExt, LightDarkButton};
use egui_notify::Toasts;
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, DATABASE, GEAR, GRID_FOUR, INFO, PENCIL, PLUS,
        SQUARE_SPLIT_HORIZONTAL, SQUARE_SPLIT_VERTICAL, TABS, TRASH,
    },
};
use egui_tiles::{Container, ContainerKind, Tile, TileId, Tiles, Tree};
use egui_tiles_ext::{ContainerExt as _, TilesExt as _, TreeExt as _};
use itertools::Either;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    fmt::Write,
    iter::once,
    mem::take,
    str,
    sync::mpsc::{Receiver, Sender, channel},
    time::Duration,
};
use tracing::{error, info, trace};

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

pub(super) const SIZE: f32 = 32.0;
const MARGIN: Vec2 = vec2(4.0, 0.0);
const NOTIFICATIONS_DURATION: Duration = Duration::from_secs(15);

// const DESCRIPTION: &str = "Positional-species and positional-type composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

fn custom_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = custom_visuals(style.visuals);
    ctx.set_style(style);
}

fn custom_visuals<T: BorrowMut<Visuals>>(mut visuals: T) -> T {
    visuals.borrow_mut().collapsing_header_frame = true;
    visuals
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Panels
    left_panel: bool,
    // Panes
    tree: Tree<Pane>,

    // Data channel
    #[serde(skip)]
    channel: (Sender<(String, String)>, Receiver<(String, String)>),

    // Windows
    #[serde(skip)]
    about: About,
    // Notifications
    #[serde(skip)]
    toasts: Toasts,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_panel: true,
            tree: Tree::empty("central_tree"),
            channel: channel(),
            toasts: Default::default(),
            about: Default::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // Customize style of egui.
        let mut fonts = FontDefinitions::default();
        add_to_fonts(&mut fonts, Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        custom_style(&cc.egui_ctx);

        // return Default::default();
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let app = Self::load(cc).unwrap_or_default();
        app.context(&cc.egui_ctx);
        app
    }

    fn load(cc: &CreationContext) -> Option<Self> {
        let storage = cc.storage?;
        let value = get_value(storage, APP_KEY)?;
        Some(value)
    }

    fn context(&self, ctx: &Context) {
        // Data channel
        ctx.data_mut(|data| data.insert_temp(Id::new("Data"), self.channel.0.clone()));
    }
}

// Panels
impl App {
    fn panels(&mut self, ctx: &Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.central_panel(ctx);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                Sides::new().show(
                    ui,
                    |_| {},
                    |ui| {
                        warn_if_debug_build(ui);
                        ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                        ui.separator();
                    },
                );
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.0))
            .show(ctx, |ui| {
                let mut behavior = Behavior { close: None };
                self.tree.ui(&mut behavior, ui);
                if let Some(id) = behavior.close {
                    self.tree.tiles.remove(id);
                }
            });
    }

    // Top panel
    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            bar(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Light/Dark
                    ui.light_dark_button(SIZE);
                    ui.separator();
                    // Reset
                    if ui
                        .button(RichText::new(TRASH).size(SIZE))
                        .on_hover_text(localize!("reset_application"))
                        .clicked()
                    {
                        *self = Default::default();
                        self.context(ctx);
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(ARROWS_CLOCKWISE).size(SIZE))
                        .on_hover_text(localize!("reset_gui"))
                        .clicked()
                    {
                        ui.memory_mut(|memory| {
                            memory.caches = take(memory).caches;
                        });
                        self.context(ctx);
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(SQUARE_SPLIT_VERTICAL).size(SIZE))
                        .on_hover_text(localize!("vertical"))
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Vertical);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(SQUARE_SPLIT_HORIZONTAL).size(SIZE))
                        .on_hover_text(localize!("horizontal"))
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Horizontal);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(GRID_FOUR).size(SIZE))
                        .on_hover_text(localize!("grid"))
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Grid);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(TABS).size(SIZE))
                        .on_hover_text(localize!("tabs"))
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Tabs);
                            }
                        }
                    }
                    ui.separator();
                    // Resizable
                    let mut resizable = true;
                    if ui
                        .button(RichText::new(ARROWS_HORIZONTAL).size(SIZE))
                        .on_hover_text(localize!("resize"))
                        .clicked()
                    {
                        let mut panes = self
                            .tree
                            .tiles
                            .tiles_mut()
                            .filter_map(|tile| match tile {
                                Tile::Pane(pane) => Some(pane),
                                Tile::Container(_) => None,
                            })
                            .peekable();
                        if let Some(pane) = panes.peek() {
                            resizable ^= pane.control.settings.resizable;
                        }
                        for pane in panes {
                            pane.control.settings.resizable = resizable;
                        }
                    };
                    let mut editable = true;
                    if ui
                        .button(RichText::new(PENCIL).size(SIZE))
                        .on_hover_text(localize!("edit"))
                        .clicked()
                    {
                        let mut panes = self
                            .tree
                            .tiles
                            .tiles_mut()
                            .filter_map(|tile| match tile {
                                Tile::Pane(pane) => Some(pane),
                                Tile::Container(_) => None,
                            })
                            .peekable();
                        if let Some(pane) = panes.peek() {
                            editable ^= pane.control.settings.editable;
                        }
                        for pane in panes {
                            pane.control.settings.editable = editable;
                        }
                    };
                    ui.separator();
                    // Load
                    ui.menu_button(RichText::new(DATABASE).size(SIZE), |ui| {
                        if ui
                            .button(RichText::new(format!("{DATABASE} HMF-1")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(HMF_1.clone(), "HMF-1"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} HMF-2")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(HMF_2.clone(), "HMF-2"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} HMF-3")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(HMF_3.clone(), "HMF-3"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} HMF-4")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(HMF_4.clone(), "HMF-4"));
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .button(RichText::new(format!("{DATABASE} CV-15")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(CV_15.clone(), "CV-15"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} CZ-30412")).heading())
                            .clicked()
                        {
                            self.tree
                                .insert_pane(Pane::init(CZ_30412.clone(), "CZ-30412"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} CP-9")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(CP_9.clone(), "CP-9"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} CV-395")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(CV_395.clone(), "CV-395"));
                            ui.close_menu();
                        }
                        //
                        if ui
                            .button(RichText::new(format!("{DATABASE} ISO-FJ")).heading())
                            .clicked()
                        {
                            self.tree.insert_pane(Pane::init(ISO_FJ.clone(), "ISO-FJ"));
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .button(RichText::new(format!("{DATABASE} C70-Control")).heading())
                            .clicked()
                        {
                            self.tree
                                .insert_pane(Pane::init(C70_CONTROL.clone(), "C70-Control"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} C70-H2O2")).heading())
                            .clicked()
                        {
                            self.tree
                                .insert_pane(Pane::init(C70_H2O2.clone(), "C70-H2O2"));
                            ui.close_menu();
                        }
                        if ui
                            .button(RichText::new(format!("{DATABASE} C70-NaCl")).heading())
                            .clicked()
                        {
                            self.tree
                                .insert_pane(Pane::init(C70_NACL.clone(), "C70-NaCl"));
                            ui.close_menu();
                        }
                    });
                    // Create
                    if ui.button(RichText::new(PLUS).size(SIZE)).clicked() {
                        self.tree.insert_pane(Pane::new());
                    }
                    ui.separator();
                    // About
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // About
                        if ui
                            .button(RichText::new(INFO).size(SIZE))
                            .on_hover_text("About window")
                            .clicked()
                        {
                            self.about.open ^= true;
                        }
                        ui.separator();
                        // Locale
                        ui.locale_button().on_hover_text(localize!("language"));
                    });
                });
            });
        });
    }
}

// /// Extension methods for [`Tiles`]
// pub trait TilesExt<T> {
//     fn filter_child_pane<'a>(
//         &'a mut self,
//         f: impl Fn(&T) -> bool + 'a,
//     ) -> impl Iterator<Item = TileId> + 'a;
// }

// impl<T> TilesExt<T> for Tiles<T> {
//     fn filter_child_pane<'a>(
//         &'a mut self,
//         f: impl Fn(&T) -> bool + 'a,
//     ) -> impl Iterator<Item = TileId> + 'a {
//         self.tiles()
//     }
// }

// /// [`Container`] extension methods
// pub trait ContainerExt {
//     fn filter_child_pane<'a, T>(
//         &'a self,
//         tiles: &'a Tiles<T>,
//     ) -> Box<dyn Iterator<Item = &'a T> + 'a>;
// }

// impl ContainerExt for Container {
//     fn filter_child_pane<'a, T>(
//         &'a self,
//         tiles: &'a Tiles<T>,
//     ) -> Box<dyn Iterator<Item = &'a T> + 'a> {
//         Box::new(
//             self.children()
//                 .filter_map(|child| tiles.get(*child))
//                 .flat_map(|child| match child {
//                     Tile::Container(container) => container.filter_child_pane(tiles),
//                     Tile::Pane(pane) => Box::new(once(pane)),
//                 }),
//         )
//     }

//     // fn active_panes<'a, T>(&'a self, tiles: &'a Tiles<T>, f: impl Fn(&T)) {
//     //     for child in self.active_children() {
//     //         match tiles.get(*child).unwrap() {
//     //             Tile::Container(container) => container.active_panes(tiles, &f),
//     //             Tile::Pane(pane) => f(pane),
//     //         }
//     //     }
//     // }
// }

// fn filter_pane_by<'a, T: 'a>(
//     iter: impl Iterator<Item = (&'a TileId, &'a Tile<T>)> + 'a,
//     f: impl Fn(&T) -> bool + 'a,
// ) -> impl Iterator<Item = TileId> + 'a {
//     iter.filter(move |(_, tile)| match *tile {
//         Tile::Pane(pane) => f(pane),
//         Tile::Container(container) => container.children(),
//     })
//     .map(|(tile_id, _)| *tile_id)
// }

// Windows
impl App {
    fn windows(&mut self, ctx: &Context) {
        self.about.window(ctx);
    }
}

// Notifications
impl App {
    fn notifications(&mut self, ctx: &Context) {
        self.toasts.show(ctx);
    }
}

// Copy/Paste, Drag&Drop
impl App {
    fn drag_and_drop(&mut self, ctx: &Context) {
        // Preview hovering files
        if let Some(text) = ctx.input(|input| {
            (!input.raw.hovered_files.is_empty()).then(|| {
                let mut text = String::from("Dropping files:");
                for file in &input.raw.hovered_files {
                    write!(text, "\n{}", file.display()).ok();
                }
                text
            })
        }) {
            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
        // Parse dropped files
        if let Some(dropped_files) = ctx.input(|input| {
            (!input.raw.dropped_files.is_empty()).then_some(input.raw.dropped_files.clone())
        }) {
            info!(?dropped_files);
            for dropped in dropped_files {
                trace!(?dropped);
                let content = match dropped.content() {
                    Ok(content) => content,
                    Err(error) => {
                        error!(%error);
                        self.toasts
                            .error(format!("{}: {error}", dropped.display()))
                            .closable(true)
                            .duration(Some(NOTIFICATIONS_DURATION));
                        continue;
                    }
                };
                trace!(content);
                let name = dropped.name();
                self.channel
                    .0
                    .send((
                        name.strip_suffix(".ron")
                            .and_then(|name| name.strip_suffix(".hmf"))
                            .unwrap_or(name)
                            .to_owned(),
                        content,
                    ))
                    .ok();
            }
        }
    }

    fn parse(&mut self, _ctx: &Context) {
        for (name, content) in self.channel.1.try_iter() {
            trace!(name, content);
            match ron::de::from_str(&content) {
                Ok(data_frame) => {
                    trace!(?data_frame);
                    self.tree.insert_pane(Pane::init(data_frame, name));
                }
                Err(error) => error!(%error),
            };
        }
    }

    // fn paste(&mut self, ctx: &Context) {
    //     if !ctx.memory(|memory| memory.focused().is_some()) {
    //         ctx.input(|input| {
    //             for event in &input.raw.events {
    //                 if let Event::Paste(paste) = event {
    //                     if let Err(error) = self.parse(paste) {
    //                         error!(?error);
    //                         self.toasts
    //                             .error(error.to_string().chars().take(64).collect::<String>())
    //                             .set_duration(Some(Duration::from_secs(5)))
    //                             .set_closable(true);
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }

    // fn parse(&mut self, paste: &str) -> Result<()> {
    //     use crate::parsers::whitespace::Parser;
    //     let parsed = Parser::parse(paste)?;
    //     debug!(?parsed);
    //     for parsed in parsed {
    //         // self.docks.central.tabs.input.add(match parsed {
    //         //     Parsed::All(label, (c, n), tag, dag, mag) => FattyAcid {
    //         //         label,
    //         //         formula: ether!(c as usize, n as usize),
    //         //         values: [tag, dag, mag],
    //         //     },
    //         //     // Parsed::String(label) => Row { label, ..default() },
    //         //     // Parsed::Integers(_) => Row { label, ..default() },
    //         //     // Parsed::Float(tag) => Row { label, ..default() },
    //         //     _ => unimplemented!(),
    //         // })?;
    //         // self.config.push_row(Row {
    //         //     acylglycerols,
    //         //     label:  parsed.,
    //         //     ether: todo!(),
    //         //     // ..default()
    //         // })?;
    //     }
    //     // let mut rows = Vec::new();
    //     // for row in paste.split('\n') {
    //     //     let mut columns = [0.0; COUNT];
    //     //     for (j, column) in row.split('\t').enumerate() {
    //     //         ensure!(j < COUNT, "Invalid shape, columns: {COUNT} {j}");
    //     //         columns[j] = column.replace(',', ".").parse()?;
    //     //     }
    //     //     rows.push(columns);
    //     // }
    //     // for acylglycerols in rows {
    //     //     self.config.push_row(Row {
    //     //         acylglycerol: acylglycerols,
    //     //         ..default()
    //     //     })?;
    //     // }
    //     Ok(())
    // }

    // fn export(&self) -> Result<(), impl Debug> {
    //     let content = to_string(&TomlParsed {
    //         name: self.context.state.entry().meta.name.clone(),
    //         fatty_acids: self.context.state.entry().fatty_acids(),
    //     })
    //     .unwrap();
    //     self.file_dialog
    //         .save(
    //             &format!("{}.toml", self.context.state.entry().meta.name),
    //             content,
    //         )
    //         .unwrap();
    //     Ok::<_, ()>(())
    // }

    // fn import(&mut self) -> Result<(), impl Debug> {
    //     self.file_dialog.load()
    // }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
        // set_value(storage, APP_KEY, &Self::default());
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Pre update
        self.panels(ctx);
        self.windows(ctx);
        self.notifications(ctx);
        // Post update
        self.drag_and_drop(ctx);
        self.parse(ctx);
    }
}

mod computers;
mod panes;
mod widgets;
mod windows;
