
use crate::{
  types::*,
  args::*,
  backends::{ imageio::process_img
            , videoio::process_vid }
};

#[cfg(feature = "zip")]
use crate::backends::zip::extract;

use std::{
  time::Duration,
  collections::{
    hash_map::Entry,
    HashMap
  }, fs, io, path::{ Path, PathBuf }
};

#[cfg(feature = "zip")]
use anyhow::Context;

use sha3::{ Digest, Sha3_256 };

use indicatif::{ ProgressBar, ProgressStyle };

use futures::future;

const EXTENSIONS: &str = "*.{jpg,jpeg,png,tiff,webp,mp4}";

pub async fn process(args: &mut Args) -> anyhow::Result<()> {
  let path = &args.directory;
  let mut seen_hashes = HashMap::new();
  let mut seen_hashes_videos = HashMap::new();

  let pb = ProgressBar::new_spinner();
  pb.enable_steady_tick(Duration::from_millis(120));
  pb.set_style(
    ProgressStyle::with_template("{spinner:.blue} {msg}")
      .unwrap()
      // For more spinners check out the cli-spinners project:
      // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
      .tick_strings(&[
        "▹▹▹▹▹",
        "▸▹▹▹▹",
        "▹▸▹▹▹",
        "▹▹▸▹▹",
        "▹▹▹▸▹",
        "▹▹▹▹▸",
        "▪▪▪▪▪",
      ])
  );

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

  pb.set_message("preparing");
  let (target_directory, new_target) = if let Some(target_dir) = &args.output {
    if Path::new(&target_dir).exists() {
      if args.clean {
        pb.set_message("cleaning target directory...");
        let walker = globwalk::GlobWalkerBuilder::from_patterns(
            target_dir, &[EXTENSIONS]
          ).max_depth(4)
           .follow_links(false)
           .build()?
           .filter_map(Result::ok);
        for entry in walker {
          let file_path = entry.path();
          if let Some(extension) = file_path.extension() {
            let mut file = fs::File::options().read(true)
                                              .open(file_path)?;
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
      ( Some(target_dir.as_str()), false )
    } else {
      async_fs::create_dir_all(&target_dir).await?;
      ( Some(target_dir.as_str()), true )
    }
  } else {
    ( None, false )
  };

  let nothing_todo =
    target_directory.is_none()
    && args.additional.is_empty()
    && args.blur.is_none()
    && args.brighten.is_none()
    && args.resize.is_none()
    && args.thumbnail.is_none()
    && args.rotate.is_none();

  #[cfg(feature = "zip")]
  {
    pb.set_message("unpacking");
    let zip_walker = globwalk::GlobWalkerBuilder::from_patterns(
      path, &["*.{zip}"]
    ).max_depth(4)
      .follow_links(false)
      .build()?
      .filter_map(Result::ok);
    for entry in zip_walker {
      let file_path = entry.path();
      let file_path_str = file_path.to_str()
                                   .context("can't get file_path")?;
      let file_stem = file_path.file_stem()
                               .context("no file stem")?
                               .to_str()
                               .context("file stem is not a string")?;
      let directory = file_path.parent()
                               .context("no parent path")?
                               .to_str()
                               .unwrap_or("");
      let new_filepath = format!("{directory}/{file_stem}");
      pb.set_message(format!("unzipping {new_filepath}"));
      extract(file_path_str, new_filepath.as_str());
      let path_to_remove: PathBuf = file_path.to_path_buf();
      tokio::spawn(async move {
        if let Err(why) = async_fs::remove_file(path_to_remove).await {
          println!("Error removing file {why}");
        }
      });
    }
  }

  let mut img_paths: Vec<PathBuf>   = vec![];
  let mut video_paths: Vec<PathBuf> = vec![];

  pb.set_message("collecting: file paths...");
  let mut duplicates_counter = if args.clean {
    Some(0)
  } else {
    None
  };

  let walker = globwalk::GlobWalkerBuilder::from_patterns(
      path, &[EXTENSIONS]
    ).max_depth(4)
     .follow_links(false)
     .build()?
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
              if let Some(counter) = &mut duplicates_counter {
                *counter += 1;
              }
              let path_to_remove: PathBuf = file_path.to_path_buf();
              tokio::spawn(async move {
                if let Err(why) = async_fs::remove_file(path_to_remove).await {
                  println!("Error removing file {why}");
                }
              });
              continue;
            }
          }
        }
        if !nothing_todo || args.separate_videos {
          video_paths.push(file_path.to_path_buf());
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
              if let Some(counter) = &mut duplicates_counter {
                *counter += 1;
              }
              let path_to_remove: PathBuf = file_path.to_path_buf();
              tokio::spawn(async move {
                if let Err(why) = async_fs::remove_file(path_to_remove).await {
                  println!("Error removing file {why}");
                }
              });
              continue;
            }
          }
        }
        if !nothing_todo {
          img_paths.push(file_path.to_path_buf());
        }
      }
    }
  }
  if let Some(counter) = duplicates_counter {
    pb.finish_with_message(format!("Done, removed {counter} duplicates"));
  } else {
    pb.finish_with_message("Done");
  }

  println!("processing videos");
  let pb_videos = ProgressBar::new(video_paths.len() as u64);
  pb_videos.set_style(
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, {percent}%, ETA {eta})",
    )
    .unwrap());
  for file_path in video_paths.into_iter() {
    if let Err(why) = process_vid( path
                                 , file_path
                                 , args
                                 , &target_directory
                                 ).await {
      println!("Error processing video: {why}");
      pb_videos.inc(1);
    }
  }
  pb_videos.finish();

  println!("processing images");
  let pb_images = ProgressBar::new(img_paths.len() as u64);
  pb_images.set_style(
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, {percent}%, ETA {eta})",
    )
    .unwrap());

  let mut image_processing_tasks = vec![];
  for file_path in img_paths.into_iter() {
    match process_img( path
                     , file_path
                     , args
                     , &target_directory
                     , &mut seen_hashes
                     , new_target
                     ).await {
      Ok(Some(task)) => {
        image_processing_tasks.push(task);
      }, Ok(None) => {
        // do nothing
      }, Err(why) => {
        println!("Error processing image: {why}");
      }
    }
    pb_images.inc(1);
  }
  pb_images.finish();

  if !args.one {
    pb.set_message("please wait for all threads to finish!");
    future::join_all(image_processing_tasks).await;
    pb.finish_with_message("OK");
  }

  Ok(())
}
