use crate::args::*;

use image::{
  ImageFormat,
  imageops::FilterType
};

use std::{
  fs::{ File, self },
  path::Path
};

fn get_output_dir( input_dir: &str
                 , f: &Path
                 , target: &str
                 ) -> anyhow::Result<String> {
  let directory = f.parent()
                    .unwrap()
                    .to_str()
                    .unwrap_or("")
                    .replace(input_dir, target);
  if directory != target && !Path::new(&directory).exists() {
    fs::create_dir_all(&directory)?;
  }
  Ok(directory)
}

pub fn process_img( input_dir: &str 
                  , f: &Path
                  , args: &Args
                  , target_dir: &Option<&str>
                  ) -> anyhow::Result<()> {
 let fstem = f.file_stem().unwrap()
                          .to_str()
                          .unwrap();
  if let Some(ignore_mask) = &args.ignore {
    if fstem.contains(ignore_mask) {
      if let Some(target) = target_dir {
        let fname = f.file_name().unwrap()
                     .to_str().unwrap();
        let directory = get_output_dir(input_dir, f, target)?;
        let mut new_path = format!("{directory}/{fname}");
        let mut i = 1;
        while Path::new(&new_path).exists() {
          new_path = format!("{directory}/{fname}-{i}.mp4");
          i += 1;
        }
        fs::copy(f, new_path)?;
      }
      return Ok(());
    }
  }
  let mut img = image::open(f)?;
  let mut output = if let Some(target) = target_dir {
    let directory = get_output_dir(input_dir, f, target)?;
    let mut new_path = format!("{directory}/{fstem}.jpg");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fstem}-{i}.jpg");
      i += 1;
    }
    File::create(&new_path)?
  } else {
    let new_path = format!("{fstem}-bunched.jpg");
    if Path::new(&new_path).exists() {
      fs::remove_file(&new_path)?;
    }
    File::create(&new_path)?
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
