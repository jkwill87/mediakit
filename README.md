# mediakit

A Rust library for parsing media filenames and file metadata into structured tags. It extracts
titles, season and episode numbers, codecs, resolutions, release information, and bounded container
metadata from media paths.

[Try the interactive browser demo](https://jkwill87.github.io/mediakit/).

## Quick start

```rust
use mediakit::inspect::{FilenameInspector, Inspector};

let inspector = FilenameInspector::new(
    "The.Bear.S01E01.System.1080p.WEB.H264-FLAME.mkv",
)
.analyze();

for tag in inspector.tags() {
    println!("{}: {}", tag.key(), tag.value());
}
```

`FilenameInspector` parses filename metadata. `FileInspector` adds size, format, MIME, and bounded
header probing for Matroska/WebM, ISO-BMFF/QuickTime, AVI, MPEG-TS/M2TS, and ASF/WMV. The public
`mediakit::probe` API exposes ordered audio and video streams with typed errors.

## Feature flags

| Flag            | Default | Description                     |
| --------------- | ------- | ------------------------------- |
| `with_serde`    | Yes     | Enables serde serialization     |
| `with_whatlang` | No      | Enables text-language detection |

## Development

Install [mise](https://mise.jdx.dev/) and use the pinned Rust, Bun, and wasm-pack toolchain:

```bash
mise install
mise test
mise lint
mise run demo:build
cd demo && mise x -- bun run check && mise x -- bun run build
```

The browser demo lives under `demo/`. Its `demo/wasm` package is intentionally independent from
the root Cargo package and depends on `mediakit` through `../..`.

Pushes to `main` rebuild the WASM package, type-check and bundle the demo, and publish `demo/dist`
to [GitHub Pages](https://jkwill87.github.io/mediakit/) through `.github/workflows/push.yml`.

## License

MIT
