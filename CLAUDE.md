# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A **Leptos 0.8 CSR** web app (Rust → WASM, single client-side bundle, no server
render, no hydration) that draws Candyland cards. Click the card to draw the next
one from a shuffled deck; "New Game" reshuffles. The app is two source files:
`src/main.rs` (Leptos components + signals) and `src/cards.rs` (deck model and
logic). Built by **Trunk**; styled with Tailwind v4.

Migrated from Dioxus — see `docs/superpowers/specs/` if a design doc was recorded.

## Commands

Requires the Trunk CLI (`trunk`) and the wasm target
(`rustup target add wasm32-unknown-unknown`). Formatting uses **leptosfmt**
(formats the `view!` macros), not plain `rustfmt`.

```bash
trunk serve                 # dev server + live reload at http://127.0.0.1:8080
trunk build --release       # optimized static bundle → dist/ (wasm + hashed CSS + assets)
cargo test                  # deck-logic unit tests (native, fast)
cargo test get_card         # run a single test by name
cargo clippy --release
leptosfmt src               # format; `leptosfmt --check src` to verify
```

Trunk auto-downloads its own `wasm-bindgen` and the **Tailwind v4 standalone CLI**
(pinned via `[tools] tailwindcss` in `Trunk.toml`) — no npm/node needed.
`rust-analyzer.toml` points editor format-on-save at `leptosfmt --stdin --rustfmt`.

## Architecture notes

- **Card model** (`cards.rs`, framework-agnostic): a `Card` is either a color card
  (`color: Some`, `count` 1 or 2, `symbol: None`) or a symbol card (`symbol: Some`,
  `color: None`). `Card::empty()` is the pre-first-draw placeholder. `init_cards()`
  builds 6 colors × (4 singles + 3 doubles) + 5 symbol cards = 47, shuffled;
  `get_card()` pops the last card and auto-reshuffles a fresh deck when empty.
- **UI** (`main.rs`): `App` holds four `RwSignal`s — `deck` (remaining cards),
  `card` (currently shown), `recent` (the history trail, `Vec<(u32, Card)>` newest
  first, capped at `TRAIL_LEN`), and `seq` (monotonic draw counter). `card_face` /
  `mini_face` return `AnyView` (arms produce different node types) and run inside
  reactive closures so the view updates on draw. Layout is a flex column: title,
  centered card, and a bottom **history trail** (`.trail`).
- **Draw-feedback animations** (CSS keyframes in `styles/tailwind.css`, driven from
  `main.rs`): every tap plays a 500ms **card flip** plus a 500ms trail update (the
  new mini slides in from behind the opaque "RECENT" label while fading up; existing
  minis slide over). CSS animations don't replay on a persistent node, so
  `anim_class(seq)` alternates an `a`/`b` class each draw — the two keyframe sets are
  identical, but changing the `animation-name` forces the browser to replay even on a
  repeat card. The trail uses a **keyed `<For>`** (key = `seq` id) so only the newly
  created mini runs its fade-in. `Element.animate` (WAAPI) was avoided because it
  needs `web_sys_unstable_apis`.
- **Reads across writes**: the draw handler uses `cards.get_untracked()` (clones the
  Vec) rather than holding a `.read()` guard across the subsequent `cards.set()` —
  holding a signal borrow across a write panics.
- **Card images**: `card_image()` maps a symbol string to a static URL like
  `/assets/cards/cone.jpg`. Trunk ships `assets/cards/` into `dist/` via the
  `copy-dir` link in `index.html`. Adding a symbol means updating three places: the
  `symbols` list in `init_cards()`, the match arms in `card_image()`, and the image
  file under `assets/cards/`.
- **Shell & styling**: `index.html` is the Trunk entry point — it holds the
  favicon, viewport meta, the `data-trunk rel="tailwind-css"` link, the `copy-dir`
  for card images, and `data-trunk rel="rust"` (which compiles the bin to wasm).
  The `<body>` is empty; `main()` calls `mount_to_body(App)`. Tailwind input is
  `styles/tailwind.css`: `@import "tailwindcss"`, `@source "../src"` (so the scanner
  sees classes in the `.rs` `view!` macros), plus custom `@layer components` color
  classes (`.red`/`.blue`/…) and the `body` background. Bare color words like
  `red`/`blue` are these custom classes, not Tailwind utilities.
- **getrandom on WASM**: `.cargo/config.toml` sets `getrandom_backend="wasm_js"` and
  `getrandom` uses the `wasm_js` feature — both required for `rand` in the browser.

## Build & deploy

- `Dockerfile` (used by CI): `rust:1-slim` builder with cargo-chef dep caching +
  pinned Trunk runs `trunk build --release`; runtime is
  **static-web-server on `scratch`** (`ghcr.io/static-web-server/static-web-server:2`,
  ~13MB — mostly the SWS binary) serving `dist/` from `/public` on port 80. This is
  a static SPA — there is no app server binary of our own. On-demand compression is
  on but does not apply to the (small) `.wasm`.
- CI (`.github/workflows/build.yml`): on push to any branch / tags / PRs, builds
  and pushes the image (`<registry>/candyland-roller`) to a private registry, then
  pings Watchtower to redeploy.
