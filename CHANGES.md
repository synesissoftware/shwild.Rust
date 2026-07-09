# shwild.Rust - CHANGES <!-- omit in toc -->


## 0.2.0 - 8th July 2026

* added `assert_shwild_matches!()` and `assert_shwild_not_matches!()` test assertion macros, available with the `"assertions"` feature (enabled by default);
* added `"flexible-flags-type"` feature — optional [**base-traits**](https://github.com/synesissoftware/base-traits) dependency (`implement-AsI64-for-built_ins`) allowing macro `flags` parameters to be any type implementing `AsI64`; when disabled, `flags` must be `i64`;
* `"assertions"` no longer implies **base-traits**; use `"full"` to enable assertions, flexible flags, and lookup ranges together;
* extended `shwild_matches!()` 3-parameter form with the same flexible-`flags` behaviour;
* added **macros** benchmark (`benches/macros.rs`);
* crate-level and macro `///` documentation for the assertion macros;
* **README.md** macros and dependencies sections updated;


## 0.1.5 - 9th July 2026

* bunch of boilerplate improvements: .gitattributes; .vimrc; EXAMPLES.md; VS Code settings; ignores; license; rustfmt.toml;


## 0.1.4 - 8th July 2026

* added **CHANGES.md** (back-filled) and **NEWS.md**;
* added **EXAMPLES.md** and per-example documentation;
* **README.md** badges, dependency links, related projects, and `null-feature` documentation;
* added crate-level `//!` documentation;
* added CI (`.github/workflows/ci.yml`) and quality scripts (`scripts/fmt`, checkers);
* added `rust-version` (MSRV 1.79);
* renamed `.rustfmt.toml` => **rustfmt.toml**; updated formatting settings;
* added `.gitattributes`;
* shortened `description` in **Cargo.toml**;
* Clippy and test-naming fixes for CI; `check_test_names.py` allows `__CONSTRUCT__` padding;
* upgraded **criterion** from 0.5 => 0.8;


## 0.1.3 - 28th March 2025

* crates.io packaging metadata — `categories`, `keywords`, `documentation`, and expanded `description`;
* added `exclude` for `target` and `.github` in **Cargo.toml**;
* added **TODO.md**;
* **character-play** changed from `[[bin]]` to `[[example]]`;


## 0.1.2 - 3rd November 2024

* added `"test-regex"` feature — optional **regex** dependency for benchmarks and scratch programs;
* enabled `"lookup-ranges"` in default features;
* added **cw-regex** benchmark;
* added **character-play** scratch program;
* added `regex_comparision_tests` unit tests (gated on `"test-regex"`);


## 0.1.1 - 3rd November 2024

* added `"lookup-ranges"` feature — optional **collect-rs** dependency for `UnicodePointMap`-based range matching;
* added **Cargo.lock**;
* README: added Features section; clarified Wild-1 escape behaviour in pattern elements;


## 0.1.0 - 3rd November 2024

* first public release;
* added `matches()` and `shwild_matches!()`;
* added `CompiledMatcher`;
* added `Error`, `Result`, and `IGNORE_CASE`;
* added example programs **list-matching-files** and **list-matching-files-compiled**;
* added benchmarks **range_string-creation_functions**, **shwild-compiled_matcher**, and **shwild-matches**;
* added **README.md**;


All history before this day is moot!


<!-- ########################### end of file ########################### -->
