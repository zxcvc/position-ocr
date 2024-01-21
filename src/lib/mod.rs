use anyhow::{self};
use rusty_tesseract::{self as act, Args, Image};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, path};

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

pub fn points_from_img<P: Into<path::PathBuf>>(path: P) -> Result<String, anyhow::Error> {
    let img = Image::from_path(path)?;
    let act_arg = Args {
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "0123456789,.-".into(),
        )]),
        ..Default::default()
    };
    let res = act::image_to_string(&img, &act_arg)?;
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
    let res_json = serde_json::to_string(&points)?;
    Ok(res_json)
}
