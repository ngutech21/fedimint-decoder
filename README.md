# Fedimint Decoder [![Build](https://github.com/ngutech21/fedimint-decoder/actions/workflows/deploy.yml/badge.svg)](https://github.com/ngutech21/fedimint-decoder/actions/workflows/deploy.yml)

Client-side web app for decoding Fedimint invite codes and OOB ecash notes.
Decoding runs locally in the browser and does not require a backend.

https://ngutech21.github.io/fedimint-decoder/


## Development

Enter the Nix development shell:

```sh
nix develop
```

Run the app locally:

```sh
trunk serve
```

Build the static app:

```sh
nix build .#
```

The build output is available through the `result` symlink after a successful
Nix build.
