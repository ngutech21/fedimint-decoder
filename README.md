# Fedimint Decoder [![Build](https://github.com/ngutech21/fedimint-decoder/actions/workflows/deploy.yml/badge.svg)](https://github.com/ngutech21/fedimint-decoder/actions/workflows/deploy.yml)

Client-side web app for decoding Fedimint invite codes and OOB ecash notes.
Decoding runs locally in the browser and does not require a backend.

## Goals

- Decode Fedimint invite codes with Fedimint's canonical Rust types.
- Decode OOB ecash notes and show offline-safe structure and amounts.
- Compile the app to WebAssembly.
- Keep normal operation serverless and local-only.

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
