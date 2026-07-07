# shwild.Rust Example - **list-matching-files-compiled**

## Summary

An example using **shwild.Rust**'s `CompiledMatcher` to list files in the current directory whose paths match one or more shell wildcard pattern(s) given on the command-line. Each pattern is parsed once up front; this is preferable when the same pattern(s) will be matched repeatedly. When no patterns are specified, `"*"` is assumed.


## Source

```Rust
// examples/list-matching-files-compiled/main.rs : filter files using `CompiledMatcher`

use std::{
    env as std_env,
    fs as std_fs,
    process as std_process,
};


fn main() {
    let directory = ".";

    let patterns = {
        let r = std_env::args().skip(1).collect::<Vec<_>>();

        if r.is_empty() {
            vec!["*".into()]
        } else {
            r
        }
    };
    let matchers = patterns
        .iter()
        .map(|pattern| {
            shwild::CompiledMatcher::from_pattern_and_flags(&pattern, 0).unwrap_or_else(|e| {
                eprintln!("failed to parse pattern '{pattern}': {e}");

                std_process::exit(1);
            })
        })
        .collect::<Vec<_>>();

    println!("searching in '{directory}' with pattern(s) {:?}", patterns);

    match std_fs::read_dir(directory) {
        Ok(entries) => {
            // for each file in the directory ...
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let path_s = format!("{}", path.display());

                        // ... check against ...
                        for matcher in &matchers {
                            // ... each pattern ...
                            if matcher.matches(&path_s) {
                                // ... and print when it matches any one.
                                println!("\t{path_s}");

                                break;
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("failed to read file in '{directory}': {e}");
                    },
                };
            }
        },
        Err(e) => {
            eprintln!("failed to read files in '{directory}': {e}");
        },
    };
}
```


## Running and output

When executed, as in:

```bash
$ cargo run --example list-matching-files-compiled
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
$ cargo run --example list-matching-files-compiled -- '*.md'
```

it gives output similar to:

```
searching in '.' with pattern(s) ["*.md"]
	./README.md
	./CHANGES.md
	./TODO.md
```


<!-- ########################### end of file ########################### -->
