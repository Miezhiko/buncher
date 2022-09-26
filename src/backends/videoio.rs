use crate::args::*;

use std::path::Path;

use anyhow::Context;

const VIDEOS: &str = "../video";

pub async fn process_vid( input_dir: &str 
                        , f: &Path
                        , args: &Args
                        , target_dir: &Option<&str>
                        ) -> anyhow::Result<()> {
  if let Some(target) = target_dir {
    let fname = f.file_name().context("no fname")?
                             .to_str()
                             .unwrap_or_else(|| f.to_str().unwrap_or_default() );
    let directory = 
      if args.separate_videos {
        format!("{target}/{VIDEOS}")
      } else {
        f.parent()
         .context("no parent path")?
         .to_str()
         .unwrap_or("")
         .replace(input_dir, target)
      };
    if &directory != target && !Path::new(&directory).exists() {
      async_fs::create_dir_all(&directory).await?;
    }
    let mut new_path = format!("{directory}/{fname}");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fname}-{i}");
      i += 1;
    }
    async_fs::copy(f, &new_path).await?;
  } else if args.separate_videos {
    let old_path = f.parent()
                    .context("no parent path")?
                    .to_str()
                    .unwrap_or("");
    if old_path.contains(VIDEOS) {
      return Ok(());
    }
    let fname = f.file_name().context("no fname")?
                 .to_str()
                 .unwrap_or_else(|| f.to_str().unwrap_or_default() );
    let new_root = format!("{input_dir}/{VIDEOS}");
    if !Path::new(&new_root).exists() {
      async_fs::create_dir_all(&new_root).await?;
    }
    let mut new_path = format!("{new_root}/{fname}");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{new_root}/{fname}-{i}");
      i += 1;
    }
    async_fs::rename(f, &new_path).await?;
  }
  Ok(())
}
