use crate::args::*;

use std::path::Path;

use anyhow::Context;

pub async fn process_vid( input_dir: &str 
                        , f: &Path
                        , _args: &Args
                        , target_dir: &Option<&str>
                        ) -> anyhow::Result<()> {
  if let Some(target) = target_dir {
    let fname = f.file_name().context("no fname")?
                             .to_str()
                             .unwrap_or_else(|| f.to_str().unwrap_or_default() );
    let directory = f.parent()
                     .context("no parent path")?
                     .to_str()
                     .unwrap_or("")
                     .replace(input_dir, target);
    if &directory != target && !Path::new(&directory).exists() {
      async_fs::create_dir_all(&directory).await?;
    }
    let mut new_path = format!("{directory}/{fname}.mp4");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fname}-{i}.mp4");
      i += 1;
    }
    async_fs::copy(f, &new_path).await?;
  }

  Ok(())
}
