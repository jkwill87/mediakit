# How guessit parses scene release filenames

[GuessIt](https://github.com/guessit-io/guessit) (v3.8.0, latest stable) is a heuristic, pattern-matching filename parser built on the **Rebulk** library. It does not enforce a formal grammar; instead, it registers hundreds of regex/string/functional patterns across ~25 property modules, runs them all against the input, resolves conflicts by priority and length, then applies post-processing rules to clean up results. **The title is not matched by a pattern at all — it is whatever unmatched "hole" remains** after every other property has been claimed. This architecture makes guessit flexible across messy real-world filenames but creates well-documented blind spots on strictly formatted scene releases.

---

## Episode identification relies on layered chain patterns

Episode parsing lives in `guessit/rules/properties/episodes.py`, the largest file in the codebase. It uses Rebulk's **chain pattern** system to match sequences of sub-patterns with optional separators. Configuration values (`season_max: 100`, `episode_max: 1000`, season/episode words, markers) are loaded from `config/options.json` and passed into the builder function.

**SxxExx — the strongest pattern.** The core chain matches `S{season}E{episode}` with repeating episode groups:

```python
rebulk.chain(
    formatter={'season': int, 'episode': int, 'version': int},
    tags=['SxxExx'], children=True
)
.regex(r'S(?P<season>\d{1,4})')  # S01
.regex(r'E(?P<episode>\d{1,4})')  # E01
.regex(r'(?:E(?P<episode>\d{1,4}))', repeat=(0, None))  # E02E03... (multi-ep)
.regex(r'v(?P<version>\d+)', repeat=(0, 1))  # optional v2 suffix
```

This single chain handles `S01E01`, `S01E01E02E03` (consecutive multi-episode), and `S01E01v2` (versioned episodes). Separators between `S` and `E` components are optional and can be dots, dashes, spaces, or underscores. Season supports **1–4 digits** (capped by `season_max`, default 100); episode supports 1–4 digits (capped by `episode_max`, default 1000).

**NxNN format** uses a separate chain: `(?P<season>\d{1,2})x(?P<episode>\d{1,2})`, matching patterns like `1x03` or `12x08`. A related `{season}e{episode}` chain handles `1e03` style. Both are lower confidence than SxxExx.

**Multi-episode ranges** (e.g., `S01E01-E03`) are expanded by the `EpisodeNumberSeparatorRange` post-processing rule. When two episode matches are connected by a `-` separator, the rule generates intermediate episode numbers — so `S01E01-E03` produces episodes `[1, 2, 3]`. The `S01E01-03` variant (no `E` prefix on the end number) is also handled. A parallel `SeasonSeparatorRange` rule expands season ranges like `S01-S03`.

**Episode word patterns** match localized keywords: `episode`, `ep`, `épisode`, `cap`, `capítulo`, `capitulo`, `folge` — each followed by 1–4 digits. Season words include `season`, `saison`, `temporada`, `serie`, `series`, `staffel`. Both use case-insensitive regex with flexible separators.

**Absolute episode numbers** are handled through a rename mechanism. When no season match exists and a bare episode number is found, the `RenameToAbsoluteEpisode` rule renames the `episode` property to `absolute_episode`. The `-E` / `--episode-prefer-number` flag activates a weak bare-number pattern (`\d{2,4}`) for anime-style filenames like `Naruto.213.720p`. Without this flag, **a combined-number splitter** can interpret `213` as season 2, episode 13 (divmod by 100), but only if the result passes sanity checks (season ≤ 50).

**Daily show dates** use `python-dateutil` for parsing. The regex captures `YYYY.MM.DD`, `DD.MM.YYYY`, and `MM.DD.YYYY` with flexible separators (`[.\-_ ]`). The `--date-year-first` and `--date-day-first` flags control ambiguous date interpretation. A `SeasonYear` rule disambiguates when a 4-digit number could be either a year or a season+episode combo.

**Episode parts** are handled in `part.py` using prefix words `Part` and `Pt` followed by a numeral (Arabic or Roman): `(?:Part|Pt)\W?(?P<part>\d{1,2})`. Alpha episode suffixes like `S02E01A` are captured by a trailing `(?P<episode_details>[a-z])` group appended to the SxxExx chain with `repeat=(0, 1)`.

---

## Source, resolution, codec, and group all follow a pattern-per-value model

Each technical property module creates a `Rebulk` instance with `regex_defaults(flags=re.IGNORECASE, abbreviations=[dash])`, where `dash = re.compile(r'[-\.]')` makes dots and hyphens interchangeable in all patterns. A `seps_surround` validator ensures every match is bounded by separator characters (` .-;:_+/`), preventing partial matches inside words.

**Source detection** (`source.py`, renamed from `format.py` in v3.0) maps multiple input patterns to canonical values:

| Canonical Value    | Patterns                                                              |
| ------------------ | --------------------------------------------------------------------- |
| `Blu-ray`          | `Blu-?ray(?:-?Rip)?`, `B[DR]`, `B[DR]-?Rip`, `BD[59]`, `BD25`, `BD50` |
| `Ultra HD Blu-ray` | `UHD-?Blu-?ray`                                                       |
| `Web`              | `WEB-?DL`, `WEB-?HD`, `WEB`, `DL-?WEB`, `DL(?=-?Mux)`                 |
| `WEBRip`           | `WEB-?Rip`, `WEB-?DL-?Rip`, `WEB-?Cap`                                |
| `HDTV`             | `HDTV`, `HD-?TV`                                                      |
| `DVD`              | `DVD-?Rip`, `DVD-?Mux`, `DVD-?R(?:$\|(?!ip))`, `DVD-?[59]`            |
| `UHDTV`            | `UHD-?TV`, `UHD-?Rip`                                                 |

A `ValidateSource` rule removes matches that lack proper separator boundaries. The `-?` in patterns makes hyphens optional, so both `BluRay` and `Blu-Ray` match identically.

**Resolution** (`screen_size.py`) matches standard progressive/interlaced formats: `360p`, `480p`, `576p`, `720p`, `900p`, `1080p`, `1080i`, `2160p`, `4320p`. The `4K` string maps to `2160p`. Framerate suffixes like `720p50` or `1080p24` are stripped to their base resolution. The `ValidateScreenSize` rule prevents conflicts with year or episode numbers. Notably, **540p is not in the default config** and will be misidentified.

**Video codec** (`video_codec.py`) normalizes common codec tokens:

| Canonical Value | Patterns            |
| --------------- | ------------------- |
| `H.264`         | `[hx]-?264`, `AVC`  |
| `H.265`         | `[hx]-?265`, `HEVC` |
| `Xvid`          | `Xvid`              |
| `DivX`          | `DivX`              |
| `MPEG-2`        | `MPEG-?2`           |
| `VP9`           | `VP9`               |
| `AV1`           | `AV1`               |

The `[hx]` character class means both `x264` and `h264` (and `H264`, `X264` due to `IGNORECASE`) map to the same `H.264` value.

**Release group** (`release_group.py`) uses a fundamentally different approach — **positional detection** rather than vocabulary matching. The `SceneReleaseGroup` rule looks for text after the **last hyphen** in the filename, provided that hyphen is preceded by recognized technical properties (source, codec, resolution, etc.). This mirrors the scene convention where `Title.S01E01.720p.HDTV.x264-GROUP` places the group after the final dash. An `AnimeReleaseGroup` rule handles `[GROUP]` bracket prefixes common in anime. Configuration specifies `forbidden_names` (`bonus`, `by`, `for`, `par`, `pour`, `rip`) that are rejected as group names, and `ignored_seps` (`[]{}()`) that are stripped from candidates.

---

## Tags like PROPER and REPACK normalize to shared values

The `other.py` module handles miscellaneous scene tags. Verified source code from the bazarr-bundled guessit shows the actual pattern registrations:

```python
rebulk.regex('Audio-?Fix', 'Audio-?Fixed', value='AudioFix')
rebulk.regex('Sync-?Fix', 'Sync-?Fixed', value='SyncFix')
rebulk.regex('Dual', 'Dual-?Audio', value='DualAudio')
rebulk.regex('ws', 'wide-?screen', value='WideScreen')
rebulk.regex('Re-?Enc(?:oded)?', value='ReEncoded')
rebulk.string('Real', 'Fix', 'Fixed', value='Proper', tags=['has-neighbor-before', 'has-neighbor-after'])
rebulk.string('Proper', 'Repack', 'Rerip', 'Dirfix', 'Nfofix', 'Prooffix', value='Proper', tags=['streaming_service.prefix', 'streaming_service.suffix'])
rebulk.regex('(?:Proof-?)?Sample-?Fix', value='Proper', tags=['streaming_service.prefix', 'streaming_service.suffix'])
```

A critical design choice: **PROPER, REPACK, RERIP, DIRFIX, NFOFIX, PROOFFIX, Real, Fix, and Fixed all normalize to the single value `'Proper'`**. This means guessit does not distinguish between a REPACK and a PROPER in its output — both become `other: 'Proper'`. The `has-neighbor-before`/`has-neighbor-after` tags on `Real`/`Fix`/`Fixed` require these common English words to appear adjacent to other recognized matches, preventing false positives on titles containing those words.

**EXTENDED, INTERNAL, and similar edition tags** were moved to a separate `edition.py` property in v3.0. Key values include `Extended` (patterns: `Extended`, `re:Extended-?Cut`), `Internal` (pattern: `INTERNAL`), `Limited` (`LIMITED`), `Unrated`, `Uncut`, `Remastered`, `Director's Cut` (`re:Director'?s?-?Cut`), `IMAX`, `Theatrical`, `Special` (`re:Special-?Edition`), `Collector`, `Criterion`, and others. In v3.4+, these are config-driven — patterns are loaded from `options.json` rather than hardcoded.

**HDR and Dolby Vision** detection is handled by a `video_hdr_format` property (introduced ~v3.4). Known values include `HDR10`, `HDR10+` (`HDR10Plus`), `Dolby Vision` (`DV`, `DOVI`, `Dolby-?Vision`), `HDR` (generic), `SDR`, `HLG`, and `PQ`. However, as documented in issue #648, **DV detection has been historically problematic** — the two-letter token `DV` conflicts with language codes and was misidentified as `alternative_title` or the Dutch language `Multiple languages` in versions before explicit support was added.

**READNFO, SUBBED, DUBBED, CONVERT** are handled in `other.py` with patterns like `rebulk.string('Read-?NFO', value='ReadNFO')`, `rebulk.string('Subbed', value='Subbed')`, and similar. The `Remux` tag is also in `other.py`: `rebulk.string('Remux', value='Remux')`.

---

## Title extraction works by finding "holes" between matched properties

The title detection algorithm in `title.py` is fundamentally different from all other property detections. Instead of matching known patterns, the `TitleFromPosition` rule identifies the title as the **first sufficiently long unmatched region** ("hole") in the filename after all other patterns have been applied.

The process works as follows. First, the filename is split into "fileparts" based on path separators (`/`, `\`), and brackets/parentheses create sub-groups. Second, all property modules run their patterns against the entire string, creating matches for season, episode, source, codec, resolution, language, and every other property. Third, `TitleFromPosition` scans for the **first hole before any recognized metadata** in the filename's main part. For `Show.Name.S01E03.720p.HDTV.x264-GROUP`, the engine matches `S01E03`, `720p`, `HDTV`, `x264`, and `GROUP` — leaving `Show.Name` as the first hole, which becomes the title.

For episode-type content, a **second hole** between the episode marker and the next technical property becomes the `episode_title`. In `Treme.1x03.Right.Place,.Wrong.Time.HDTV.XviD-NoTV.avi`, `Treme` is the title, and `Right Place, Wrong Time` (the hole between `1x03` and `HDTV`) is the episode_title.

The `cleanup` formatter converts separator-delimited words back to normal spacing (`Show.Name` → `Show Name`), and `reorder_title` handles inverted titles (`Movie, The` → `The Movie`). An `expected_title` option (`-T` flag) allows users to provide regex patterns for known titles, which get priority matching via `rebulk.functional()`.

This "holes" approach means **title boundary detection is entirely dependent on the accuracy of every other pattern**. If a metadata token goes unrecognized, it gets absorbed into the title. If a title word happens to match a known property (like "Seasons" matching the season keyword, as in issue #800), the title gets truncated.

---

## Known limitations expose fundamental architectural tradeoffs

GuessIt's heuristic architecture creates several well-documented failure modes for scene release parsing, with **77 open issues** as of March 2026.

**Resolution without 'p' suffix causes misidentification.** A filename like `Gladiator.EXTENDED.2000.720.BrRip` parses `720` as season 7, episode 20 rather than 720p resolution (issue #693). The screen_size patterns require the `p` or `i` suffix, matching the scene standard but failing on non-standard encodings.

**Release group name corruption** is a persistent category of bugs. Group names containing reserved words get partially stripped: `PARTiCLE` loses its `PART` prefix, `NovaRip` loses its `aRip` suffix (issue #297, partially fixed in v3.2). Groups containing digits like `NF69` or `GOLF68` trigger false `film` number matches (issue #294). The `expected_group` option (`-G 're:\bGroupName\b'`) provides a workaround but requires foreknowledge of group names.

**Movie vs. episode type ambiguity** is a fundamental challenge. Without the explicit `-t episode` flag, guessit must infer the content type. Bare numbers like `213` could be an episode number, a combined season+episode, or just part of a movie title. The library errs toward movie type by default, which can cause episode metadata to be missed entirely.

**Anime naming conventions** diverge significantly from scene standards. The `[Group] Title - Episode [Resolution] [CRC32].ext` format is partially supported via `AnimeReleaseGroup`, but dash-separated subtitle titles (e.g., `Nanatsu no Taizai - Kamigami no Gekirin`) create parsing ambiguity. The CRC32 hash inside brackets is detected but can conflict with other bracket-enclosed metadata.

**Title words matching property keywords** cause silent truncation. Issue #800 documents "The Four Seasons" being parsed as "the four" because "Seasons" triggers the season keyword pattern. Similarly, show names containing numbers, language codes, or codec-like strings (e.g., a show called "V8") can be partially consumed by property matchers.

**No positional enforcement.** Unlike strict scene grammars where position is semantic (title first, group last after dash), guessit's patterns match anywhere in the string. Post-processing rules attempt positional validation, but they are heuristic and can fail. The `seps_surround` validator prevents mid-word matches but doesn't enforce the standard `Title.Meta.Meta-Group` ordering.

**DV/Dolby Vision detection remains fragile.** As of issue #648, `DV` conflicts with language detection (`DL` = "Multiple languages" in some contexts, and `DV` can be parsed as an ISO language code). The fix required explicit video_hdr_format patterns, but edge cases persist when `DV` appears adjacent to language-like tokens like `DL.DV` or `SL.DV`.

---

## Conclusion

GuessIt's architecture — hundreds of independent Rebulk patterns composed into a single matcher with conflict resolution and post-processing rules — makes it remarkably effective across the chaotic diversity of real-world filenames. The SxxExx chain pattern with its repeating episode groups elegantly handles multi-episode ranges, and the config-driven approach in v3.4+ allows customization without code changes. However, the "holes" title detection algorithm, the normalization of PROPER/REPACK/RERIP into a single value, and the lack of strict positional grammar enforcement mean that guessit is best understood as an **approximate parser optimized for recall over precision**. Applications requiring strict scene naming compliance — particularly for release group preservation, PROPER vs. REPACK distinction, or Dolby Vision detection — should layer additional validation on top of guessit's output or provide `expected_title` and `expected_group` hints to constrain the matching space.
