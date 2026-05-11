# Product

## Register

product

## Users

Fedimint Decoder is for technically interested Fedimint and Bitcoin users, especially developers, operators, implementers, and power users who write code, inspect protocol data, or work directly with Fedimint integrations. They arrive with a pasted invite code or OOB ecash token and want to understand its structure without joining a federation, redeeming funds, or sending sensitive material to a server.

## Product Purpose

Fedimint Decoder exists to make Fedimint invite codes and OOB ecash notes inspectable in a local-only browser tool. Success means a user can paste a token, see the decoded structure, amounts, federation information, and parser errors, then leave with a clearer technical understanding of the artifact. The product must be explicit about offline limits: it can show format and structure, but it must not imply spendability, acceptance by a federation, or unspent status.

## Brand Personality

Calm, precise, and privacy-first. The interface should feel like a trusted protocol workbench: quiet enough for careful inspection, direct enough for developers, and restrained enough to avoid implying more certainty than offline decoding can provide.

## Anti-references

Do not make the product feel like a crypto trading dashboard, a wallet redemption flow, a promotional site, or a hype-driven Web3 surface. Avoid decorative blockchain tropes, neon-on-black theatrics, speculative language, and any copy that suggests a note is valid payment, redeemable, or unspent without online verification.

## Design Principles

1. Make the decoder the first screen. Users should arrive directly in the task, with the input and decoded output as the primary interface.
2. Preserve local trust. The interface should repeatedly honor the local-only privacy model through behavior and wording, not through loud reassurance.
3. Separate structure from certainty. Decoded fields, amounts, and parser success are useful; online status and spendability remain unknown unless intentionally verified later.
4. Serve technical scanning. Dense structured fields, monospace values, raw representations, and clear errors are part of the core workflow.
5. Hide sensitive internals by default. Advanced/raw views may exist, but spend keys and spendable note internals must never be casually exposed.

## Accessibility & Inclusion

Target WCAG AA contrast and keyboard-accessible controls. The tool should work well for users pasting long, unbroken protocol strings, using reduced motion settings, zooming text, or scanning results on narrow screens. Color must not be the only channel for status or meaning.
