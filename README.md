# CRS - a TUI multiplatform chat service

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
