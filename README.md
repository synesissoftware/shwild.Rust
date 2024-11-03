# shwild <!-- omit in toc -->

**SH**ell-compatible **WILD**cards, for **Rust**.

[![Crates.io](https://img.shields.io/crates/v/shwild.svg)](https://crates.io/crates/shwild)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
	- [Pattern Elements](#pattern-elements)
- [Installation](#installation)
- [Components](#components)
	- [Constants](#constants)
	- [Enumerations](#enumerations)
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

**shwild** is a small, standalone library, implemented in C++ with a C and a C++ API, that provides shell-compatible wildcard matching.

**shwild.Rust** is a **Rust** port, with minimal API differences. The design emphasis is on simplicity-of-use, modularity, and performance.

```Rust
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
* **Wild-1** - represented by the single character `'?'` in the pattern, which represents a match of exactly any one character. In the above example `r"Where are the* [🐼🐻]s\?"` the `'?'` is _not_ interpreted as a wild-1 because it is escaped by the `'\'` character;
* **Wild-N** - represented by the single character `'*'` in the pattern, which represents a match of any number of characters;
* **Range** - represented by a sequence of characters within `'['` and `']'`, as in the `"[🐼🐻]"` fragment in the above example, which will match to any one of range character in the input. As well as an unordered sequence of literal characters, ranges may also capture contiguous sequences, as in `"[zc-aja]"` (any of characters `'a'`, `'b'`, `'c'`, `'j'`, `'z'`) or in `"[abm-PrZ]"` (any of characters `'a'`, `'b'`, `'m'`, `'M'`, `'n'`, `'N'`, `'o'`, `'O'`, `'p'`, `'P'`, `'r'`, `'Z'`);
* **Not-range** - represented in the same form as a **Range** but where the first range character is `'^'` and the remaining characters represent a set of characters that cannot appear (at the requisite position) in the input;


## Installation

Reference in **Cargo.toml** in the usual way:

```toml
shwild = { version = "~0.1" }
```


## Components

### Constants

The constant `IGNORE_CASE` causes matching to ignore case.


### Enumerations

The `shwild::Error` enum is used to represent a parse result, defined as:

```Rust
pub enum Error {
    /// Parse error encountered.
    ParseError {
        line :    usize,
        column :  usize,
        message : String,
    },
}
```

The `shwild::Result` enum is a specialized `std::result::Result` type for **shwild**, defined as:

```Rust
pub type Result<T> = std_result::Result<T, shwild::Error>;
```


### Functions

The `shwild::matches()` function attempts to parse a `pattern` according to `flags` and then match against it the string `input`.

```Rust
pub mod shwild {

	pub fn matches(
		pattern : &str,
		input : &str,
		flags : i64,
	) -> Result<bool>;
}
```


### Macros

The `shwild::shwild_matches!()` macro is a shorthand for the `shwild::matches()` function, providing 2-parameter and 3-parameter forms. The 2-parameter form passes 0 for the `flags` parameter.


### Structures

The `shwild::CompiledMatcher` structure is the data structure that is used to parse the pattern and then test the input string. Because there is a small, but non-zero, cost to parsing patterns - and complex patterns more so, of course - so if matching is to be repeated in a context where performance costs matter then you may prefer to create an instance of `CompiledMatcher` and then use it to test against, as in:

```Rust
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

```Rust

	// a pattern for rudimentary Windows path names
	let pattern = r"[A-Z]\?*\?*.[ce][ox][em]";

	let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

	eprintln!("matcher={matcher:?}");
```


### Traits

No public traits are defined at this time.


## Examples

T.B.C.


## Project Information

### Where to get help

[GitHub Page](https://github.com/synesissoftware/shwild.Rust "GitHub Page")


### Contribution guidelines

Defect reports, feature requests, and pull requests are welcome on https://github.com/synesissoftware/shwild.Rust.


### Dependencies

There are no external dependencies.


#### Dev Dependencies

Crates upon which **shwild** has development dependencies:

* [**criterion**](https://github.com/bheisler/criterion.rs);
* [**test_help-rs**](https://github.com/synesissoftware/test_help-rs);


### Related projects

None at this time.


### License

**shwild** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->

