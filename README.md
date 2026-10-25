# Terminal Graphics Example

This repository contains two examples projects that use the [Kitty Terminal Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/) to draw **raster graphics** *on the terminal*.

 - [cube](./cube/): Draws a 3D cube on the terminal, using actual raster graphics instead of ASCII characters.
 - [termui](./termui/): Utilizes [microui](https://github.com/rxi/microui) and [cairo](https://crates.io/crates/cairo-rs) to show a simple UI on the terminal using raster graphics (Doesn't support all microui commands, just a proof of concept).

Most of the applications I saw that use the Terminal Graphics Protocol online were just fancy file previews or limited bitmap rendering. I was wondering if real time graphics would be possible using the Terminal Graphics Protocol so I decided to write these examples as a proof of concept and so that I can come back to them later if I ever want to do terminal graphics again. You can see the [kitty.rs](./cube/src/kitty.rs) source file if you just want to copy-paste the graphics protocol part.

## Requirements
  - [Rust & cargo](https://rustup.rs)
  - Terminal emulator that supports the Terminal Graphics Protocol. I personally use [kitty](https://sw.kovidgoyal.net/kitty/)

## Quick Start
You can press `q` to quit the application(s). Ctrl-C won't work. Also, you probably will need to run them outside of `tmux`.

For `cube`:
```console
  $ cargo run -p cube --release
```

For `termui`:
```console
  $ cargo run -p termui --release
```

