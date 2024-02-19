use anyhow::{self};
use ocr::plumbing::leptonica_plumbing::{memory::LeptonicaDestroy, Pix};
// use rusty_tesseract::{self as act, Args, Image};
use serde::{Deserialize, Serialize};

use std::{ffi::CString, fs, io::Read};
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

pub struct OcrApi {
    pub api: ocr::plumbing::TessBaseApi,
}

impl OcrApi {
    pub fn new(data: &'static [u8]) -> Self {
        let mut api = ocr::plumbing::TessBaseApi::default();
        api.init_1(
            data,
            Some(&CString::new("eng").unwrap()),
            ocr::OcrEngineMode::TesseractOnly as i32,
        )
        .unwrap();
        api.set_variable(
            &CString::new("tessedit_char_whitelist").unwrap(),
            &CString::new("0123456789.-,").unwrap(),
        )
        .unwrap();

        Self { api }
    }

    pub fn get_text(&mut self, path: &str) -> Result<String, anyhow::Error> {
        let data = fs::read(path)?;
        let mut origin_pix = Pix::read_mem(&data)?;
        let mut pix = unsafe {
            let pix = ocr::plumbing::leptonica_plumbing::leptonica_sys::pixConvertRGBToGray(
                origin_pix.as_mut(),
                0.30,
                0.59,
                0.11,
            );
            Pix::new_from_pointer(pix)
        };

        self.api.set_image_2(&pix);
        let res =
            String::from_utf8_lossy(self.api.get_utf8_text()?.as_ref().to_bytes()).to_string();
        unsafe {
            self.api.set_image(&[], 0, 0, 0, 0)?;
            origin_pix.destroy();
            pix.destroy();
        }

        let mut points = Vec::new();
        for item in res.lines() {
            let a: Vec<f32> = item
                .split(',')
                .take(2)
                .map(str::parse)
                .filter(Result::is_ok)
                .map(Result::unwrap)
                .collect();
            match (a.first(), a.get(1)) {
                (Some(&x), Some(&y)) => points.push(Point::new(x, y)),
                _ => {}
            }
        }
        let res_json = serde_json::to_string(&points)
            .unwrap_or_default()
            .replace("\"x\"", "x")
            .replace("\"y\"", "y");
        Ok(res_json)
    }
}
