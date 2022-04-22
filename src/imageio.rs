use crate::{
  types::*,
  args::*
};

use anyhow::Context;
use image::{
  ImageFormat,
  imageops::FilterType
};

use std::{
  collections::hash_map::Entry,
  fs::File,
  path::Path,
  io
};

use sha3::{ Digest, Sha3_256 };

async fn get_output_dir( input_dir: &str
                       , f: &Path
                       , target: &str
                       ) -> anyhow::Result<String> {
  let directory = f.parent()
                   .context("no parent path")?
                   .to_str()
                   .unwrap_or("")
                   .replace(input_dir, target);
  if directory != target && !Path::new(&directory).exists() {
    async_fs::create_dir_all(&directory).await?;
  }
  Ok(directory)
}

pub async fn process_img( input_dir: &str 
                        , f: &Path
                        , args: &Args
                        , target_dir: &Option<&str>
                        , seen_hashes: &mut SHA256
                        ) -> anyhow::Result<()> {
 let fstem = f.file_stem().context("no file stem")?
                          .to_str()
                          .context("file stem is not a string")?;
  if let Some(ignore_mask) = &args.ignore {
    if fstem.contains(ignore_mask) {
      if let Some(target) = target_dir {
        let fname = f.file_name().context("no file name")?
                     .to_str().context("file name is not a string")?;
        let directory = get_output_dir(input_dir, f, target).await?;
        let mut new_path = format!("{directory}/{fname}");
        let mut i = 1;
        while Path::new(&new_path).exists() {
          new_path = format!("{directory}/{fname}-{i}.mp4");
          i += 1;
        }
        async_fs::copy(f, new_path).await?;
      }
      return Ok(());
    }
  }
  let mut img = image::open(f)?;
  let mut new_path = String::new();
  let mut output = if let Some(target) = target_dir {
    let directory = get_output_dir(input_dir, f, target).await?;
    new_path = format!("{directory}/{fstem}.jpg");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fstem}-{i}.jpg");
      i += 1;
    }
    File::options().read(true).write(true).create_new(true).open(&new_path)?
  } else {
    let new_path = format!("{fstem}-bunched.jpg");
    if Path::new(&new_path).exists() {
      async_fs::remove_file(&new_path).await?;
    }
    File::options().read(true).write(true).create_new(true).open(&new_path)?
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
  if args.clean && !new_path.is_empty() {
    let mut hasher = Sha3_256::new();
    io::copy(&mut output, &mut hasher)?;
    let hash = hasher.finalize();
    match seen_hashes.entry(hash) {
      std::collections::hash_map::Entry::Vacant(map) => {
        map.insert(Path::new(&new_path).to_path_buf());
      },
      Entry::Occupied(_map) => {
        println!("removing duplication in target path {}", &new_path);
        async_fs::remove_file(&new_path).await?;
      }
    }
  }
  Ok(())
}
