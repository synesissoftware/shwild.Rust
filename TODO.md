# shwild.Rust - TODO <!-- omit in toc -->


## Table of Contents <!-- omit in toc -->

- [0.1.4 boilerplate checklist](#014-boilerplate-checklist)
- [Source layout (deferred)](#source-layout-deferred)
- [Functional improvements](#functional-improvements)
- [Performance improvements](#performance-improvements)


## 0.1.4 boilerplate checklist

### Cargo packaging

* [x] `default-features = false` on optional and dev dependencies;
* [x] `rust-version` (MSRV 1.79);
* [x] `version = "0.1.4"`;
* [x] upgraded **criterion** from 0.5 => 0.8;
* [x] ~~narrow `exclude` (drop `.github`)~~ — **won't fix** (keep `exclude = ["target", ".github"]` since 0.1.3);

### CI

* [x] `.github/workflows/ci.yml` (test, clippy, examples, doc, fmt, checkers);
* [x] Clippy, DOC_76, and test-naming fixes;
* [x] MSRV job on 1.79 (`cargo check --lib --locked`);
* [x] `check_test_names.py` allows `__CONSTRUCT__` padding around construct names;

### Documentation

* [x] back-filled **CHANGES.md** (0.1.0–0.1.3) and **0.1.4** entry;
* [x] crate-level `//!` documentation;
* [x] **EXAMPLES.md** and per-example `.md` files;
* [x] **NEWS.md** release table (through 0.1.4);
* [x] **README.md** badges, dependency links, related projects, `null-feature`;
* [x] shortened `description` in **Cargo.toml**;

### Formatting and quality tooling

* [x] `.cargo/config.toml`;
* [x] `.cursor/rules/rust-standards.mdc`;
* [x] `.gitattributes`;
* [x] `.gitignore` (`/scripts/__pycache__/`);
* [x] `scripts/check_derives.py`, `check_doc_76.py`, `check_test_names.py`, `fmt`;
* [x] renamed `.rustfmt.toml` => **rustfmt.toml**;

### Layout and structure

* [x] ~~flatten examples to `examples/*.rs`~~ — **won't fix** (keep `examples/*/main.rs`);
* [x] ~~move **character-play** from `[[example]]` to `[[bin]]`~~ — **won't fix** (since 0.1.3);
* [x] ~~normalise bench filenames to snake_case~~ — **won't fix** (keep current names);

### Release

* [ ] merge `boilerplate` branch and tag **0.1.4** on GitHub;
* [ ] publish **0.1.4** to crates.io;


## Source layout (deferred)

Split `src/lib.rs` (~3,450 lines) into multiple source files. **Defer past 0.1.4**
— behaviour-neutral refactor; ship as a dedicated follow-up change.

**API unit tests remain in `src/lib.rs`** (`#[cfg(test)] mod tests { ... }` and
`regex_comparision_tests`).

Proposed module layout:

```
src/
  lib.rs              # crate docs, re-exports, matches(), shwild_matches!,
                      # assert_shwild_matches!/assert_shwild_not_matches!;
                      # #[cfg(test)] API unit tests (stay here)
  constants.rs        # IGNORE_CASE
  error.rs            # Error + Display/Error trait impls
  traits.rs           # Match trait
  types.rs            # CharacterRangeType
  match_structures.rs # MatchLiteral, MatchRange, MatchWildN, ... + Match impls
  utils.rs            # MatcherSequence, parsing helpers, prepare_range_string
  compiled_matcher.rs # CompiledMatcher, ParseState, parse logic
```

Suggested order of work:

1. Extract internal modules (`match_structures`, `utils`, `compiled_matcher`, …)
   — largest line-count reduction with lowest risk;
2. Extract `error`, `constants`, `traits`, `types` — thin `lib.rs` façade
   (Diagnosticism-style);
3. Leave API tests in `lib.rs` (no `tests/mod.rs`).


## Functional improvements

* [ ] **no-std**;
* [ ] unit-test macros;


## Performance improvements

* [ ] special cases (for compiled only) such as `"*brown*"` could just be `strstr()`;
* [ ] thorough optimisation review (including optional "unsafe");


<!-- ########################### end of file ########################### -->
