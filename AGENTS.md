# AGENTS.md

## Toolchain

Run commands through mise so the pinned toolchain is used:

- `mise x -- <cmd>` for direct commands
- `mise run <task>` for tasks defined in `mise.toml`

Pinned tools are Rust 1.97.0, Bun 1.3.14, and `cargo:wasm-pack` 0.15.0.

## Commands

| Goal                  | Command                                                              |
| --------------------- | -------------------------------------------------------------------- |
| Build the library     | `mise x -- cargo build`                                               |
| Check all targets     | `mise run rust:check`                                                 |
| Run Rust tests        | `mise test`                                                           |
| Test all features     | `mise test --all-features`                                            |
| Lint Rust             | `mise lint`                                                           |
| Format Rust           | `mise x -- cargo fmt`                                                 |
| Run benchmarks        | `mise run rust:bench`                                                 |
| Build demo WASM       | `mise run demo:build`                                                 |
| Run demo dev server   | `mise run demo:dev`                                                   |
| Type-check demo       | `cd demo && mise x -- bun run check`                                  |
| Build demo bundle     | `cd demo && mise x -- bun run build`                                  |
| Run inspect example   | `mise x -- cargo run --example inspect-path -- path/to/file.mkv`      |

After changing `src/` or `demo/wasm`, rebuild the WASM package with `mise run demo:build` before
relying on the browser demo.

## Repository layout

| Path        | Purpose                                                        |
| ----------- | -------------------------------------------------------------- |
| `src/`      | Core parser library exposed through `inspect`, `meta`, `probe` |
| `tests/`    | Public API integration tests                                   |
| `benches/`  | Criterion benchmarks                                           |
| `examples/` | Command-line examples                                          |
| `demo/wasm` | Independent wasm-bindgen package with a root path dependency   |
| `demo/`     | React 19 and TypeScript browser demo                           |

## Architecture and conventions

- `inspect::FilenameInspector` tokenizes filenames and runs the inspectors under
  `src/inspect/inspectors/filename/`.
- `probe` owns bounded, dependency-free container probing; `inspect::FileInspector` converts
  primary streams into tags.
- Parsed metadata uses `meta::Tag` and typed fields under `src/meta/fields/`.
- Regex matching uses the `regex-automata` Pike VM directly to keep WASM builds small.
- The crate denies missing public docs, uses `thiserror` for typed errors, and forbids unsafe code.
- Unit tests are colocated as `*.test.rs` files and included with `#[path = "..."]` modules.
- Keep generated `demo/wasm/pkg` output in sync locally but do not commit it.
- `demo/wasm` has its own lockfile because this repository is not a Cargo workspace.

## Repository notes

- GitHub Actions runs the mise-backed Rust and demo checks before publishing packages.
- Successful `main` pushes publish a next-patch `-dev<RUN_NUMBER>` prerelease to crates.io; reruns
  reuse that version, and `vMAJOR.MINOR.PATCH` tags publish the corresponding stable release.
- The crates.io credential belongs only in the `CARGO_REGISTRY_TOKEN` GitHub Actions secret or a
  gitignored local mise override.
- Personal overrides belong in gitignored `mise.local.toml`; never commit credentials.
- `target/`, `demo/wasm/target/`, generated WASM, demo dependencies, and demo output are ignored.
