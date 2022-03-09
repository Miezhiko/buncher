use clap::Parser;

use std::str::FromStr;

#[derive(Parser, Debug, PartialEq)]
pub enum Operation {
  Flip,
  Mirror,
}

#[derive(Parser, Debug)]
pub enum Rotate {
  Rotate90,
  Rotate180,
  Rotate270,
}

#[derive(Parser, Debug)]
pub struct Size2D {
  pub width: u32,
  pub height: u32
}

impl FromStr for Size2D {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                             .split(',')
                             .collect();

    let x_fromstr = coords[0].parse::<u32>()?;
    let y_fromstr = coords[1].parse::<u32>()?;

    Ok(Size2D { width: x_fromstr, height: y_fromstr })
  }
}

impl FromStr for Rotate {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "90"  => Ok(Rotate::Rotate90),
      "180" => Ok(Rotate::Rotate180),
      "270" => Ok(Rotate::Rotate270),
      _     => Err(anyhow::anyhow!("Supported values: 90 180 270"))
    }
  }
}

impl FromStr for Operation {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> anyhow::Result<Self> {
    match s {
      "f"   => Ok(Operation::Flip),
      "m"   => Ok(Operation::Mirror),
      _   => Err(anyhow::anyhow!("Unknown operation"))
    }
  }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
  #[clap(short, long, required=true, forbid_empty_values=true, help="Source directory")]
  pub directory: String,

  #[clap(short, long, required=false, forbid_empty_values=true, help="Output directory")]
  pub output: Option<String>,

  #[clap(short, long, help="Remove duplicates")]
  pub clean: bool,

  #[clap(long, help="Additional operations")]
  pub additional: Vec<Operation>,

  #[clap(long, help="Flip images")]
  pub flip: bool,

  #[clap(long, help="Mirror images")]
  pub mirror: bool,

  #[clap(long, forbid_empty_values=true, help="Blur images")]
  pub blur: Option<f32>,

  #[clap(long, forbid_empty_values=true, help="Rotate images")]
  pub rotate: Option<Rotate>,

  #[clap(long, forbid_empty_values=true, help="Thumbnail images")]
  pub thumbnail: Option<Size2D>,

  #[clap(long, forbid_empty_values=true, help="Resize images")]
  pub resize: Option<Size2D>
}
