use anyhow::{self};
use ocr::plumbing::leptonica_plumbing::Pix;
// use rusty_tesseract::{self as act, Args, Image};
use serde::{Deserialize, Serialize};
use serde_json;
use std::ffi::CString;
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

pub fn img_to_points(path: &str, lang_data: &[u8]) -> Result<String, anyhow::Error> {
    let mut api = ocr::plumbing::TessBaseApi::create();
    api.init_1(
        lang_data,
        Some(&CString::new("eng")?),
        ocr::OcrEngineMode::TesseractLstmCombined as i32,
    )?;
    let mut pix = match Pix::read(&CString::new(path)?) {
        Ok(pix) => pix,
        Err(_) => {
            return Ok("".to_string());
        }
    };
    let pix = unsafe {
        let pix = ocr::plumbing::leptonica_plumbing::leptonica_sys::pixConvertRGBToGray(
            pix.as_mut(),
            0.30,
            0.59,
            0.11,
        );
        pix
    };
    api.set_image_2(&unsafe { Pix::new_from_pointer(pix) });
    api.set_variable(
        &CString::new("tessedit_char_whitelist")?,
        &CString::new("0123456789.-,")?,
    )?;
    // let mut tes_oct = ocr::Tesseract::new(Some("./"), Some("eng"))?
    //     .set_variable("tessedit_char_whitelist", "0123456789.-,")?
    //     .set_image(path)?;
    let res = String::from_utf8_lossy(api.get_utf8_text()?.as_ref().to_bytes()).to_string();
    let mut points = Vec::new();
    for item in res.lines() {
        let a: Vec<f32> = item
            .split(',')
            .take(2)
            .map(str::parse)
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
        match (a.get(0), a.get(1)) {
            (Some(x), Some(y)) => points.push(Point::new(*x, *y)),
            _ => {}
        }
    }
    let res_json = serde_json::to_string(&points)
        .unwrap_or_default()
        .replace("\"x\"", "x")
        .replace("\"y\"", "y");
    Ok(res_json)
}
