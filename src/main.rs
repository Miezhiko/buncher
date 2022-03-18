mod args;
mod time;
mod imageio;
mod videoio;

use args::*;
use time::*;
use imageio::process_img;
use videoio::process_vid;

use clap::Parser;
use glob::glob;

use std::{
  time::Instant,
  collections::{
    hash_map::Entry,
    HashMap
  }, fs, io
};

use sha3::{Digest, Sha3_256};

fn main() -> anyhow::Result<()> {
  let mut args = Args::parse();
  let timer = Instant::now();
  let path = &args.directory;

  let target_directory = if let Some(target_dir) = &args.output {
    fs::create_dir_all(target_dir)?;
    Some(target_dir.as_str())
  } else {
    None
  };

  let mut seen_hashes = HashMap::new();

  args.additional.dedup();

  if args.flip && !args.additional.contains(&Operation::Flip) {
    args.additional.push(Operation::Flip);
  }
  if args.mirror && !args.additional.contains(&Operation::Mirror) {
    args.additional.push(Operation::Mirror);
  }
  if args.grayscale && !args.additional.contains(&Operation::Grayscale) {
    args.additional.push(Operation::Grayscale);
  }
  if args.invert && !args.additional.contains(&Operation::Invert) {
    args.additional.push(Operation::Invert);
  }

  for f in glob(&format!("{path}/*.jpg"))?
    .chain(glob(&format!("{path}/*.png"))?)
    .chain(glob(&format!("{path}/*.tiff"))?) {
    match f {
      Ok(file_path) => {
        println!("processing: {}", file_path.display());
        if args.clean {
          let mut file = fs::File::options().read(true).open(&file_path)?;
          let mut hasher = Sha3_256::new();
          io::copy(&mut file, &mut hasher)?;
          let hash = hasher.finalize();
          match seen_hashes.entry(hash) {
            Entry::Vacant(map) => {
              map.insert(file_path.clone());
            },
            Entry::Occupied(_map) => {
              println!("removing as duplication");
              fs::remove_file(&file_path)?;
              continue;
            }
          }
        }
        process_img(path, &file_path, &args, &target_directory)?;
      }
      Err(e) => {
        eprintln!("ERROR: {}", e);
      }
    };
  }

  for f in glob(&format!("{path}/*.mp4"))? {
    match f {
      Ok(file_path) => {
        println!("processing: {}", file_path.display());
        if args.clean {
          let mut file = fs::File::options().read(true).open(&file_path)?;
          let mut hasher = Sha3_256::new();
          io::copy(&mut file, &mut hasher)?;
          let hash = hasher.finalize();
          match seen_hashes.entry(hash) {
            Entry::Vacant(map) => {
              map.insert(file_path.clone());
            },
            Entry::Occupied(_map) => {
              println!("removing as duplication");
              fs::remove_file(&file_path)?;
              continue;
            }
          }
        }
        process_vid(path, &file_path, &args, &target_directory)?;
      }
      Err(e) => {
        eprintln!("ERROR: {}", e);
      }
    };
  }

  println!("Elapsed {}", Elapsed::from(&timer));
  Ok(())
}
