use crate::args::*;

use std::{
  fs, io,
  path::{ Path, PathBuf },
  collections::{ HashMap, hash_map::Entry }
};

use generic_array::GenericArray;
use typenum::{ UInt, UTerm, B1, B0 };

use sha3::{Digest, Sha3_256};

pub fn process_vid( input_dir: &str 
                  , f: &Path
                  , args: &Args
                  , target_dir: &Option<&str>
                  , seen_hashes: &mut HashMap< GenericArray< u8
                                                           , UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0> >
                                             , PathBuf >
                  ) -> anyhow::Result<()> {
  if let Some(target) = target_dir {
    let fname = f.file_name().unwrap()
                             .to_str()
                             .unwrap_or_else(|| f.to_str().unwrap());
    let directory = f.parent()
                     .unwrap()
                     .to_str()
                     .unwrap_or("")
                     .replace(input_dir, target);
    if &directory != target && !Path::new(&directory).exists() {
      fs::create_dir_all(&directory)?;
    }
    let mut new_path = format!("{directory}/{fname}.mp4");
    let mut i = 1;
    while Path::new(&new_path).exists() {
      new_path = format!("{directory}/{fname}-{i}.mp4");
      i += 1;
    }
    fs::copy(f, &new_path)?;

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
          fs::remove_file(&new_path)?;
        }
      }
    }
  }

  Ok(())
}
