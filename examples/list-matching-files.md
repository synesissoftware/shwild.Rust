# shwild.Rust - Example - **list-matching-files**

## Summary

An example using **shwild.Rust**'s `matches()` function to list files in the current directory whose paths match one or more shell wildcard pattern(s) given on the command-line. When no patterns are specified, `"*"` is assumed.


## Source

[Source](./list-matching-files/main.rs)


## Execution

When executed, as in:

```bash
$ cargo run --example list-matching-files
```

it gives output similar to:

```
searching in '.' with pattern(s) ["*"]
	./Cargo.toml
	./LICENSE
	./README.md
	...
```

With pattern argument(s), as in:

```bash
$ cargo run --example list-matching-files -- '*.md'
```

it gives output similar to:

```
searching in '.' with pattern(s) ["*.md"]
	./README.md
	./CHANGES.md
	./TODO.md
```


<!-- ########################### end of file ########################### -->
