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
`mediakit::probe` API exposes ordered video, audio, and embedded subtitle streams with typed errors:

```rust
use mediakit::probe::FileProber;

let media = FileProber::new("movie.mkv").unwrap().probe().unwrap();
println!("container: {}", media.container);
```

Probe results report the exact content-derived `MediaFormat` in `MediaInfo::container` and group
enabled/default/language data for every stream under its shared `StreamInfo`. Preferred audio,
video, and subtitle streams are available through the corresponding
`MediaInfo::primary_*_stream` methods.

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

Successful `main` pushes also publish the next patch as a development prerelease on
[crates.io](https://crates.io/crates/mediakit), such as `0.1.1-dev42`. Reruns reuse the same version.
Pushing a stable `vMAJOR.MINOR.PATCH` tag publishes that exact release version. Publishing runs only
after lint and tests pass; the registry credential is stored in the `CARGO_REGISTRY_TOKEN` GitHub
Actions secret.

## License

MIT
