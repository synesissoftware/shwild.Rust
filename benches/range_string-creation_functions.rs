// benches/range_string-creation_functions.rs : evaluates performance of different range-string creation-function approaches

#![allow(non_snake_case)]
#![feature(ascii_char)]

use shwild;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    // BenchmarkId,
    Criterion,
};


mod constants {
    #![allow(non_upper_case_globals)]
    #![allow(unused)]

    pub(crate) const EMPTY_STRING : &str = "";
    pub(crate) const RANGE_STRING_abcd : &str = "abcd";
    pub(crate) const RANGE_STRING_ALPHABET : &str = "abcdefghijklmnopqrstuvwxyz";
    pub(crate) const RANGE_STRING_ALPHABET_MIXED_CASE : &str = "abcDefGHijklMNOPqrstUVwXyz";
    pub(crate) const RANGE_STRING_DIGITS : &str = "0123456789";
    pub(crate) const RANGE_STRING_MIXED_ASCII : &str =
        "!\"#$%&'()*+,-./3x0123456789:;<=>?4x@ABCDEFGHIJKLMNO5xPQRSTUVWXYZ[\\]^_6x`abcdefghijklmno7xpqrstuvwxyz{|}~";
}


mod utils {
    #![allow(dead_code)]


    pub(super) fn range_string_from_slice_X(
        chars : &[char],
        flags : i64,
    ) -> String {
        // range_string_from_slice_0(chars, flags)
        // range_string_from_slice_1(chars, flags)
        range_string_from_slice_2(chars, flags)
        // range_string_from_slice_3(chars, flags)
        // range_string_from_slice_4(chars, flags)
        // range_string_from_slice_5(chars, flags)
        // range_string_from_slice_6(chars, flags)
        // range_string_from_slice_7(chars, flags)
    }

    pub(super) fn range_string_from_slice_0(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            for c in chars {
                ci_chars.push(c.to_ascii_uppercase());
                ci_chars.push(c.to_ascii_lowercase());
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect() // appears to be (marginally) faster for `Vec<>`, although it's very close
                                    // chars.as_slice().iter().collect()
    }

    pub(super) fn range_string_from_slice_1(
        chars : &[char],
        flags : i64,
    ) -> String {
        if chars.is_empty() {
            return String::new();
        }

        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            for c in chars {
                ci_chars.push(c.to_ascii_uppercase());
                ci_chars.push(c.to_ascii_lowercase());
            }

            // NOTE: might be quicker to add the chars upper, then sort, then append with lowers

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
    }

    pub(super) fn range_string_from_slice_2(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            for c in chars {
                if c.is_alphabetic() {
                    ci_chars.push(c.to_ascii_uppercase());
                    ci_chars.push(c.to_ascii_lowercase());
                } else {
                    ci_chars.push(*c);
                }

                /* no difference with:
                if c.is_alphabetic() {
                    ci_chars.push(c.to_ascii_uppercase());
                }
                ci_chars.push(c.to_ascii_lowercase());
                 */
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect() // appears to be (marginally) faster for `Vec<>`, although it's very close
                                    // chars.as_slice().iter().collect()
    }

    pub(super) fn range_string_from_slice_3(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            for &c in chars {
                ci_chars.push(c.to_ascii_uppercase());
            }

            ci_chars.sort_unstable();

            for &c in chars {
                if c.is_ascii_alphabetic() {
                    ci_chars.push(c.to_ascii_lowercase());
                }
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
        // chars.as_slice().iter().collect()
    }

    pub(super) fn range_string_from_slice_4(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            if chars.len() > 10 {
                for c in chars {
                    if c.is_alphabetic() {
                        ci_chars.push(c.to_ascii_uppercase());
                        ci_chars.push(c.to_ascii_lowercase());
                    } else {
                        ci_chars.push(*c);
                    }
                }
            } else {
                for &c in chars {
                    ci_chars.push(c.to_ascii_uppercase());
                }

                ci_chars.sort_unstable();

                for &c in chars {
                    if c.is_ascii_alphabetic() {
                        ci_chars.push(c.to_ascii_lowercase());
                    }
                }
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
        // chars.as_slice().iter().collect()
    }

    pub(super) fn range_string_from_slice_5(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = Vec::with_capacity(chars.len() * 2);

            ci_chars.resize(chars.len(), '\0');

            for (ix, &c) in chars.iter().enumerate() {
                if c.is_alphabetic() {
                    ci_chars.push(c.to_ascii_uppercase());
                }
                ci_chars[ix] = c.to_ascii_lowercase();
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
        // chars.as_slice().iter().collect()
    }

    #[allow(unexpected_cfgs)]
    #[cfg(feature = "auto-buffer")]
    pub(super) fn range_string_from_slice_6(
        chars : &[char],
        flags : i64,
    ) -> String {
        let mut chars = if 0 != (shwild::IGNORE_CASE & flags) {
            let mut ci_chars = auto_buffer::AutoBuffer::<char, 50>::with_capacity(chars.len() * 2);

            for (ix, c) in chars.iter().enumerate() {
                if c.is_alphabetic() {
                    ci_chars.push(c.to_ascii_uppercase());

                    ci_chars.push(c.to_ascii_lowercase());
                } else {
                    ci_chars.push(*c);
                }
            }

            ci_chars
        } else {
            chars.into()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
    }

    pub(super) fn range_string_from_slice_7(
        chars : &[char],
        flags : i64,
    ) -> String {
        if 0 != (shwild::IGNORE_CASE & flags) {
            let mut bytes = Vec::<u8>::with_capacity(chars.len() * 2);

            for c in chars {
                if c.is_alphabetic() {
                    bytes.push(c.to_ascii_uppercase() as u8);
                }
                bytes.push(c.to_ascii_lowercase() as u8);
            }

            bytes.sort_unstable();

            bytes.dedup();

            unsafe { String::from_utf8_unchecked(bytes) }
        } else {
            let mut bytes : Vec<u8> = Vec::with_capacity(chars.len());

            for c in chars {
                bytes.push(*c as u8);
            }

            bytes.sort_unstable();

            bytes.dedup();

            unsafe { String::from_utf8_unchecked(bytes) }
        }
    }
}


mod implementation {
    use criterion::BenchmarkId;


    pub(super) fn make_id(
        benchmarked_function_name : &'static str,
        input_string : &str,
        flags : i64,
    ) -> BenchmarkId {
        let parameter = if 0 != (flags & shwild::IGNORE_CASE) {
            format!("I:'{input_string}'")
        } else {
            format!("'{input_string}'")
        };

        BenchmarkId::new(format!("`{benchmarked_function_name}()`"), parameter)
    }
}


fn BENCHMARK_range_string_from_slice_X_WITH_(
    c : &mut Criterion,
    input_string : &str,
    flags : i64,
) {
    let input = input_string.chars().collect::<Vec<_>>();

    let id = implementation::make_id("range_string_from_slice_X", input_string, flags);

    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| {
            let s = utils::range_string_from_slice_X(black_box(input.as_slice()), black_box(flags));

            let _ = black_box(s);
        })
    });
}

fn BENCHMARK_range_string_from_slice_1_WITH_(
    c : &mut Criterion,
    input_string : &str,
    flags : i64,
) {
    let input = input_string.chars().collect::<Vec<_>>();

    let id = implementation::make_id("range_string_from_slice_1", input_string, flags);

    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| {
            let s = utils::range_string_from_slice_1(black_box(input.as_slice()), black_box(flags));

            let _ = black_box(s);
        })
    });
}

fn BENCHMARK_range_string_from_slice_2_WITH_(
    c : &mut Criterion,
    input_string : &str,
    flags : i64,
) {
    let input = input_string.chars().collect::<Vec<_>>();

    let id = implementation::make_id("range_string_from_slice_2", input_string, flags);

    c.bench_with_input(id, &input, |b, input| {
        b.iter(|| {
            let s = utils::range_string_from_slice_2(black_box(input.as_slice()), black_box(flags));

            let _ = black_box(s);
        })
    });
}


pub fn BENCHMARK_range_string_from_slice_X_WITH_EMPTY_STRING(c : &mut Criterion) {
    let input_string = constants::EMPTY_STRING;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_abcd(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_abcd_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_ALPHABET(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_MIXED_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_DIGITS(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_DIGITS;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_DIGITS_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_DIGITS;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_MIXED_ASCII(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_MIXED_ASCII;
    let flags = 0;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_X_WITH_MIXED_ASCII_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_MIXED_ASCII;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_X_WITH_(c, input_string, flags);
}


pub fn BENCHMARK_range_string_from_slice_1_WITH_EMPTY_STRING(c : &mut Criterion) {
    let input_string = constants::EMPTY_STRING;
    let flags = 0;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_abcd(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = 0;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_abcd_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_ALPHABET(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = 0;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_MIXED_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = 0;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_1_WITH_(c, input_string, flags);
}


pub fn BENCHMARK_range_string_from_slice_2_WITH_EMPTY_STRING(c : &mut Criterion) {
    let input_string = constants::EMPTY_STRING;
    let flags = 0;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_abcd(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = 0;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_abcd_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_abcd;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_ALPHABET(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = 0;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_MIXED_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = 0;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}
pub fn BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE(c : &mut Criterion) {
    let input_string = constants::RANGE_STRING_ALPHABET_MIXED_CASE;
    let flags = shwild::IGNORE_CASE;

    BENCHMARK_range_string_from_slice_2_WITH_(c, input_string, flags);
}


criterion_group!(
    benches,
    // range_string_from_slice_X
    BENCHMARK_range_string_from_slice_X_WITH_EMPTY_STRING,
    BENCHMARK_range_string_from_slice_X_WITH_abcd,
    BENCHMARK_range_string_from_slice_X_WITH_abcd_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_X_WITH_ALPHABET,
    BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_MIXED_CASE,
    BENCHMARK_range_string_from_slice_X_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_X_WITH_DIGITS,
    BENCHMARK_range_string_from_slice_X_WITH_DIGITS_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_X_WITH_MIXED_ASCII,
    BENCHMARK_range_string_from_slice_X_WITH_MIXED_ASCII_IGNORING_CASE,
    // range_string_from_slice_1
    BENCHMARK_range_string_from_slice_1_WITH_EMPTY_STRING,
    BENCHMARK_range_string_from_slice_1_WITH_abcd,
    BENCHMARK_range_string_from_slice_1_WITH_abcd_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_1_WITH_ALPHABET,
    BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_MIXED_CASE,
    BENCHMARK_range_string_from_slice_1_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE,
    // range_string_from_slice_2
    BENCHMARK_range_string_from_slice_2_WITH_EMPTY_STRING,
    BENCHMARK_range_string_from_slice_2_WITH_abcd,
    BENCHMARK_range_string_from_slice_2_WITH_abcd_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_2_WITH_ALPHABET,
    BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_IGNORING_CASE,
    BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_MIXED_CASE,
    BENCHMARK_range_string_from_slice_2_WITH_ALPHABET_MIXED_CASE_IGNORING_CASE,
);
criterion_main!(benches);
