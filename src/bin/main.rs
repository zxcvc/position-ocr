use anyhow::{self, Ok};
use fltk::{
    self,
    prelude::{InputExt, WidgetExt},
};
use ocr::img_to_points;
#[path = "../ui/mod.rs"]
mod ui;

const IMG_PATH: &str = "./1.png";

fn main() -> Result<(), anyhow::Error> {
    let res = img_to_points(IMG_PATH)?;
    dbg!(res);
    let app = fltk::app::App::default();
    let mut ui = ui::UserInterface::make_window();
    let mut input_value = 0;
    let mut input = ui.input;
    input.set_value(input_value.to_string().as_str());
    ui.button.set_callback(move |_btn| {
        input_value += 1;
        input.set_value(input_value.to_string().as_str());
    });
    app.run()?;
    Ok(())
}
