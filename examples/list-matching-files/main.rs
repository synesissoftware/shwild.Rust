// Example program illustrating use of shwild to filter by name files found
// in the executing directory

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
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();

                        let path_s = format!("{}", path.display());

                        for pattern in &patterns {
                            match shwild::matches(pattern, &path_s, 0) {
                                Ok(is_matched) => {
                                    if is_matched {
                                        println!("\t{path_s}");

                                        break;
                                    }
                                },
                                Err(e) => {
                                    eprintln!("failed to match against '{path_s}': {e}");
                                },
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("failed to read file in '{directory}': {e}");
                    },
                }
            }
        },
        Err(e) => {
            eprintln!("failed to read files in '{directory}': {e}");
        },
    }
}
