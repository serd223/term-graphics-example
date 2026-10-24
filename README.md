# Terminal Graphics Example

This is an example project that uses the [Kitty Terminal Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/) to draw a software rendered 3D cube on the terminal, using actual raster graphics instead of ASCII characters.

I wasn't satistfied with any of the examples or libraries I found regarding this topic but I was pretty sure that this would be fairly easy to implement. I mainly wrote this example so I can come back to it later when I want to do some terminal graphics. If you just want to copy-paste the kitty graphics protocol part, see [kitty.rs](./src/kitty.rs).

## Requirements
  - [Rust & cargo](https://rustup.rs)
  - Terminal emulator that supports the Terminal Graphics Protocol. I personally use [kitty](https://sw.kovidgoyal.net/kitty/)

## Quick Start
```console
  $ cargo run --release
```

You can press `q` to quit the application. Ctrl-C won't work. Also, you probably will need to run this application outside of `tmux`.

