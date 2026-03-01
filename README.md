# CRS - a TUI multiplatform chat service

![Clippy](https://github.com/t-webber/vim-buffer/actions/workflows/clippy.yml/badge.svg?branch=main)
![Build](https://github.com/t-webber/vim-buffer/actions/workflows/build.yml/badge.svg?branch=main)
![Rustdoc](https://github.com/t-webber/vim-buffer/actions/workflows/rustdoc.yml/badge.svg?branch=main)
![Rusfmt](https://github.com/t-webber/vim-buffer/actions/workflows/rustfmt.yml/badge.svg?branch=main)
![Taplo](https://github.com/t-webber/vim-buffer/actions/workflows/taplo.yml/badge.svg?branch=main)
![Spelling](https://github.com/t-webber/vim-buffer/actions/workflows/spelling.yml/badge.svg?branch=main)

[![github](https://img.shields.io/badge/GitHub-t--webber/crs-blue?logo=GitHub)](https://github.com/t-webber/vim-buffer)
[![license](https://img.shields.io/badge/Licence-MIT%20or%20Apache%202.0-darkgreen)](https://github.com/t-webber/vim-buffer?tab=MIT-2-ov-file)
[![coverage](https://img.shields.io/badge/Coverage-100%25-purple)](https://github.com/t-webber/vim-buffer/actions/workflows/nightly.yml)
[![rust-edition](https://img.shields.io/badge/Rust--edition-2024-darkred?logo=Rust)](https://doc.rust-lang.org/stable/edition-guide/rust-2024/)

## Useful links

- [matrix server with integrated bridge manager](https://github.com/beeper/bridge-manager)
- [a library for making nice TUIs](https://ratatui.rs/concepts/)
- [what are bridges?](https://docs.mau.fi/bridges/)

## Requirements

You will need to install those dependencies:

- `make`
- `tmux`
- [`uv`](https://docs.astral.sh/uv/getting-started/installation/)
- [`cargo`](https://rust-lang.org/tools/install/)

## Getting started

- If you are not on amd64, please edit the `ARCH` variable of `server/Makefile` with the appropriate value. You can also edit the domain name and other configuration options in `server/Makefile` if you desire. After changing the configuration, make sure to run `make clean`.
- Run the project:

```bash
cd server
./tmux.sh
```

- That's it!
