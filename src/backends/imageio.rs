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
  fs::File,
  path::{ Path, PathBuf },
  sync::Arc,
};

use tokio::task::JoinHandle;

use indicatif::ProgressBar;

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

async fn process( input_dir: String 
                , f: PathBuf
                , args: Args
                , target_dir: Option<String>
                , fstem: String
                , extension: String
                , pb_images: Arc<ProgressBar>
                ) -> anyhow::Result<()> {
  let mut img = image::open(&f)?;

  let mut output = if let Some(target) = target_dir {
    let directory = get_output_dir(&input_dir, &f, &target).await?;
    let mut new_path = format!("{directory}/{fstem}.{extension}");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fstem}-{i}.{extension}");
      i += 1;
    }
    File::options().write(true).create_new(true).open(&new_path)?
  } else {
    let new_path = format!("{fstem}-bunched.{extension}");
    if Path::new(&new_path).exists() {
      async_fs::remove_file(&new_path).await?;
    }
    File::options().write(true).create_new(true).open(&new_path)?
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
  if let Some(hue) = args.hue {
    img = img.huerotate(hue);
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
  if args.png {
    img.write_to(&mut output, ImageFormat::Png)?;
  } else {
    img.write_to(&mut output, ImageFormat::Jpeg)?;
  }
  pb_images.inc(1);
  Ok(())
}

pub async fn process_img( input_dir: &str 
                        , f: PathBuf
                        , args: &Args
                        , target_dir: &Option<&str>
                        , pb_images: &Arc<ProgressBar>
                        ) -> anyhow::Result
                                <Option
                                  <JoinHandle
                                    <Result<(), anyhow::Error>>
                                  >
                                >
                        {
 let fstem = f.file_stem().context("no file stem")?
              .to_str()
              .context("file stem is not a string")?;
  let extension: &str = if args.png {
      "png"
    } else {
      "jpg"
    };
  if let Some(ignore_mask) = &args.ignore {
    if fstem.contains(ignore_mask) {
      if let Some(target) = target_dir {
        let fname = f.file_name().context("no file name")?
                     .to_str().context("file name is not a string")?;
        let directory = get_output_dir(input_dir, &f, target).await?;
        let mut new_path = format!("{directory}/{fname}");
        let mut i = 1;
        while Path::new(&new_path).exists() {
          new_path = format!("{directory}/{fname}-{i}.{extension}");
          i += 1;
        }
        async_fs::copy(&f, new_path).await?;
      }
      return Ok(None);
    }
  }
  let t_dir = target_dir.as_ref().map(|target| target.to_string());
  let pb_clone = Arc::clone(pb_images);
  Ok(Some(
    tokio::spawn(
      process( input_dir.to_string()
            , f.clone()
            , args.clone()
            , t_dir
            , fstem.to_string()
            , extension.to_string()
            , pb_clone
            )
  )))
}
