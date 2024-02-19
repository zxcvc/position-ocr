#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui::{self, style::Selection, Color32, Stroke}, icon_data};
use env_logger::{self};
use rfd::{self};
use std::{path, sync::Arc};

#[path = "../../utils/mod.rs"]
mod utils;

mod them;

const ENG_DATA: &[u8] = include_bytes!("../../../eng.traineddata");
const FONT_DATA: &[u8] = include_bytes!("../../../msyh.ttc");
const ICON_DATA: &[u8] = include_bytes!("../../../icon.png");
const WIDTH: f32 = 980.0;
const HEIGHT: f32 = 480.0;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport:egui::ViewportBuilder{
            icon:Some(Arc::new(icon_data::from_png_bytes(ICON_DATA).unwrap())),
            ..egui::ViewportBuilder::default()
            .with_inner_size([WIDTH, HEIGHT])
            .with_drag_and_drop(false)
        },
        ..Default::default()
    };

    eframe::run_native(
        "IMG TO JSON",
        options,
        Box::new(|cc| {
            let vis = cc.egui_ctx.style().visuals.clone();
            let vis = egui::style::Visuals {
                selection: Selection {
                    bg_fill: Color32::from_rgba_premultiplied(51, 103, 209, 255),
                    stroke: Stroke {
                        width: 1.0,
                        color: Color32::from_rgba_premultiplied(255, 255, 255, 255),
                    },
                },
                ..vis
            };
            cc.egui_ctx.set_visuals(vis);
            let font_data = eframe::egui::FontData::from_static(FONT_DATA);
            let mut font_define = eframe::egui::FontDefinitions::default();
            font_define.font_data.insert("arial".to_string(), font_data);
            font_define
                .families
                .entry(eframe::egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "arial".to_string());
            cc.egui_ctx.set_fonts(font_define);
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    picked_path: String,
    data: Vec<Vec<String>>,
    pub api: ocr::OcrApi,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            picked_path: "".to_string(),
            data: vec![],
            api: ocr::OcrApi::new(ENG_DATA),
        }
    }
}

impl<'a> eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.picked_path);
                let select_path_btn = ui.button("选择文件夹");
                if select_path_btn.clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.picked_path = path.display().to_string();
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                let submit_btn = ui.button("识别");
                let export_btn = ui.button("导出");

                if submit_btn.clicked() {
                    let imgs = utils::get_images_from_dir(&self.picked_path);
                    if let Some(paths) = imgs {
                        let res: Vec<_> = paths
                            .iter()
                            .zip(1..=paths.len())
                            .map(|(item, index)| {
                                vec![
                                    format!("{}", index),
                                    path::Path::new(&item)
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .to_string(),
                                    self.api.get_text(item).unwrap_or_default(),
                                ]
                            })
                            .collect();
                        self.data = res;
                    }
                }

                if export_btn.clicked() {
                    let now = chrono::Local::now();
                    let file_name = now.format("%Y_%m_%d_%H_%M_%S.txt").to_string();
                    utils::save_as(&file_name, self.data.iter().map(|v| v[2].clone()).collect())
                        .expect("保存失败");
                }
            });

            ui.separator();

            use egui_extras::{Column, TableBuilder};

            let table = TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::with_main_justify(
                    egui::Layout::left_to_right(egui::Align::LEFT),
                    true,
                ))
                .column(Column::auto_with_initial_suggestion(40.0))
                .column(Column::auto_with_initial_suggestion(60.0))
                .column(Column::remainder())
                .min_scrolled_height(0.0);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("序号");
                    });
                    header.col(|ui| {
                        ui.strong("图片");
                    });
                    header.col(|ui| {
                        ui.strong("结果");
                    });
                })
                .body(|body| {
                    body.rows(20.0, self.data.len(), |mut row| {
                        let index = row.index();
                        row.col(|ui| {
                            ui.label(&self.data[index][0]);
                        });
                        row.col(|ui| {
                            ui.label(&self.data[index][1]);
                        });
                        row.col(|ui| {
                            ui.text_edit_singleline(&mut self.data[index][2]);
                        });
                    })
                })
        });
    }
}
