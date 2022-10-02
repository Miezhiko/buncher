use std::{
  collections::HashMap,
  path::PathBuf
};

use generic_array::GenericArray;
use typenum::{ UInt, UTerm, B1, B0 };

use clap::Parser;

pub type SHA256 = HashMap< GenericArray< u8
                         , UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0> >
                         , PathBuf >;

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
