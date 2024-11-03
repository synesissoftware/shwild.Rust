// benches/cw-regex.rs : evaluates parsing and matching costs compared-with regular expressions

#![allow(non_snake_case)]

use shwild;

use regex::Regex;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};


mod constants {
    pub(crate) const S_TQBFJOTLD : &str = "The quick brown fox jumps over the lazy dog";

    pub(crate) const S_BROWN_STRINGS : &[&str] = &[
        // insert list:
        S_TQBFJOTLD,
        "brown",
        "brownian motion",
    ];
}


mod implementation {
    use criterion::BenchmarkId;


    pub(super) fn make_id(
        benchmarked_function_name : &str,
        pattern : &str,
    ) -> BenchmarkId {
        BenchmarkId::new(format!("`{benchmarked_function_name}()`"), pattern)
    }
}


// parsing

fn BENCHMARK_Regex_PARSE_brown_1(c : &mut Criterion) {
    let pattern = "brown";

    c.bench_function("`BENCHMARK_Regex_PARSE_brown_1()`", |b| {
        b.iter(|| {
            let re = Regex::new(black_box(pattern));

            let _ = black_box(re);
        })
    });
}

fn BENCHMARK_shwild_PARSE_brown_1(c : &mut Criterion) {
    let pattern = "*brown*";

    c.bench_function("`BENCHMARK_shwild_PARSE_brown_1()`", |b| {
        b.iter(|| {
            let cm = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), 0);

            let _ = black_box(cm);
        })
    });
}

fn BENCHMARK_Regex_PARSE_brown_2(c : &mut Criterion) {
    let pattern = "^brown$";

    c.bench_function("`BENCHMARK_Regex_PARSE_brown_2()`", |b| {
        b.iter(|| {
            let re = Regex::new(black_box(pattern));

            let _ = black_box(re);
        })
    });
}

fn BENCHMARK_shwild_PARSE_brown_2(c : &mut Criterion) {
    let pattern = "brown";

    c.bench_function("`BENCHMARK_shwild_PARSE_brown_2()`", |b| {
        b.iter(|| {
            let cm = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), 0);

            let _ = black_box(cm);
        })
    });
}


// matching

fn BENCHMARK_Regex_MATCH_brown_1(c : &mut Criterion) {
    let pattern = "brown";
    let re = Regex::new(pattern).unwrap();

    let id = implementation::make_id("BENCHMARK_Regex_MATCH_brown_1", pattern);

    c.bench_with_input(id, &re, |b, re| {
        b.iter(|| {
            let mut n = 0;

            for s in constants::S_BROWN_STRINGS {
                if re.is_match(black_box(s)) {
                    n += 1;
                }
            }

            let _ = black_box(n);
        })
    });
}

fn BENCHMARK_shwild_MATCH_brown_1(c : &mut Criterion) {
    let pattern = "*brown*";
    let cm = shwild::CompiledMatcher::from_pattern_and_flags(pattern, 0).unwrap();

    let id = implementation::make_id("BENCHMARK_shwild_MATCH_brown_1", pattern);

    c.bench_with_input(id, &cm, |b, cm| {
        b.iter(|| {
            let mut n = 0;

            for s in constants::S_BROWN_STRINGS {
                if cm.matches(black_box(s)) {
                    n += 1;
                }
            }

            let _ = black_box(n);
        })
    });
}

fn BENCHMARK_Regex_MATCH_brown_2(c : &mut Criterion) {
    let pattern = "^brown$";
    let re = Regex::new(pattern).unwrap();

    let id = implementation::make_id("BENCHMARK_Regex_MATCH_brown_2", pattern);

    c.bench_with_input(id, &re, |b, re| {
        b.iter(|| {
            let mut n = 0;

            for s in constants::S_BROWN_STRINGS {
                if re.is_match(black_box(s)) {
                    n += 1;
                }
            }

            let _ = black_box(n);
        })
    });
}

fn BENCHMARK_shwild_MATCH_brown_2(c : &mut Criterion) {
    let pattern = "brown";
    let cm = shwild::CompiledMatcher::from_pattern_and_flags(pattern, 0).unwrap();

    let id = implementation::make_id("BENCHMARK_shwild_MATCH_brown_2", pattern);

    c.bench_with_input(id, &cm, |b, cm| {
        b.iter(|| {
            let mut n = 0;

            for s in constants::S_BROWN_STRINGS {
                if cm.matches(black_box(s)) {
                    n += 1;
                }
            }

            let _ = black_box(n);
        })
    });
}


criterion_group!(
    benches,
    // parsing
    BENCHMARK_Regex_PARSE_brown_1,
    BENCHMARK_shwild_PARSE_brown_1,
    BENCHMARK_Regex_PARSE_brown_2,
    BENCHMARK_shwild_PARSE_brown_2,
    // matching
    BENCHMARK_Regex_MATCH_brown_1,
    BENCHMARK_shwild_MATCH_brown_1,
    BENCHMARK_Regex_MATCH_brown_2,
    BENCHMARK_shwild_MATCH_brown_2,
);
criterion_main!(benches);


/* ///////////////////////////// end of file //////////////////////////// */
