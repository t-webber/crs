# Homeserver setup

## Customise

Specify your architecture and the domain name to use in the variables at the beginning of the `Makefile`.

## Run the server

```bash
# Create a venv
python -m venv venv
. ./venv/bin/activate

# Install dependencies
make install

# Run the Synapse homeserver
make start
```

## Mautrixes

Use another terminal to run the mautrixes, for instance:

```bash
make mautrix-whatsapp-run
```

- It is normal to get a few 502 errors when connecting to the Synapse server. Simply wait and the BEB should solve this.
- Make sure the homeserver is running before you start the mautrix.
