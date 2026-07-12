// benches/macros.rs : evaluates performance of matching macros

#![allow(non_snake_case)]

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};

use std::hint::black_box;

use shwild::shwild_matches;

#[cfg(feature = "assertions")]
use shwild::{
    assert_shwild_matches,
    assert_shwild_not_matches,
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

    pub(crate) mod windows_path_inputs {
        #![allow(non_upper_case_globals)]
        #![allow(unused)]

        pub(crate) const ALL : [&str; 7] = [
            "",
            "C:/",
            "C:/dir",
            "C:/dir/stem.com",
            "C:/dir/stem.exe",
            "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name",
            "C:/directory-with-a-veeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeerrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrryyyyyyyyyyyyyyyyyyyyyyyyyyyyyy-long-name/stem.com",
        ];

        pub(crate) const MATCHING : [&str; 3] = [
            r"C:\directory\file.exe",
            r"X:\dir\filestem.exe",
            r"D:\dir\sub-dir\filestem.exe",
        ];

        pub(crate) const NOT_MATCHING : [&str; 4] = [
            "",
            r"X:\filestem.exe",
            r"_:\dir\filestem.exe",
            r"D:\dir\sub-dir\filestem.bat",
        ];
    }
}

mod macros_benches {
    #![allow(non_snake_case)]

    use super::*;


    pub fn shwild_matches_input_empty(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";
        let flags = 0;

        c.bench_function("`shwild_matches!()` - empty string", |b| {
            b.iter(|| {
                let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                let _ = black_box(r);
            })
        });
    }

    pub fn shwild_matches_input_literal_small(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";
        let flags = 0;

        c.bench_function("`shwild_matches!()` - literal (small)", |b| {
            b.iter(|| {
                let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                let _ = black_box(r);
            })
        });
    }

    pub fn shwild_matches_input_literal_medium(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";
        let flags = 0;

        c.bench_function("`shwild_matches!()` - literal (medium)", |b| {
            b.iter(|| {
                let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                let _ = black_box(r);
            })
        });
    }

    pub fn shwild_matches_input_literal_large(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";
        let flags = 0;

        c.bench_function("`shwild_matches!()` - literal (large)", |b| {
            b.iter(|| {
                let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                let _ = black_box(r);
            })
        });
    }

    pub fn shwild_matches_against_nrange_continuum_simple(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - nrange continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_against_nrange_continuum_reverse(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - nrange continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_against_nrange_continuum_crosscase(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - nrange continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_against_range_continuum_simple(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - range continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_against_range_continuum_reverse(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - range continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_against_range_continuum_crosscase(c : &mut Criterion) {
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

        c.bench_function("`shwild_matches!()` - range continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }

    pub fn shwild_matches_test_against_pattern_WindowsPath(c : &mut Criterion) {
        let pattern = constants::patterns::WINDOWS_PATH;
        let flags = 0;
        let inputs = constants::windows_path_inputs::ALL;

        c.bench_function("`shwild_matches!()` - Windows Path", |b| {
            b.iter(|| {
                for input in &inputs {
                    let r = shwild_matches!(black_box(pattern), black_box(input), black_box(flags));

                    let _ = black_box(r);
                }
            })
        });
    }
}



#[cfg(feature = "assertions")]
mod assertion_benches {
    #![allow(non_snake_case)]

    use super::*;


    pub fn assert_shwild_matches_input_empty(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";

        c.bench_function("`assert_shwild_matches!()` - empty string", |b| {
            b.iter(|| {
                assert_shwild_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_matches_input_literal_small(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";

        c.bench_function("`assert_shwild_matches!()` - literal (small)", |b| {
            b.iter(|| {
                assert_shwild_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_matches_input_literal_medium(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";

        c.bench_function("`assert_shwild_matches!()` - literal (medium)", |b| {
            b.iter(|| {
                assert_shwild_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_matches_input_literal_large(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = "";

        c.bench_function("`assert_shwild_matches!()` - literal (large)", |b| {
            b.iter(|| {
                assert_shwild_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_matches_against_nrange_continuum_simple(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_SIMPLE;

        let inputs = [
            // insert list:
            " ",
            "_",
        ];

        c.bench_function("`assert_shwild_matches!()` - nrange continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_against_nrange_continuum_reverse(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_REVERSE;

        let inputs = [
            // insert list:
            " ",
            "_",
        ];

        c.bench_function("`assert_shwild_matches!()` - nrange continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_against_nrange_continuum_crosscase(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_CROSSCASE;

        let inputs = [
            // insert list:
            " ",
            "_",
        ];

        c.bench_function("`assert_shwild_matches!()` - nrange continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_against_range_continuum_simple(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_SIMPLE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
        ];

        c.bench_function("`assert_shwild_matches!()` - range continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_against_range_continuum_reverse(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_REVERSE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
        ];

        c.bench_function("`assert_shwild_matches!()` - range continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_against_range_continuum_crosscase(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_CROSSCASE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
            "A",
            "B",
            "C",
            "D",
        ];

        c.bench_function("`assert_shwild_matches!()` - range continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_matches_test_against_pattern_WindowsPath(c : &mut Criterion) {
        let pattern = constants::patterns::WINDOWS_PATH;
        let inputs = constants::windows_path_inputs::MATCHING;

        c.bench_function("`assert_shwild_matches!()` - Windows Path", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_input_empty(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = " ";

        c.bench_function("`assert_shwild_not_matches!()` - empty string", |b| {
            b.iter(|| {
                assert_shwild_not_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_not_matches_input_literal_small(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = " ";

        c.bench_function("`assert_shwild_not_matches!()` - literal (small)", |b| {
            b.iter(|| {
                assert_shwild_not_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_not_matches_input_literal_medium(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = " ";

        c.bench_function("`assert_shwild_not_matches!()` - literal (medium)", |b| {
            b.iter(|| {
                assert_shwild_not_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_not_matches_input_literal_large(c : &mut Criterion) {
        let pattern = constants::EMPTY_STRING;
        let input = " ";

        c.bench_function("`assert_shwild_not_matches!()` - literal (large)", |b| {
            b.iter(|| {
                assert_shwild_not_matches!(black_box(pattern), black_box(input));
            })
        });
    }

    pub fn assert_shwild_not_matches_against_nrange_continuum_simple(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_SIMPLE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
            "",
            "aa",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - nrange continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_against_nrange_continuum_reverse(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_REVERSE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
            "",
            "aa",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - nrange continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_against_nrange_continuum_crosscase(c : &mut Criterion) {
        let pattern = constants::patterns::NRANGE_CONTINUUM_CROSSCASE;

        let inputs = [
            // insert list:
            "a",
            "b",
            "c",
            "d",
            "",
            "aa",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - nrange continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_against_range_continuum_simple(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_SIMPLE;

        let inputs = [
            // insert list:
            "",
            " ",
            "aa",
            "_",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - range continuum (simple)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_against_range_continuum_reverse(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_REVERSE;

        let inputs = [
            // insert list:
            "",
            " ",
            "aa",
            "_",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - range continuum (reverse)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_against_range_continuum_crosscase(c : &mut Criterion) {
        let pattern = constants::patterns::RANGE_CONTINUUM_CROSSCASE;

        let inputs = [
            // insert list:
            "",
            " ",
            "aa",
            "_",
        ];

        c.bench_function("`assert_shwild_not_matches!()` - range continuum (crosscase)", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }

    pub fn assert_shwild_not_matches_test_against_pattern_WindowsPath(c : &mut Criterion) {
        let pattern = constants::patterns::WINDOWS_PATH;
        let inputs = constants::windows_path_inputs::NOT_MATCHING;

        c.bench_function("`assert_shwild_not_matches!()` - Windows Path", |b| {
            b.iter(|| {
                for input in &inputs {
                    assert_shwild_not_matches!(black_box(pattern), black_box(input));
                }
            })
        });
    }
}


criterion_group!(
    shwild_matches_benches,
    macros_benches::shwild_matches_input_empty,
    macros_benches::shwild_matches_input_literal_small,
    macros_benches::shwild_matches_input_literal_medium,
    macros_benches::shwild_matches_input_literal_large,
    macros_benches::shwild_matches_test_against_pattern_WindowsPath,
    macros_benches::shwild_matches_against_nrange_continuum_simple,
    macros_benches::shwild_matches_against_nrange_continuum_reverse,
    macros_benches::shwild_matches_against_nrange_continuum_crosscase,
    macros_benches::shwild_matches_against_range_continuum_simple,
    macros_benches::shwild_matches_against_range_continuum_reverse,
    macros_benches::shwild_matches_against_range_continuum_crosscase,
);

#[cfg(feature = "assertions")]
criterion_group!(
    assert_shwild_matches_benches,
    assertion_benches::assert_shwild_matches_input_empty,
    assertion_benches::assert_shwild_matches_input_literal_small,
    assertion_benches::assert_shwild_matches_input_literal_medium,
    assertion_benches::assert_shwild_matches_input_literal_large,
    assertion_benches::assert_shwild_matches_test_against_pattern_WindowsPath,
    assertion_benches::assert_shwild_matches_against_nrange_continuum_simple,
    assertion_benches::assert_shwild_matches_against_nrange_continuum_reverse,
    assertion_benches::assert_shwild_matches_against_nrange_continuum_crosscase,
    assertion_benches::assert_shwild_matches_against_range_continuum_simple,
    assertion_benches::assert_shwild_matches_against_range_continuum_reverse,
    assertion_benches::assert_shwild_matches_against_range_continuum_crosscase,
);

#[cfg(feature = "assertions")]
criterion_group!(
    assert_shwild_not_matches_benches,
    assertion_benches::assert_shwild_not_matches_input_empty,
    assertion_benches::assert_shwild_not_matches_input_literal_small,
    assertion_benches::assert_shwild_not_matches_input_literal_medium,
    assertion_benches::assert_shwild_not_matches_input_literal_large,
    assertion_benches::assert_shwild_not_matches_test_against_pattern_WindowsPath,
    assertion_benches::assert_shwild_not_matches_against_nrange_continuum_simple,
    assertion_benches::assert_shwild_not_matches_against_nrange_continuum_reverse,
    assertion_benches::assert_shwild_not_matches_against_nrange_continuum_crosscase,
    assertion_benches::assert_shwild_not_matches_against_range_continuum_simple,
    assertion_benches::assert_shwild_not_matches_against_range_continuum_reverse,
    assertion_benches::assert_shwild_not_matches_against_range_continuum_crosscase,
);

#[cfg(feature = "assertions")]
criterion_main!(
    shwild_matches_benches,
    assert_shwild_matches_benches,
    assert_shwild_not_matches_benches,
);

#[cfg(not(feature = "assertions"))]
criterion_main!(shwild_matches_benches);
