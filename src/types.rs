use clap::Parser;

#[derive(Parser, Debug, PartialEq, Eq, Clone)]
pub enum Operation {
  Flip,
  Mirror,
  Invert,
  Grayscale
}

#[derive(Parser, Debug, PartialEq, Eq, Clone)]
pub enum Rotate {
  Rotate90,
  Rotate180,
  Rotate270
}

#[derive(Parser, Debug, Clone)]
pub struct Size2D {
  pub width: u32,
  pub height: u32
}
