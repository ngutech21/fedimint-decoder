# Fedimint Decoder

## Goal

Build a serverless, client-side web app that decodes Fedimint invite codes and
OOB ecash notes, similar in spirit to lightningdecoder.com for BOLT11 invoices.

The app must not require a backend. Decoding should happen locally in the
browser.

## Architecture

- Use Rust as the source of truth for decoding.
- Compile the browser app to WebAssembly.
- Prefer Leptos CSR for the UI.
- Do not use SSR, server functions, Axum, or any backend API for normal app
  operation.
- Use Fedimint crates for parsing and decoding instead of reimplementing the
  wire formats.
- Keep UI code separate from decoding/adaptation code, at least as a Rust module
  such as `decode.rs`. A separate crate is optional, not required.

## Fedimint Types

Invite codes should be decoded with Fedimint's `InviteCode` type from
`fedimint_core::invite_code`.

OOB ecash notes should be decoded with `OOBNotes` from `fedimint_mint_client`.
The app should treat Fedimint as the canonical parser and then map its output
into UI-friendly structures.

## OOB Notes Display

For OOB notes, useful offline/client-side fields include:

- total amount in msat, sat, and BTC
- federation ID prefix
- full federation ID when an embedded invite is present
- embedded invite code when present
- guardian/API URL and peer information when an embedded invite is present
- whether an API secret is present
- note count
- denomination breakdown, for example amount/count pairs
- raw decoded representation for advanced users
- parse/format errors

Sensitive note internals, including spend keys and raw spendable note data,
must not be displayed by default. If exposed, they should be behind an explicit
advanced/raw view with clear labeling.

## Offline Limits

Because the app is client-only and should not contact a federation by default,
it must not claim that a note is spendable or unspent.

Offline decoding can show structure and amounts. It cannot reliably determine:

- whether the note has already been spent
- whether the federation currently accepts it
- whether signatures validate against the federation's current config, unless
  the required config/public keys are available locally

Use wording like `format valid`, `decoded successfully`, and `spent status
unknown`. Avoid wording like `valid payment` or `redeemable` unless online
verification is intentionally added later.

## Technology Choices

Preferred initial stack:

- Rust
- Leptos CSR
- WASM target: `wasm32-unknown-unknown`
- Trunk or the current Leptos-recommended CSR tooling
- static hosting

Alternatives if Leptos causes friction:

- Yew for a React-like Rust/WASM component model
- Dioxus Web if future desktop/mobile reuse becomes important
- Vanilla `wasm-bindgen` plus minimal DOM code for a very small UI

## Development Priorities

1. Prove that the required Fedimint crates compile for the browser WASM target.
2. Implement invite code decoding first.
3. Implement OOB notes parsing and amount/denomination display.
4. Add structured error handling for invalid input and unsupported formats.
5. Add tests around known sample invite codes and OOB note strings.
6. Build the UI only after the decode surface is stable.

## UX Requirements

- The first screen should be the actual decoder, not a landing page.
- Input should accept pasted invite codes or OOB notes.
- The app should auto-detect the decoded type when practical.
- Results should have a structured view and a raw JSON view.
- Privacy expectations should be clear: normal decoding is local-only.
- Do not send pasted tokens to any server.

## Non-Goals For The Initial Version

- Joining federations
- Redeeming ecash
- Checking whether notes are spent
- Managing wallet state
- Storing user secrets
- Running a backend service

