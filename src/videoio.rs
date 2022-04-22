use crate::{
  types::*,
  args::*
};

use std::{
  fs, io,
  path::Path,
  collections::hash_map::Entry
};

use anyhow::Context;
use sha3::{ Digest, Sha3_256 };

pub async fn process_vid( input_dir: &str 
                        , f: &Path
                        , args: &Args
                        , target_dir: &Option<&str>
                        , seen_hashes: &mut SHA256
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

    if args.clean {
      let mut file = fs::File::options().read(true).open(&new_path)?;
      let mut hasher = Sha3_256::new();
      io::copy(&mut file, &mut hasher)?;
      let hash = hasher.finalize();
      match seen_hashes.entry(hash) {
        Entry::Vacant(map) => {
          map.insert(Path::new(&new_path).to_path_buf());
        },
        Entry::Occupied(_map) => {
          println!("removing duplication in target path {}", &new_path);
          async_fs::remove_file(&new_path).await?;
        }
      }
    }
  }

  Ok(())
}
