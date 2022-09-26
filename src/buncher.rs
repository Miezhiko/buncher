
use crate::{
  types::*,
  args::*,
  backends::{ imageio::process_img
            , videoio::process_vid }
};

use std::{
  collections::{
    hash_map::Entry,
    HashMap
  }, fs, io, path::Path
};

use sha3::{ Digest, Sha3_256 };

pub async fn process(args: &mut Args) -> anyhow::Result<()> {
  let path = &args.directory;
  let mut seen_hashes = HashMap::new();
  let mut seen_hashes_videos = HashMap::new();

  let target_directory = if let Some(target_dir) = &args.output {
    if Path::new(&target_dir).exists() {
      if args.clean {
        let walker = globwalk::GlobWalkerBuilder::from_patterns(
            target_dir, &["*.{jpg, png, tiff, mp4}"]
          ).max_depth(4)
           .follow_links(false)
           .build()?
           .into_iter()
           .filter_map(Result::ok);
        for entry in walker {
          let file_path = entry.path();
          if let Some(extension) = file_path.extension() {
            let mut file = fs::File::options().read(true).open(file_path)?;
            let mut hasher = Sha3_256::new();
            io::copy(&mut file, &mut hasher)?;
            let hash = hasher.finalize();
            if extension == "mp4" {
              match seen_hashes_videos.entry(hash) {
                Entry::Vacant(map) => {
                  map.insert(file_path.to_path_buf());
                },
                Entry::Occupied(_map) => {
                  println!("removing duplication video in target path {}", file_path.as_os_str().to_str().unwrap_or(""));
                  async_fs::remove_file(&file_path).await?;
                  continue;
                }
              }
            } else {
              match seen_hashes.entry(hash) {
                Entry::Vacant(map) => {
                  map.insert(file_path.to_path_buf());
                },
                Entry::Occupied(_map) => {
                  println!("removing duplication image in target path {}", file_path.as_os_str().to_str().unwrap_or(""));
                  async_fs::remove_file(&file_path).await?;
                  continue;
                }
              }
            }
          }
        }
      }
    } else {
      async_fs::create_dir_all(&target_dir).await?;
    }
    Some(target_dir.as_str())
  } else {
    None
  };

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

  let nothing_todo =
    target_directory.is_none()
    && args.additional.is_empty()
    && args.blur.is_none()
    && args.brighten.is_none()
    && args.resize.is_none()
    && args.thumbnail.is_none()
    && args.rotate.is_none();

  let walker = globwalk::GlobWalkerBuilder::from_patterns(
      path, &["*.{jpg, png, tiff, mp4}"]
    ).max_depth(4)
     .follow_links(false)
     .build()?
     .into_iter()
     .filter_map(Result::ok);
  for entry in walker {
    let file_path = entry.path();
    if let Some(extension) = file_path.extension() {
      if extension == "mp4" {
        if args.clean {
          let mut file = fs::File::options().read(true).open(file_path)?;
          let mut hasher = Sha3_256::new();
          io::copy(&mut file, &mut hasher)?;
          let hash = hasher.finalize();
          match seen_hashes_videos.entry(hash) {
            Entry::Vacant(map) => {
              map.insert(file_path.to_path_buf());
            },
            Entry::Occupied(_map) => {
              println!("removing as duplication video {}", file_path.as_os_str().to_str().unwrap_or(""));
              async_fs::remove_file(&file_path).await?;
              continue;
            }
          }
        }
        if !nothing_todo || args.separate_videos {
          println!("processing: {}", file_path.display());
          process_vid(path, file_path, args, &target_directory).await?;
        }
      } else {
        if args.clean {
          let mut file = fs::File::options().read(true).open(file_path)?;
          let mut hasher = Sha3_256::new();
          io::copy(&mut file, &mut hasher)?;
          let hash = hasher.finalize();
          match seen_hashes.entry(hash) {
            Entry::Vacant(map) => {
              map.insert(file_path.to_path_buf());
            },
            Entry::Occupied(_map) => {
              println!("removing as duplication image {}", file_path.as_os_str().to_str().unwrap_or(""));
              async_fs::remove_file(&file_path).await?;
              continue;
            }
          }
        }
        if !nothing_todo {
          println!("processing: {}", file_path.display());
          process_img(path, file_path, args, &target_directory, &mut seen_hashes).await?;
        }
      }
    }
  }

  Ok(())
}
