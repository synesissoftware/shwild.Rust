# shwild.Rust Example - **list-matching-files**

## Summary

An example using **shwild.Rust**'s `matches()` function to list files in the current directory whose paths match one or more shell wildcard pattern(s) given on the command-line. When no patterns are specified, `"*"` is assumed.


## Source

```Rust
// examples/list-matching-files/main.rs : filter files by name using `matches()`

use std::{
    env as std_env,
    fs as std_fs,
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

    println!("searching in '{directory}' with pattern(s) {patterns:?}");

    match std_fs::read_dir(directory) {
        Ok(entries) => {
            // for each file in the directory ...
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let path_s = format!("{}", path.display());

                        // ... check against ...
                        for pattern in &patterns {
                            // ... each pattern ...
                            match shwild::matches(pattern, &path_s, 0) {
                                Ok(is_matched) => {
                                    if is_matched {
                                        // ... and print when it matches any one.
                                        println!("\t{path_s}");

                                        break;
                                    }
                                },
                                Err(e) => {
                                    eprintln!("failed to match against '{path_s}': {e}");
                                },
                            };
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
