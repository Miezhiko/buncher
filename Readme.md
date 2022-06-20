images (and maybe videos) directory bunch processing app
========================================================

[![mawa](https://github.com/Miezhiko/buncher/actions/workflows/mawa.yml/badge.svg)](https://github.com/Miezhiko/buncher/actions/workflows/mawa.yml)

```bash
buncher 0.2.7

USAGE:
    buncher [OPTIONS] --directory <DIRECTORY>

OPTIONS:
        --additional <ADDITIONAL>    Additional operations
        --blur <BLUR>                Blur images
        --brighten <BRIGHTEN>        Brighten images
    -c, --clean                      Remove duplicates
    -d, --directory <DIRECTORY>      Source directory
        --flip                       Flip images
        --grayscale                  Grayscale images
    -h, --help                       Print help information
    -i, --ignore <IGNORE>            Ignore image by name mask
        --invert                     Invert images
        --mirror                     Mirror images
    -o, --output <OUTPUT>            Output directory
        --png                        Store as png instead of jpg
        --resize <RESIZE>            Resize images
        --rotate <ROTATE>            Rotate images
    -s, --separate-videos            Separate videos to different directory
        --thumbnail <THUMBNAIL>      Thumbnail images
    -V, --version                    Print version information
```

for example:

```bash
cargo run --release -- --directory input --output output --grayscale --clean --ignore my
```

`RESIZE` example is `(64,64)`

possible `ROTATE` variants are `90`, `180`, `270`

Supported list of additional operations:
----------------------------------------

 - flip
 - mirror
 - invert
 - grayscale
 - ...

**this is very early version**
