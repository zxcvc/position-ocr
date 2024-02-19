use anyhow::Ok;

use std::{
    fs,
    io::{BufWriter, Write},
    path,
};
pub fn get_images_from_dir<P: AsRef<path::Path>>(path: P) -> Option<Vec<String>> {
    let image_types: [&str; 3] = ["jpg", "jpeg", "png"];
    let dir = fs::read_dir(path).ok()?;
    let res = dir.filter_map(|item| {
        let entry = item.ok()?;
        let path = entry.path();
        let ext_name = path.extension()?.to_str()?;
        if image_types.contains(&ext_name) {
            Some(path.to_str()?.to_string())
        } else {
            None
        }
    });
    Some(res.collect())
}

pub fn save_as(file_name: &str, content: Vec<String>) -> Result<(), anyhow::Error> {
    if let Some(path) = rfd::FileDialog::new().set_file_name(file_name).save_file() {
        let file = fs::OpenOptions::new().create(true).write(true).open(path)?;
        let mut buf_writer = BufWriter::new(file);
        for line in content.iter() {
            writeln!(buf_writer, "{}", line);
        }
        buf_writer.flush();
    }

    Ok(())
}
