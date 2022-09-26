mod types;
mod args;
mod time;
mod buncher;
mod backends;

use args::*;
use time::*;

use clap::Parser;

use std::time::Instant;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let mut args  = Args::parse();
  let timer     = Instant::now();

  buncher::process(&mut args).await?;

  println!("Elapsed {}", Elapsed::from(&timer));
  Ok(())
}
