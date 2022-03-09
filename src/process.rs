use crate::args::Operation;

use image::ImageFormat;

use std::{
  fs::File,
  path::Path
};

pub fn process_file(f: &Path, op: &Operation) -> anyhow::Result<()> {
  let img = image::open(f)?;
  let fname = f.file_stem().unwrap()
                            .to_str()
                            .unwrap_or_else(|| f.to_str().unwrap());
  let mut output = File::create(&format!("{fname}-bunched.jpg"))?;
  let processed = match op {
    Operation::Resize => {
      img.thumbnail(500, 500)
    },
    Operation::Flip => {
      img.flipv()
    },
    Operation::Mirror => {
      img.fliph()
    },
    Operation::Rotate90 => {
      img.rotate90()
    },
    Operation::Rotate180 => {
      img.rotate180()
    },
    Operation::Blur => {
      img.blur(0.2)
    },
  };
  processed.write_to(&mut output, ImageFormat::Jpeg)?;
  Ok(())
}
