use anyhow::{self};
// use rusty_tesseract::{self as act, Args, Image};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, path};
use tesseract::{self as ocr};
#[derive(Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}
impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// pub fn points_from_img<P: Into<path::PathBuf>>(path: P) -> Result<String, anyhow::Error> {
//     let img = Image::from_path(path)?;
//     let act_arg = Args {
//         config_variables: HashMap::from([(
//             "tessedit_char_whitelist".into(),
//             "0123456789,.-".into(),
//         )]),
//         ..Default::default()
//     };
//     let res = act::image_to_string(&img, &act_arg)?;
//     let mut points = Vec::new();
//     for item in res.lines() {
//         let a: Vec<f32> = item
//             .split(',')
//             .take(2)
//             .map(str::parse)
//             .map(Result::unwrap)
//             .collect();
//         points.push(Point::new(a[0], a[1]));
//     }
//     let res_json = serde_json::to_string(&points)?;
//     Ok(res_json)
// }

pub fn img_to_points(path: &str) -> Result<String, anyhow::Error> {
    let mut tes_oct = ocr::Tesseract::new(None, Some("eng"))?
        .set_image(path)?
        .set_variable("tessedit_char_whitelist", "0123456789.-,")?;
    let res = tes_oct.get_text()?;
    let mut points = Vec::new();
    for item in res.lines() {
        let a: Vec<f32> = item
            .split(',')
            .take(2)
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        points.push(Point::new(a[0], a[1]));
    }
    let res_json = serde_json::to_string(&points)?
        .replace("\"x\"", "x")
        .replace("\"y\"", "y");
    Ok(res_json)
}
