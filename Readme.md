<h1 align="center">
  buncher
  <br>
</h1>

<h3> images (and maybe videos) directory bunch parallel processing, supporting zips </h3>

<p align="center">
  <a href="#features">About</a>
  •
  <a href="#notes">Notes</a>
</p>

[![mawa](https://github.com/Miezhiko/buncher/actions/workflows/mawa.yml/badge.svg)](https://github.com/Miezhiko/buncher/actions/workflows/mawa.yml)
[![Discord](https://img.shields.io/discord/611822838831251466?label=Discord&color=pink)](https://discord.gg/GdzjVvD)
[![Twitter Follow](https://img.shields.io/twitter/follow/Miezhiko.svg?style=social)](https://twitter.com/Miezhiko)

## About

```bash
Usage:

Options:
  -d, --directory <DIRECTORY>    Source directory
  -o, --output <OUTPUT>          Output directory
  -c, --clean                    Remove duplicates
  -s, --separate-videos          Separate videos to different directory
      --additional <ADDITIONAL>  Additional operations
      --flip                     Flip images
      --mirror                   Mirror images
      --invert                   Invert images
      --grayscale                Grayscale images
      --png                      Store as png instead of jpg
      --blur <BLUR>              Blur images
      --brighten <BRIGHTEN>      Brighten images
      --rotate <ROTATE>          Rotate images
      --thumbnail <THUMBNAIL>    Thumbnail images
      --resize <RESIZE>          Resize images
  -i, --ignore <IGNORE>          Ignore image by name mask
  -h, --help                     Print help information
  -V, --version                  Print version information
```

for example:

```bash
buncher> cargo run --release -- --directory input --output output --grayscale --clean --ignore my
   Compiling buncher v0.3.7 (buncher)
    Finished release [optimized] target(s) in 54.44s
     Running `target/release/buncher --directory input --output output --grayscale --clean --ignore my`
▪▪▪▪▪ Done, removed 67 duplicates
processing videos
  [00:00:00] [████████████████████████████████████████] (152/152, 100%, ETA 0s)
processing images
⠒ [00:02:32] [██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] (182/2928, 6%, ETA 41m)    
```

`RESIZE` example is `(64,64)`

possible `ROTATE` variants are `90`, `180`, `270`

## Notes

*Supported list of additional operations:*

 - flip
 - mirror
 - invert
 - grayscale
 - ...

**this is very early version**
