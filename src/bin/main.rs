use anyhow::{self, Ok};
use fltk::{self, prelude::{WidgetExt, InputExt}};
use ocr::points_from_img;
#[path="../ui/mod.rs"]
mod ui;



const IMG_PATH: &str = "./1.png";

fn main()->Result<(),anyhow::Error> {
    let res = points_from_img(IMG_PATH)?;
    let app = fltk::app::App::default();
    let mut ui = ui::UserInterface::make_window();
    let mut input_value = 0;
    let mut input = ui.input;
    input.set_value(input_value.to_string().as_str());
    ui.button.set_callback(move |btn|{
        input_value += 1;
        input.set_value(input_value.to_string().as_str());
    });
    app.run()?;
    Ok(())
}
