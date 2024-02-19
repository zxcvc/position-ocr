// #![windows_subsystem = "windows"]

use anyhow::{self};
use fltk::{
    self,
    prelude::{ButtonExt, GroupExt, InputExt, TableExt, WidgetBase, WidgetExt, WindowExt},
};
use fltk_table::SmartTable;

use std::{self, fs, path};
#[path = "../../ui/mod.rs"]
mod ui;

const ENG_DATA: &[u8] = include_bytes!("../../../eng.traineddata");

fn get_images_from_dir<P: AsRef<path::Path>>(path: P) -> Result<Vec<String>, anyhow::Error> {
    let image_types: [&str; 3] = ["jpg", "png", "jpeg"];
    let dir = fs::read_dir(path)?;
    let res = dir
        .filter_map(|item| match item {
            Ok(it) => {
                match it.path().extension() {
                    Some(ext) => {
                        let ext_str = ext.to_str().unwrap_or_default();
                        if image_types.contains(&ext_str) {
                            Some(it.path().to_str().unwrap_or_default().into())
                        } else {
                            None
                        }
                    }
                    None => None,
                }
                // Some(it.path().to_str().unwrap_or("").into())
            }
            Err(_) => None,
        })
        .collect();
    Ok(res)
}

fn main() -> Result<(), anyhow::Error> {
    let mut api = ocr::OcrApi::new(ENG_DATA);
    let app = fltk::app::App::default();
    let (w, h) = fltk::app::screen_size();
    let mut ui = ui::UserInterface::make_window();
    ui.main_window.size_range(0, 0, w as i32, h as i32);

    let mut file_input = ui.dir_input;
    file_input.set_trigger(fltk::enums::CallbackTrigger::Changed);
    let mut clone_file_input = file_input.clone();
    let mut select_btn = ui.select_btn;

    select_btn.set_callback(move |_btn| {
        let mut dialog =
            fltk::dialog::NativeFileChooser::new(fltk::dialog::FileDialogType::BrowseDir);
        dialog.set_directory(&clone_file_input.value()).expect("1");
        dialog.show();
        let path = dialog.filename();
        if !path.exists() {
            return;
        }
        clone_file_input.set_value(path.to_str().unwrap_or_default());
    });

    ui.main_window.begin();
    let mut table = SmartTable::default().with_pos(370, 5).with_size(700, 415);
    table.set_clip_children(true);
    ui.main_window.end();
    let mut win = ui.main_window;

    let mut cloned = table.clone();
    win.resize_callback(move |win, _x, _y, _w, _h| {
        win.redraw();
        cloned.set_col_width(0, (cloned.width() as f32 * 0.2) as i32);
        cloned.set_col_width(1, (cloned.width() as f32 * 0.7) as i32);
    });

    let mut comfir_btn = ui.comfir_button;
    comfir_btn.set_callback(move |_btn| {
        let image_paths = get_images_from_dir(file_input.value()).unwrap_or_default();
        let result: Vec<Vec<String>> = image_paths
            .iter()
            .enumerate()
            .map(|(_index, item)| {
                vec![
                    path::Path::new(item)
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    api.get_text(item).unwrap_or_default(),
                ]
            })
            .collect();
        // table.set_col_header(true);
        let mut cloned_table = table.clone();
        cloned_table.clear();
        cloned_table.with_opts(fltk_table::TableOpts {
            rows: result.len() as i32,
            cols: 2,
            editable: true,
            cell_align: fltk::enums::Align::Left,
            cell_padding: 2,
            ..Default::default()
        });
        table.clear();
        win.redraw();

        table.set_col_width(0, (table.width() as f32 * 0.2) as i32);
        table.set_col_width(1, (table.width() as f32 * 0.75) as i32);

        for (y, rows) in result.iter().enumerate() {
            for (x, item) in rows.iter().enumerate() {
                table.clone().set_cell_value(y as i32, x as i32, item);
            }
        }
    });
    app.run()?;
    Ok(())
}
