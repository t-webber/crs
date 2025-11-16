# CRS - a TUI multiplatform chat service

## Useful links

- [matrix server with integrated bridge manager](https://github.com/beeper/bridge-manager)
- [a library for making nice TUIs](https://ratatui.rs/concepts/)
- [what are bridges?](https://docs.mau.fi/bridges/)

## Getting started

First, you need to setup the server. Refer to [Homeserver setup](#homeserver-setup).

Then, from the root of the repo, run `cargo run` and the TUI should start.

## Homeserver setup

### Configuration

If you are not on amd64, please edit the `ARCH` variable of the Makefile with the appropriate value.

You can also edit the domain name and other configuration options in the `Makefile` if you desire.

Please do this configuration before running `make install`. To apply a new configuration, you will need to discard all your data with `make clean`.

### Run the server

```bash
# Create a venv
python -m venv venv
. ./venv/bin/activate

# Install dependencies
make install

# Run the Synapse homeserver
make start
```

> ![IMPORTANT]
>
> Keep the terminal when you ran `make start` running for the rest of the homeserver setup.
>
> Furthermore, all the following commands need to be run inside the python `venv`.

### Mautrixes

Use another terminal to run the mautrixes, for instance:

```bash
make mautrix-whatsapp-run
```

> ![TIP]
>
> It is normal to get a few **HTTP 502** errors when connecting to the Synapse server. Simply wait and the BEB should solve this.

### Create a user

To create a user, run this command

```bash
make createuser
# Enter your username and password
```

### Save your credentials

Create a `.env` at the root of the repo with the following entries:

```env
HOMESERVER_URL=http://localhost:8008
USERNAME=bob    # Username given to `make createuser` from step 1.
PASSWORD=passwd # Password given to `make createuser` from step 1.
```
