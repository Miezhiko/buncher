use crate::args::*;

use image::{
  ImageFormat,
  imageops::FilterType
};

use std::{
  fs::File,
  path::Path
};

pub fn process_file( f: &Path
                   , args: &Args
                   , target_dir: &Option<&str>
                   ) -> anyhow::Result<()> {
  let mut img = image::open(f)?;
  let mut output = if let Some(target) = target_dir {
    let fname = f.file_name().unwrap()
                             .to_str()
                             .unwrap_or_else(|| f.to_str().unwrap());
    File::create(&format!("{target}/{fname}.jpg"))?
  } else {
    let fname = f.file_stem().unwrap()
                             .to_str()
                             .unwrap_or_else(|| f.to_str().unwrap());
    File::create(&format!("{fname}-bunched.jpg"))?
  };
  for op in &args.additional {
    match op {
      Operation::Flip       => img = img.flipv(),
      Operation::Mirror     => img = img.fliph(),
      Operation::Invert     => img.invert(),
      Operation::Grayscale  => img = img.grayscale()
    };
  }
  if let Some(blur) = args.blur {
    img = img.blur(blur);
  }
  if let Some(brighten) = args.brighten {
    img = img.brighten(brighten);
  }
  if let Some(resize) = &args.resize {
    img = img.resize(resize.width, resize.height, FilterType::Nearest);
  }
  if let Some(thumbnail) = &args.thumbnail {
    img = img.thumbnail(thumbnail.width, thumbnail.height);
  }
  if let Some(rotate) = &args.rotate {
    img = match rotate {
      Rotate::Rotate90  => img.rotate90(),
      Rotate::Rotate180 => img.rotate180(),
      Rotate::Rotate270 => img.rotate270()
    }
  }
  img.write_to(&mut output, ImageFormat::Jpeg)?;
  Ok(())
}
