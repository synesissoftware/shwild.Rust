// Example program illustrating use of shwild to filter by name files found
// in the executing directory

use std::{
    env as std_env,
    fs as std_fs,
    process as std_process,
};


fn main() {
    let directory = ".";

    let matchers = {
        let r = std_env::args().skip(1).collect::<Vec<_>>();

        let r = if r.is_empty() { vec!["*".into()] } else { r };

        r.into_iter()
            .map(|pattern| {
                let m = shwild::CompiledMatcher::from_pattern_and_flags(&pattern, 0).unwrap_or_else(|e| {
                    eprintln!("failed to parse pattern '{pattern}': {e}");

                    std_process::exit(1);
                });

                m
            })
            .collect::<Vec<_>>()
    };

    // println!("searching in '{directory}' with pattern(s) {patterns:?}");

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
