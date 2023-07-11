use crate::types::*;

use std::str::FromStr;

use clap::Parser;

impl FromStr for Size2D {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> anyhow::Result<Self> {
    let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                             .split(',')
                             .collect();

    if coords.len() > 1 {
      let x_fromstr = coords[0].trim().parse::<u32>()?;
      let y_fromstr = coords[1].trim().parse::<u32>()?;
      Ok(Size2D { width: x_fromstr, height: y_fromstr })
    } else {
      Err(anyhow::anyhow!("you must specify height and weight (h, v)"))
    }
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
    match s.to_lowercase().as_str() {
      "flip"      => Ok(Operation::Flip),
      "mirror"    => Ok(Operation::Mirror),
      "invert"    => Ok(Operation::Invert),
      "grayscale" => Ok(Operation::Grayscale),
      _           => Err(anyhow::anyhow!("Unknown operation"))
    }
  }
}

#[derive(Parser, Debug, Clone)]
#[clap(version, about, long_about = None)]
pub struct Args {
  #[clap(short, long, required=true, help="Source directory")]
  pub directory: String,

  #[clap(short, long, required=false, help="Output directory")]
  pub output: Option<String>,

  #[clap(short, long, help="Remove duplicates")]
  pub clean: bool,

  #[clap(short, long, help="Separate videos to different directory")]
  pub separate_videos: bool,

  #[clap(long, help="Additional operations")]
  pub additional: Vec<Operation>,

  #[clap(long, help="Flip images")]
  pub flip: bool,

  #[clap(long, help="Mirror images")]
  pub mirror: bool,

  #[clap(long, help="Invert images")]
  pub invert: bool,

  #[clap(long, help="Grayscale images")]
  pub grayscale: bool,

  #[clap(long, help="Store as png instead of jpg")]
  pub png: bool,

  #[clap(long, help="Blur images")]
  pub blur: Option<f32>,

  #[clap(long, help="Brighten images")]
  pub brighten: Option<i32>,

  #[clap(long, help="Rotate images")]
  pub rotate: Option<Rotate>,

  #[clap(long, help="Thumbnail images")]
  pub thumbnail: Option<Size2D>,

  #[clap(long, help="Resize images")]
  pub resize: Option<Size2D>,

  #[clap(short, long, required=false, help="Ignore image by name mask")]
  pub ignore: Option<String>,

  #[clap(long, required=false, help="Single thread")]
  pub one: bool,

  #[clap(long, required=false, help="Restructure local library")]
  pub library: bool
}

#[cfg(test)]
mod args_tests {
  use super::*;
  #[test]
  fn size_parsing() -> anyhow::Result<()> {
    let size1 = Size2D::from_str("1,5")?;
    assert_eq!(size1.width, 1);
    assert_eq!(size1.height, 5);

    let size2 = Size2D::from_str("(2,6)")?;
    assert_eq!(size2.width, 2);
    assert_eq!(size2.height, 6);

    let size3 = Size2D::from_str("3, 7")?;
    assert_eq!(size3.width, 3);
    assert_eq!(size3.height, 7);

    Ok(())
  }

  #[test]
  fn rotate_parsing() -> anyhow::Result<()> {
    let rotate1 = Rotate::from_str("180")?;
    assert_eq!(rotate1, Rotate::Rotate180);
    Ok(())
  }

  #[test]
  fn operation_parsing() -> anyhow::Result<()> {
    let operation1 = Operation::from_str("grayscale")?;
    assert_eq!(operation1, Operation::Grayscale);
    Ok(())
  }
}
