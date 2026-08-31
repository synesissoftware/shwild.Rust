# shwild.Rust <!-- omit in toc -->

**SH**ell-compatible **WILD**cards, for **Rust** — part of the cross-language
**shwild** family.


![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Crates.io](https://img.shields.io/crates/v/shwild.svg)](https://crates.io/crates/shwild)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/shwild.Rust.svg)](https://github.com/synesissoftware/shwild.Rust/releases/latest)
![MSRV](https://img.shields.io/badge/MSRV-1.79-lightgrey)
[![CI](https://github.com/synesissoftware/shwild.Rust/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/shwild.Rust/actions/workflows/ci.yml)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/shwild.Rust)](https://github.com/synesissoftware/shwild.Rust/commits/master)
[![docs.rs](https://img.shields.io/docsrs/shwild/badge.svg)](https://docs.rs/shwild)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
	- [Pattern Elements](#pattern-elements)
- [Installation](#installation)
- [Components](#components)
	- [Constants](#constants)
	- [Enumerations](#enumerations)
	- [Features](#features)
	- [Functions](#functions)
	- [Macros](#macros)
	- [Structures](#structures)
	- [Traits](#traits)
- [Examples](#examples)
- [Project Information](#project-information)
	- [Where to get help](#where-to-get-help)
	- [Contribution guidelines](#contribution-guidelines)
	- [Dependencies](#dependencies)
		- [Dev Dependencies](#dev-dependencies)
	- [Related projects](#related-projects)
	- [License](#license)


## Introduction

**shwild** is a small, standalone C++ library with C and C++ APIs that
provides shell-compatible wildcard matching. **shwild.Rust** is a **Rust**
port with minimal API differences. The design emphasis is on
simplicity-of-use, modularity, and performance.

```rust
	let pattern = r"Where are the* [🐼🐻]s\?";

	assert_eq!(Ok(false), shwild_matches!(pattern, ""));
	assert_eq!(Ok(false), shwild_matches!(pattern, "Where are the bears?"));
	assert_eq!(Ok(true),  shwild_matches!(pattern, "Where are the 🐻s?"));
	assert_eq!(Ok(true),  shwild_matches!(pattern, "Where are the 🐼s?"));
	assert_eq!(Ok(true),  shwild_matches!(pattern, "Where are their 🐻s?"));
	assert_eq!(Ok(true),  shwild_matches!(pattern, "Where are the big brown 🐻s?"));
	assert_eq!(Ok(false), shwild_matches!(pattern, "Where are the teddy-🐻s?"));
```

(See [Examples](#examples) section for more examples.)

### Pattern Elements

The library (and other **shwild** variants) support the following pattern elements:

* **Literal** - a non-empty string fragment, as in `"Where are the"`, which matches the exact same string fragment in the input;
* **Wild-1** - represented by the single character `'?'` in the pattern, which represents a match of exactly any one character. In the above example `r"Where are the* [🐼🐻]s\?"` the `'?'` is _not_ interpreted as a wild-1 because it is escaped by the `'\'` character and instead part of the literal fragment `"s?"`;
* **Wild-N** - represented by the single character `'*'` in the pattern, which represents a match of any number of characters;
* **Range** - represented by a sequence of characters within `'['` and `']'`, as in the `"[🐼🐻]"` fragment in the above example, which will match to any one of range character in the input. As well as an unordered sequence of literal characters, ranges may also capture contiguous sequences, as in `"[zc-aja]"` (any of characters `'a'`, `'b'`, `'c'`, `'j'`, `'z'`) or in `"[abm-PrZ]"` (any of characters `'a'`, `'b'`, `'m'`, `'M'`, `'n'`, `'N'`, `'o'`, `'O'`, `'p'`, `'P'`, `'r'`, `'Z'`);
* **Not-range** - represented in the same form as a **Range** but where the first range character is `'^'` and the remaining characters represent a set of characters that cannot appear (at the requisite position) in the input;


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
shwild = { version = "0.2" }
```


## Components

### Constants

The constant `IGNORE_CASE` causes matching to ignore case.


### Enumerations

The `shwild::Error` enum is used to represent a parse result, defined as:

```rust
pub enum Error {
    /// Parse error encountered.
    ParseError {
        line: usize,
        column: usize,
        message: String,
    },
}
```

The `shwild::Result` type is a specialized `std::result::Result` type for
**shwild**, defined as:

```rust
pub type Result<T> = std_result::Result<T, shwild::Error>;
```


### Features

The following crate features are defined:

| Name                    | Effect                                                                           | Is `"default"`? | Dependent feature(s) |
| ----------------------- | -------------------------------------------------------------------------------- | --------------- | -------------------- |
| `"assertions"`          | Provides test assertion macros; enabled by default                              | Yes             |                      |
| `"flexible-flags-type"` | Allows macro flags to use types implementing `base_traits::AsI64`               | **No**          | **base-traits**     |
| `"full"`                | Enables all user-facing runtime features                                         | **No**          |                      |
| `"lookup-ranges"`       | Uses **collect-rs** `UnicodePointMap` for more efficient range matching          | Yes             | **collect-rs**       |
| `"null-feature"`        | Has no effect; useful for simplifying driver scripts                             | **No**          |                      |
| `"test-regex"`          | Enables **regex** support for benchmark and scratch/example programs            | **No**          | **regex**            |


### Functions

The `shwild::matches()` function attempts to parse a `pattern` according to
`flags` and then match the string `input` against it.

```rust
pub fn matches(
    pattern: &str,
    input: &str,
    flags: i64,
) -> Result<bool> {
    // ...
}
```


### Macros

The `shwild::shwild_matches!()` macro is a shorthand for the `shwild::matches()` function, providing 2-parameter and 3-parameter forms. The 2-parameter form passes 0 for the `flags` parameter.

The `shwild::assert_shwild_matches!()` and
`shwild::assert_shwild_not_matches!()` macros are test-oriented counterparts
that panic on failure. Each provides 2-parameter and 3-parameter forms; the
2-parameter form passes 0 for the `flags` parameter. A parse error in the
pattern panics with a descriptive message rather than returning `Err`. They
are provided only when the feature `"assertions"` is enabled, which it is by
default. The optional `"flexible-flags-type"` feature also allows the
3-parameter forms to accept types implementing `base_traits::AsI64`.

```rust
	use shwild::{
		assert_shwild_matches,
		assert_shwild_not_matches,
		IGNORE_CASE,
	};

	assert_shwild_matches!("[a-d]", "b");
	assert_shwild_not_matches!("[a-d]", "e");
	assert_shwild_matches!("[a-d]", "B", IGNORE_CASE);
```


### Structures

The `shwild::CompiledMatcher` structure is the data structure that is used to parse the pattern and then test the input string. Because there is a small, but non-zero, cost to parsing patterns - and complex patterns more so, of course - so if matching is to be repeated in a context where performance costs matter then you may prefer to create an instance of `CompiledMatcher` and then use it to test against, as in:

```rust
	let pattern = r"Where are the* [🐼🐻]s\?";

	let flags = 0;
	let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

	assert!(!matcher.matches(""));
	assert!(!matcher.matches("Where are the bears?"));
	assert!( matcher.matches("Where are the 🐻s?"));
	assert!( matcher.matches("Where are the 🐼s?"));
	assert!( matcher.matches("Where are their 🐻s?"));
	assert!( matcher.matches("Where are the big brown 🐻s?"));
	assert!(!matcher.matches("Where are the teddy-🐻s?"));
```

If you are ever need to get an understanding about the parsed state you can use the `Debug` implementation for the `CompiledMatcher`, as in:

```rust

	// a pattern for rudimentary Windows path names
	let pattern = r"[A-Z]\?*\?*.[ce][ox][em]";

	let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

	eprintln!("matcher={matcher:?}");
```


### Traits

No public traits are defined at this time.


## Examples

Examples are provided in the ```examples``` directory, along with a markdown description for each. A detailed list TOC of them is provided in [EXAMPLES.md](./EXAMPLES.md).


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/shwild.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/shwild.Rust.


### Dependencies

**shwild.Rust** has three optional runtime dependencies:

* [**base-traits**](https://github.com/synesissoftware/base-traits) -
  required if feature `"flexible-flags-type"` is specified, for
  `base_traits::AsI64`;
* [**collect-rs**](https://github.com/synesissoftware/collect-rs) - required
  if feature `"lookup-ranges"` is specified, for more efficient range
  matching;
* [**regex**](https://github.com/rust-lang/regex) - required by some
  benchmark and scratch/example programs if feature `"test-regex"` is
  specified;


#### Dev Dependencies

Crates upon which **shwild** has development dependencies:

* [**criterion**](https://github.com/bheisler/criterion.rs);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);

The committed **Cargo.lock** is retained for reproducible development and CI
builds; locked Cargo commands are used throughout the workflow.


### Related projects

* [**base-traits**](https://github.com/synesissoftware/base-traits/);
* [**collect-rs**](https://github.com/synesissoftware/collect-rs/);
* [**shwild**](https://github.com/synesissoftware/shwild/);
* [**shwild.Go**](https://github.com/synesissoftware/shwild.Go/);


### License

**shwild** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.



<!-- ########################### end of file ########################### -->

