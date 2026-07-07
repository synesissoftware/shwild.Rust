# shwild.Rust Example - **character-play**

## Summary

A scratch program that exercises **regex** crate matching behaviour for Unicode text, including emoji and combining characters. It is intended to support development of the `"test-regex"` feature and the `regex_comparision_tests` unit tests; it does not call **shwild** APIs directly.


## Source

```Rust
// test/scratch/character-play/main.rs : Unicode matching experiments with **regex**

use regex::Regex;

fn main() {
    {
        let re = Regex::new("abc").unwrap();

        assert!(re.is_match("abc"));
        assert!(re.is_match("abcd"));
    }

    {
        let re = Regex::new("abc$").unwrap();

        assert!(re.is_match("abc"));
        assert!(!re.is_match("abcd"));
    }

    {
        let re = Regex::new("a🐻c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🐻cd"));
    }

    {
        let re = Regex::new("aéc").unwrap();

        assert!(re.is_match("aéc"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("aécd"));
    }

    {
        let re = Regex::new("aéc").unwrap();

        assert!(re.is_match("aéc"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("aécd"));
    }

    {
        let re = Regex::new("a[🐻👀🛑]c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(re.is_match("a👀c"));
        assert!(re.is_match("a🛑c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🛑cd"));
    }

    {
        let re = Regex::new("a👁️c").unwrap();

        assert!(!re.is_match("a🐻c"));
        assert!(!re.is_match("a👀c"));
        assert!(!re.is_match("a🛑c"));
        assert!(re.is_match("a👁️c"));
        assert!(!re.is_match("abc"));
        assert!(!re.is_match("a🛑cd"));
    }

    {
        let re = Regex::new("a[🐻👀🛑]c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(re.is_match("a👀c"));
        assert!(re.is_match("a🛑c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🛑cd"));
    }
}
```


## Running and output

When executed, as in:

```bash
$ cargo run --example character-play --features test-regex
```

it runs to completion with no output on success (all assertions pass).


<!-- ########################### end of file ########################### -->
