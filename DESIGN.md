---
name: Fedimint Decoder
description: A local-only technical workbench for inspecting Fedimint invite codes and OOB ecash notes.
colors:
  page-mint-wash: "oklch(96% 0.011 172)"
  panel-mint-paper: "oklch(99% 0.006 172)"
  surface-mint-paper: "oklch(98% 0.006 172)"
  surface-mint-strong: "oklch(94% 0.013 172)"
  field-mint-paper: "oklch(98% 0.004 170)"
  text-deep-green-ink: "oklch(21% 0.018 175)"
  text-muted-green-slate: "oklch(47% 0.025 175)"
  line-soft-green: "oklch(84% 0.019 175)"
  line-strong-green: "oklch(74% 0.035 175)"
  accent-protocol-mint: "oklch(55% 0.14 165)"
  accent-protocol-mint-strong: "oklch(46% 0.13 165)"
  accent-on-mint: "oklch(98% 0.008 165)"
  error-red: "oklch(45% 0.16 28)"
typography:
  display:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "clamp(2.2rem, 6vw, 4.4rem)"
    fontWeight: 700
    lineHeight: 0.95
    letterSpacing: "0"
  title:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "1rem"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "0"
  body:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "1rem"
    fontWeight: 400
    lineHeight: 1.6
    letterSpacing: "0"
  label:
    fontFamily: "Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif"
    fontSize: "0.78rem"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "0"
  mono:
    fontFamily: "SFMono-Regular, Consolas, Liberation Mono, ui-monospace, monospace"
rounded:
  md: "8px"
spacing:
  xs: "5px"
  sm: "10px"
  md: "12px"
  lg: "16px"
  xl: "18px"
  shell-mobile: "20px"
  panel: "clamp(24px, 5vw, 44px)"
components:
  button-primary:
    backgroundColor: "{colors.accent-protocol-mint}"
    textColor: "{colors.accent-on-mint}"
    rounded: "{rounded.md}"
    height: "44px"
    padding: "0 20px"
  button-primary-hover:
    backgroundColor: "{colors.accent-protocol-mint-strong}"
    textColor: "{colors.accent-on-mint}"
    rounded: "{rounded.md}"
  field:
    backgroundColor: "{colors.field-mint-paper}"
    textColor: "{colors.text-deep-green-ink}"
    rounded: "{rounded.md}"
    padding: "16px"
  summary-item:
    backgroundColor: "{colors.field-mint-paper}"
    textColor: "{colors.text-deep-green-ink}"
    rounded: "{rounded.md}"
    padding: "12px"
  status-item:
    backgroundColor: "{colors.surface-mint-strong}"
    textColor: "{colors.text-deep-green-ink}"
    rounded: "{rounded.md}"
    padding: "12px"
---

# Design System: Fedimint Decoder

## 1. Overview

**Creative North Star: "The Protocol Workbench"**

Fedimint Decoder is a quiet technical surface for inspecting sensitive protocol strings. It should feel local, exact, and trustworthy, with the decoder itself as the primary screen. The user is already motivated; the design should help them paste, decode, scan, and copy what they need.

The current system uses a restrained light theme with mint-tinted neutrals, one purposeful protocol-green accent, and compact structured result rows. It rejects crypto spectacle, promotional composition, wallet-like redemption framing, nested card stacks, and any language that overstates offline certainty.

**Key Characteristics:**
- Task-first layout with the input and decoded output in one continuous, unframed workspace.
- Tinted neutral surfaces that imply calm inspection rather than financial action.
- Dense but readable result groups for long identifiers, URLs, and denominations.
- Monospace treatment for protocol values and raw strings.
- Status wording that distinguishes parser success from online validity.

## 2. Colors

The palette is a restrained mint-neutral system: mostly pale green-tinted surfaces, deep green-black text, and a single protocol mint accent reserved for action and focus.

### Primary

- **Protocol Mint** (`oklch(55% 0.14 165)`): Primary action color for the Decode button and focus border. Use sparingly; it signals the one command that changes the decoded state.
- **Protocol Mint Strong** (`oklch(46% 0.13 165)`): Hover and emphasized accent state. It should not become a general decoration color.

### Neutral

- **Mint Wash** (`oklch(96% 0.011 172)`): Page background. It keeps the tool bright without falling into plain white.
- **Mint Paper** (`oklch(99% 0.006 172)`): Input background and row background for decoded data groups.
- **Strong Mint Surface** (`oklch(94% 0.013 172)`): Offline status cells and other neutral system states.
- **Field Paper** (`oklch(98% 0.004 170)`): Textareas, summary tiles, rows, and raw code blocks.
- **Deep Green Ink** (`oklch(21% 0.018 175)`): Primary text. It replaces black with a softer technical ink tone.
- **Muted Green Slate** (`oklch(47% 0.025 175)`): Secondary copy, labels, row descriptors, and explanatory text.
- **Soft Green Line** (`oklch(86% 0.018 175)`): Borders and dividers.

### Tertiary

- **Error Red** (`oklch(45% 0.16 28)`): Error text only. Pair with explicit wording, never rely on the red alone.

### Named Rules

**The Offline Certainty Rule.** Green means a command or parser focus state; it must not imply that an ecash note is redeemable, unspent, or accepted by a federation.

**The One Accent Rule.** The accent belongs to primary action, focus, and product identity. Do not use extra crypto blues, oranges, purples, gradients, or neon accents.

## 3. Typography

**Display Font:** Inter with system UI fallbacks  
**Body Font:** Inter with system UI fallbacks  
**Label/Mono Font:** SFMono-Regular, Consolas, Liberation Mono, ui-monospace, monospace

**Character:** The typography is direct and tool-native. Inter carries the interface with familiar product ergonomics; monospace appears only when the user needs to inspect exact protocol data.

### Hierarchy

- **Display** (700, `clamp(2.2rem, 6vw, 4.4rem)`, 0.95): Used only for the main page title. Keep it short and avoid adding oversized type inside result panels.
- **Title** (700, `1rem`, 1.2): Section headings, detail block headings, input labels, and compact UI titles.
- **Body** (400, `1rem`, 1.6): Explanatory text and standard prose. Keep prose lines below roughly 65ch where possible.
- **Label** (700-800, `0.78rem`, 0): Eyebrows, field labels, summary labels, and table headers. Uppercase is acceptable for the product eyebrow and denomination header only.
- **Mono** (normal, inherited size): Federation IDs, URLs, raw codes, tokens, amounts with units, and other values where exact characters matter.

### Named Rules

**The Data Is Text Rule.** Long protocol values must wrap with `overflow-wrap: anywhere`; never truncate identifiers or token strings by default.

## 4. Elevation

The system is mostly flat. Depth comes from page tone, section dividers, row grouping, and focus rings rather than stacked cards. The decoder workspace itself is not framed as a card.

### Shadow Vocabulary

- **Focus Ring** (`box-shadow: 0 0 0 4px oklch(55% 0.14 165 / 16%)`): Use for focused inputs and controls. Pair with a border-color change.

### Named Rules

**The Flat Results Rule.** Decoded data blocks are bordered and tonal, not shadowed. Do not put result cards inside a larger card.

## 5. Components

### Buttons

- **Shape:** Rectangular product control with medium corners (`8px`).
- **Primary:** Protocol Mint background, Accent-on-Mint text, `44px` minimum height, `0 20px` padding, heavy label weight.
- **Hover / Focus:** Hover shifts to Protocol Mint Strong. Focus should use the same ring vocabulary as inputs.
- **Mobile:** Primary action stretches full width below `640px`.

### Containers

- **Corner Style:** Consistent `8px` radius.
- **Background:** The page is the workspace. Inputs and data rows use Mint Paper, status rows use Strong Mint Surface.
- **Shadow Strategy:** No resting shadows. Use tonal contrast and borders.
- **Border:** One-pixel Soft Green Line on inputs and row groups. Use `border-block` for detailed lists to avoid stacked boxes.
- **Internal Padding:** Data rows use `12px 14px`; textarea uses `16px`.

### Inputs / Fields

- **Style:** Full-width textarea, Field Paper background, Soft Green Line border, `8px` radius, `16px` padding, inherited product font.
- **Focus:** Protocol Mint border and a soft 4px mint focus ring.
- **Behavior:** Long pasted tokens should remain visible and editable. Vertical resize is allowed.
- **Error / Disabled:** Errors use Error Red text with clear parser wording. Disabled states are not currently defined and should be added before introducing async or unavailable actions.

### Navigation

No persistent navigation exists in the current app. Future navigation should remain secondary to the decoder. Tabs are appropriate for switching structured results and raw JSON; promotional navigation is not.

### Results

- **Summary Grid:** Two-column row group on desktop, one column on mobile. Items are label/value rows separated by shared dividers, not individual cards. Avoid rounded outer boxes for nested result groups.
- **Offline Status:** OOB results begin with a three-column neutral status row for format, spent status, and federation acceptance. These states must stay neutral; they do not certify spendability.
- **Peer and Denomination Lists:** Use `border-block` and divided rows, not list cards. Desktop rows use explicit grid columns; mobile collapses each row to a single column.
- **Raw Disclosure:** Raw invite codes and normalized tokens live inside explicit disclosure controls. OOB raw token copy warns that the value may contain spendable ecash data.
- **Raw Code Blocks:** Use block dividers, monospace text, and aggressive wrapping. Use inside disclosure controls by default.
- **Error Blocks:** Parser errors are split by parser target with shared row-group styling, block dividers, and a soft error surface.

## 6. Do's and Don'ts

Do make the decoder the first visible experience.  
Do keep all normal decoding language local-only and offline-safe.  
Do use monospace for exact strings, URLs, federation IDs, and raw tokens.  
Do preserve wrapping behavior for long unbroken values.  
Do use the mint accent for primary action, focus, and product identity only.
Do keep raw token material behind explicit user disclosure.  
Do show offline status directly in decoded OOB results.

Don't imply a note is spendable, unspent, redeemable, or accepted by a federation without online verification.  
Don't introduce wallet-style calls to action such as redeem, claim, pay, or withdraw.  
Don't add crypto-themed decoration, neon-on-black palettes, glowing gradients, or market-dashboard cues.  
Don't create repeated decorative card grids or nested cards around result data. Prefer grouped rows and section dividers.  
Don't hide parse errors behind vague messages; technical users need precise failure context.
