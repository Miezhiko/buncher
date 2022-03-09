mod args;
mod time;
mod process;

use args::*;
use time::*;
use process::process_file;

use clap::Parser;
use glob::glob;

use std::time::Instant;

fn main() -> anyhow::Result<()> {
  let args = Args::parse();
  let timer = Instant::now();
  let path = args.directory;

  let target_directory = if let Some(target_dir) = &args.target {
    std::fs::create_dir_all(target_dir)?;
    Some(target_dir.as_str())
  } else {
    None
  };

  for f in glob(&format!("{path}/*.jpg"))?
    .chain(glob(&format!("{path}/*.png"))?)
    .chain(glob(&format!("{path}/*.tiff"))?) {
    match f {
      Ok(file_path) => {
        println!("processing: {}", file_path.display());
        process_file(&file_path, &args.operations, &target_directory)?;
      }
      Err(e) => {
        eprintln!("ERROR: {}", e);
      }
    };
  }

  println!("Elapsed {}", Elapsed::from(&timer));
  Ok(())
}
