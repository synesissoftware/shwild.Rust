// benches/shwild-compiled_matcher.rs : evaluates performance of `CompiledMatcher`

#![allow(non_snake_case)]

use shwild;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};


mod constants {
    #![allow(non_upper_case_globals)]
    #![allow(unused)]

    pub(crate) const EMPTY_STRING : &str = "";
    pub(crate) const S_hello : &str = "hello";
    pub(crate) const S_TQBFJOTLD : &str = "The quick brown fox jumps over the lazy dog";
    pub(crate) const S_Lorem_ipsum : &str =  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    pub(crate) mod patterns {
        #![allow(non_upper_case_globals)]
        #![allow(unused)]

        pub(crate) const NRANGE_CONTINUUM_SIMPLE : &str = r"[^a-z]";
        pub(crate) const NRANGE_CONTINUUM_REVERSE : &str = r"[^z-a]";
        pub(crate) const NRANGE_CONTINUUM_CROSSCASE : &str = r"[^a-Z]";
        pub(crate) const RANGE_CONTINUUM_SIMPLE : &str = r"[a-z]";
        pub(crate) const RANGE_CONTINUUM_REVERSE : &str = r"[z-a]";
        pub(crate) const RANGE_CONTINUUM_CROSSCASE : &str = r"[a-Z]";

        pub(crate) const WINDOWS_PATH : &str = r"[A-Z]:\\?*\\?*.[ce][ox][em]";
    }
}


pub fn parse_input_empty(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - empty string", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_literal_small(c : &mut Criterion) {
    let pattern = constants::S_hello;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - literal (small)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_literal_medium(c : &mut Criterion) {
    let pattern = constants::S_TQBFJOTLD;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - literal (medium)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_literal_large(c : &mut Criterion) {
    let pattern = constants::S_Lorem_ipsum;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - literal (large)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_nrange_continuum_simple(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_SIMPLE;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - nrange continuum (simple)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_nrange_continuum_reverse(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_REVERSE;
    let flags = 0;

    c.bench_function(
        "`shwild::CompiledMatcher()` parsing - nrange continuum (reverse)",
        |b| {
            b.iter(|| {
                let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

                let _ = black_box(r);
            })
        },
    );
}

pub fn parse_input_nrange_continuum_crosscase(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_CROSSCASE;
    let flags = 0;

    c.bench_function(
        "`shwild::CompiledMatcher()` parsing - nrange continuum (crosscase)",
        |b| {
            b.iter(|| {
                let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

                let _ = black_box(r);
            })
        },
    );
}

pub fn parse_input_range_continuum_simple(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_SIMPLE;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - range continuum (simple)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_range_continuum_reverse(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_REVERSE;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - range continuum (reverse)", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn parse_input_range_continuum_crosscase(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_CROSSCASE;
    let flags = 0;

    c.bench_function(
        "`shwild::CompiledMatcher()` parsing - range continuum (crosscase)",
        |b| {
            b.iter(|| {
                let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

                let _ = black_box(r);
            })
        },
    );
}

pub fn parse_input_Windows_path(c : &mut Criterion) {
    let pattern = constants::patterns::WINDOWS_PATH;
    let flags = 0;

    c.bench_function("`shwild::CompiledMatcher()` parsing - Windows Path", |b| {
        b.iter(|| {
            let r = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn matches_against_input_empty(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        "C:/",
        "C:/dir",
        "C:/dir/stem.com",
        "C:/dir/stem.exe",
        "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name",
        "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name/stem.com",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function("`shwild::CompiledMatcher()` matching - empty string", |b| {
        b.iter(|| {
            for input in &inputs {
                let r = black_box(&matcher).matches(input);

                let _ = black_box(r);
            }
        })
    });
}

pub fn matches_against_nrange_continuum_simple(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_SIMPLE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function(
        "`shwild::CompiledMatcher()` matching - nrange continuum (simple)",
        |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = black_box(&matcher).matches(input);

                    let _ = black_box(r);
                }
            })
        },
    );
}

pub fn matches_against_nrange_continuum_reverse(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_REVERSE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function(
        "`shwild::CompiledMatcher()` matching - range continuum (reverse)",
        |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = black_box(&matcher).matches(input);

                    let _ = black_box(r);
                }
            })
        },
    );
}

pub fn matches_against_nrange_continuum_crosscase(c : &mut Criterion) {
    let pattern = constants::patterns::NRANGE_CONTINUUM_CROSSCASE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function(
        "`shwild::CompiledMatcher()` matching - nrange continuum (crosscase)",
        |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = black_box(&matcher).matches(input);

                    let _ = black_box(r);
                }
            })
        },
    );
}

pub fn matches_against_range_continuum_simple(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_SIMPLE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function("`shwild::CompiledMatcher()` matching - range continuum (simple)", |b| {
        b.iter(|| {
            for input in &inputs {
                let r = black_box(&matcher).matches(input);

                let _ = black_box(r);
            }
        })
    });
}

pub fn matches_against_range_continuum_reverse(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_REVERSE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function(
        "`shwild::CompiledMatcher()` matching - range continuum (reverse)",
        |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = black_box(&matcher).matches(input);

                    let _ = black_box(r);
                }
            })
        },
    );
}

pub fn matches_against_range_continuum_crosscase(c : &mut Criterion) {
    let pattern = constants::patterns::RANGE_CONTINUUM_CROSSCASE;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        " ",
        "a",
        "b",
        "c",
        "d",
        "aa",
        "_",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function(
        "`shwild::CompiledMatcher()` matching - range continuum (crosscase)",
        |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = black_box(&matcher).matches(input);

                    let _ = black_box(r);
                }
            })
        },
    );
}

pub fn matches_against_WindowsPath(c : &mut Criterion) {
    let pattern = constants::patterns::WINDOWS_PATH;
    let flags = 0;

    let inputs = [
        // insert list:
        "",
        "C:/",
        "C:/dir",
        "C:/dir/stem.com",
        "C:/dir/stem.exe",
        "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name",
        "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name/stem.com",
    ];

    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(black_box(pattern), black_box(flags)).unwrap();

    c.bench_function("`shwild::CompiledMatcher()` matching - Windows Path", |b| {
        b.iter(|| {
            for input in &inputs {
                let r = black_box(&matcher).matches(input);

                let _ = black_box(r);
            }
        })
    });
}


criterion_group!(
    benches,
    parse_input_empty,
    parse_input_literal_small,
    parse_input_literal_medium,
    parse_input_literal_large,
    parse_input_nrange_continuum_simple,
    parse_input_nrange_continuum_reverse,
    parse_input_nrange_continuum_crosscase,
    parse_input_range_continuum_simple,
    parse_input_range_continuum_reverse,
    parse_input_range_continuum_crosscase,
    parse_input_Windows_path,
    matches_against_input_empty,
    matches_against_nrange_continuum_simple,
    matches_against_nrange_continuum_reverse,
    matches_against_nrange_continuum_crosscase,
    matches_against_range_continuum_simple,
    matches_against_range_continuum_reverse,
    matches_against_range_continuum_crosscase,
    matches_against_WindowsPath,
);
criterion_main!(benches);
