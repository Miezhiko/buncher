use clap::Parser;

use std::str::FromStr;

#[derive(Parser, Debug)]
pub enum Operation {
  Resize,
  Flip,
  Mirror,
  Rotate90,
  Rotate180,
  Blur
}

impl FromStr for Operation {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "r"   => Ok(Operation::Resize),
      "f"   => Ok(Operation::Flip),
      "m"   => Ok(Operation::Mirror),
      "90"  => Ok(Operation::Rotate90),
      "180" => Ok(Operation::Rotate180),
      "b"   => Ok(Operation::Blur),
      _   => Err(anyhow::anyhow!("Unknown operation"))
    }
  }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
  #[clap(short, long)]
  pub directory: String,

  #[clap(short, long)]
  pub operation: Operation
}
