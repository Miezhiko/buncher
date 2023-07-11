use std::path::{ Path, PathBuf };
use std::ffi::OsStr;

use anyhow::Context;

use futures_lite::stream::StreamExt;

const VIDEOS_LIB: &str = "/video";

async fn is_dir_empty(path: &str) -> bool {
  if let Ok(mut dir) = async_fs::read_dir(path).await {
    return dir.next().await.is_none();
  }
  false
}

fn separate_filename(filename: &str, for_video: bool) -> (&str, &str) {
  let path = Path::new(filename);

  let name = path.file_stem().and_then(OsStr::to_str).unwrap_or("noname");
  let extension = path.extension().and_then(OsStr::to_str)
                      .unwrap_or(
                        if for_video {
                          "mp4"
                        } else {
                          "png"
                        }
                      );

  (name, extension)
}

pub async fn generate_library
  ( input_dir: &str 
  , videos: Vec<PathBuf>
  , images: Vec<PathBuf>
  ) -> anyhow::Result<()> {

  let videos_path = format!("{input_dir}{VIDEOS_LIB}");

  if !Path::new(&videos_path).exists() {
    async_fs::create_dir_all(&videos_path).await?;
  }

  for f in videos.iter() {
    let parent_path = f.parent()
                       .context("no parent path")?
                       .to_str()
                       .unwrap_or("");

    let fname = f.file_name().context("no fname")?
                 .to_str()
                 .unwrap_or_else(|| f.to_str().unwrap_or_default() );

    let uname_parts: Vec<_> = fname.split(&['-', '.']).collect();
    let u = uname_parts.first().unwrap_or(&fname);
    let new_dir = format!("{videos_path}/{u}");

    if !Path::new(&new_dir).exists() {
      async_fs::create_dir_all(&new_dir).await?;
    }

    let mut new_path = format!("{new_dir}/{fname}");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      let (name, extension) = separate_filename(fname, true);
      new_path = format!("{new_dir}/{name}-{i}.{extension}");
      i += 1;
    }
    async_fs::rename(f, &new_path).await?;

    if is_dir_empty(parent_path).await {
      async_fs::remove_dir_all(parent_path).await?;
    }
  }

  for f in images.iter() {
    let parent_path = f.parent()
                       .context("no parent path")?
                       .to_str()
                       .unwrap_or("");

    let fname = f.file_name().context("no fname")?
                 .to_str()
                 .unwrap_or_else(|| f.to_str().unwrap_or_default() );

    let uname_parts: Vec<_> = fname.split(&['-', '.']).collect();
    let u = uname_parts.first().unwrap_or(&fname);
    let new_dir = format!("{input_dir}/{u}");

    if !Path::new(&new_dir).exists() {
      async_fs::create_dir_all(&new_dir).await?;
    }

    let mut new_path = format!("{new_dir}/{fname}");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      let (name, extension) = separate_filename(fname, false);
      new_path = format!("{new_dir}/{name}-{i}.{extension}");
      i += 1;
    }
    async_fs::rename(f, &new_path).await?;

    if is_dir_empty(parent_path).await {
      async_fs::remove_dir_all(parent_path).await?;
    }
  }

  Ok(())
}
