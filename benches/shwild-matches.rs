use shwild;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};


mod constants {
    #![allow(non_upper_case_globals)]

    pub(crate) const EMPTY_STRING : &str = "";
    pub(crate) const S_hello : &str = "hello";
    pub(crate) const S_TQBFJOTLD : &str = "The quick brown fox jumps over the lazy dog";
    pub(crate) const S_Lorem_ipsum : &str =  "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    pub(crate) mod patterns {
        #![allow(non_upper_case_globals)]

        pub(crate) const WINDOWS_PATH : &str = r"[A-Z]\?*\?*.[ce][ox][em]";
    }
}


pub fn matches_input_empty(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let input = "";
    let flags = 0;

    // TODO criterion can benchmark between two different functions
    c.bench_function("`shwild::matches()` - empty string", |b| {
        b.iter(|| {
            let r = shwild::matches(black_box(pattern), black_box(input), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn matches_input_literal_small(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let input = "";
    let flags = 0;

    // TODO criterion can benchmark between two different functions
    c.bench_function("`shwild::matches()` - literal (small)", |b| {
        b.iter(|| {
            let r = shwild::matches(black_box(pattern), black_box(input), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn matches_input_literal_medium(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let input = "";
    let flags = 0;

    // TODO criterion can benchmark between two different functions
    c.bench_function("`shwild::matches()` - literal (medium)", |b| {
        b.iter(|| {
            let r = shwild::matches(black_box(pattern), black_box(input), black_box(flags));

            let _ = black_box(r);
        })
    });
}

pub fn matches_input_literal_large(c : &mut Criterion) {
    let pattern = constants::EMPTY_STRING;
    let input = "";
    let flags = 0;

    // TODO criterion can benchmark between two different functions
    c.bench_function("`shwild::matches()` - literal (large)", |b| {
        b.iter(|| {
            let r = shwild::matches(black_box(pattern), black_box(input), black_box(flags));

            let _ = black_box(r);
        })
    });
}

criterion_group!(
    benches,
    matches_input_empty,
    matches_input_literal_small,
    matches_input_literal_medium,
    matches_input_literal_large,
);
criterion_main!(benches);
