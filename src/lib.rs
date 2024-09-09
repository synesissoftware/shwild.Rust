// Definition of the shwild Rust package

use std::{
    error as std_error,
    fmt as std_fmt,
    result as std_result,
};


/// Represents parsing result.
#[derive(PartialEq)]
pub enum Error {
    /// Parsing was successful.
    Ok,
    /// Parse error encountered.
    ParseError {
        line :    usize,
        column :  usize,
        message : String,
    },
}

// API functions
impl Error {
}

// Mutating methods
impl Error {
}

// Non-mutating methods
impl Error {
    #![allow(non_snake_case)]

    /// Until we determine a need to separate them, `Debug` and `Display`
    /// implementations produce same representation.
    fn fmt_for_Debug_or_Display(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        match self {
            Self::Ok => write!(f, "the operating completed successfully"),
            Self::ParseError {
                message, ..
            } => {
                write!(f, "{message}")
            },
        }
    }
}

// Trait implementations

impl std_fmt::Debug for Error {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        self.fmt_for_Debug_or_Display(f)
    }
}

impl std_fmt::Display for Error {
    fn fmt(
        &self,
        f : &mut std_fmt::Formatter<'_>,
    ) -> std_fmt::Result {
        self.fmt_for_Debug_or_Display(f)
    }
}

impl std_error::Error for Error {
}


mod constants {

    pub const IGNORE_CASE : i64 = 0x0200;
}

pub use constants::IGNORE_CASE;


mod traits {
    #![allow(non_snake_case)]

    use std::fmt as std_fmt;


    /// Defines behaviour for specific matchers.
    pub(crate) trait Match: std_fmt::Debug {
        /// Attempts to match the input string `slice` against this `Match`
        /// instance and, implicitly, any following `Match` instances.
        ///
        /// # Returns:
        /// - `true` - indicates a full match; or
        /// - `false` - if not a full match.

        fn matches(
            &self,
            slice : &str,
        ) -> bool;
    }
}

mod match_structures {

    use super::traits::Match;


    #[derive(Debug)]
    pub(crate) struct MatchEnd {}

    #[derive(Debug)]
    pub(crate) struct MatchLiteral {
        next :              Option<Box<dyn Match>>,
        literal :           String,
        literal_uppercase : Option<String>,
        #[cfg_attr(debug_assertions, allow(unused))]
        minimum_required :  usize,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchNotRange {
        next :             Option<Box<dyn Match>>,
        characters :       String,
        #[cfg_attr(debug_assertions, allow(unused))]
        minimum_required : usize,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchRange {
        next :             Option<Box<dyn Match>>,
        characters :       String,
        #[cfg_attr(debug_assertions, allow(unused))]
        minimum_required : usize,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchWild1 {
        pub(crate) next :  Option<Box<dyn Match>>,
        #[cfg_attr(debug_assertions, allow(unused))]
        minimum_required : usize,
    }

    #[derive(Debug)]
    pub(crate) struct MatchWildN {
        pub(crate) next :  Option<Box<dyn Match>>,
        #[cfg_attr(debug_assertions, allow(unused))]
        minimum_required : usize,
    }


    // API functions

    impl MatchLiteral {
        pub(crate) fn new(
            next : Option<Box<dyn Match>>,
            literal : String,
            flags : i64,
        ) -> Self {
            let literal_uppercase = if 0 != (flags & super::constants::IGNORE_CASE) {
                Some(literal.to_uppercase())
            } else {
                None
            };

            // NOTE: this is a not-currently-implemented feature
            let minimum_required = usize::MAX;

            Self {
                next,
                literal,
                literal_uppercase,
                // flags,
                minimum_required,
            }
        }
    }

    impl MatchNotRange {
        pub(crate) fn new(
            next : Option<Box<dyn Match>>,
            characters : String,
            _flags : i64,
        ) -> Self {
            // NOTE: this is a not-currently-implemented feature
            let minimum_required = usize::MAX;

            Self {
                next,
                characters,
                // flags,
                minimum_required,
            }
        }
    }

    impl MatchRange {
        pub(crate) fn new(
            next : Option<Box<dyn Match>>,
            characters : String,
            _flags : i64,
        ) -> Self {
            // NOTE: this is a not-currently-implemented feature
            let minimum_required = usize::MAX;

            Self {
                next,
                characters,
                // flags,
                minimum_required,
            }
        }
    }

    impl MatchWild1 {
        pub(crate) fn new(next : Option<Box<dyn Match>>) -> Self {
            // NOTE: this is a not-currently-implemented feature
            let minimum_required = usize::MAX;

            Self {
                next,
                minimum_required,
            }
        }
    }

    impl MatchWildN {
        pub(crate) fn new(next : Option<Box<dyn Match>>) -> Self {
            // NOTE: this is a not-currently-implemented feature
            let minimum_required = usize::MAX;

            Self {
                next,
                minimum_required,
            }
        }
    }

    // Trait implementations

    impl Match for MatchEnd {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            slice.is_empty()
        }
    }

    impl Match for MatchLiteral {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            let slice_starts_with_literal = slice.starts_with(&self.literal)
                || match &self.literal_uppercase {
                    Some(literal_uppercase) => {
                        if slice.len() >= literal_uppercase.len() {
                            slice.to_uppercase().starts_with(literal_uppercase)
                        } else {
                            false
                        }
                    },
                    None => false,
                };

            if !slice_starts_with_literal {
                return false;
            }

            let next = self.next.as_ref().unwrap();

            next.matches(&slice[self.literal.len()..])
        }
    }

    impl Match for MatchNotRange {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            if slice.is_empty() {
                return false;
            }

            let c0 = slice.chars().next().unwrap();

            if self.characters.contains(c0) {
                return false;
            }

            let next = self.next.as_ref().unwrap();

            next.matches(&slice[c0.len_utf8()..])
        }
    }

    impl Match for MatchRange {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            if slice.is_empty() {
                return false;
            }

            let c0 = slice.chars().next().unwrap();

            if !self.characters.contains(c0) {
                return false;
            }

            let next = self.next.as_ref().unwrap();

            next.matches(&slice[c0.len_utf8()..])
        }
    }

    impl Match for MatchWild1 {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            if slice.is_empty() {
                return false;
            }

            let c0 = slice.chars().next().unwrap();

            let next = self.next.as_ref().unwrap();

            next.matches(&slice[c0.len_utf8()..])
        }
    }

    impl Match for MatchWildN {
        fn matches(
            &self,
            slice : &str,
        ) -> bool {
            let mut offset = 0;

            // TODO: consider using `#char_indices()`

            let next = self.next.as_ref().unwrap();

            for c in slice.chars() {
                if next.matches(&slice[offset..]) {
                    return true;
                } else {
                    offset += c.len_utf8();
                }
            }

            if next.matches(&slice[offset..]) {
                return true;
            }

            false
        }
    }


    #[cfg(test)]
    mod tests {
        #![allow(non_snake_case)]

        use super::{
            MatchEnd,
            MatchLiteral,
            MatchNotRange,
            MatchRange,
            MatchWild1,
            MatchWildN,
            super::traits::Match,
            super::utils::prepare_range_string,
        };


        mod TESTING_MatchEnd {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_End_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

                let matcher : &dyn Match = me.as_deref().unwrap();

                assert!(matcher.matches(""));
                assert!(!matcher.matches("a"));
            }
        }


        mod TESTING_MatchLiteral {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Literal_1() {
                let literal = "he".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

                let ml : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(me, literal, 0)));

                let matcher : &dyn Match = ml.as_deref().unwrap();

                assert!(matcher.matches("he"));
                assert!(!matcher.matches("hen"));
                assert!(!matcher.matches("he "));
            }

            #[test]
            fn TEST_Literal_2() {
                let literal2 = "ad".into();

                let literal1 = "he".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let ml2 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(me, literal2, 0)));
                let ml1 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(ml2, literal1, 0)));

                let matcher : &dyn Match = ml1.as_deref().unwrap();

                assert!(matcher.matches("head"));
                assert!(!matcher.matches("heads"));
                assert!(!matcher.matches("hea"));
            }
        }


        mod TESTING_MatchRange {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Range_1() {
                let characters = "0123456789";
                let flags = 0;
                let characters = prepare_range_string(characters, flags);

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mr : Option<Box<dyn Match>> = Some(Box::new(MatchRange::new(me, characters, 0)));

                let matcher : &dyn Match = mr.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(matcher.matches("0"));
                assert!(matcher.matches("1"));
                assert!(matcher.matches("2"));
                assert!(matcher.matches("3"));
                assert!(matcher.matches("4"));
                assert!(matcher.matches("5"));
                assert!(matcher.matches("6"));
                assert!(matcher.matches("7"));
                assert!(matcher.matches("8"));
                assert!(matcher.matches("9"));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("01"));
            }
        }


        mod TESTING_MatchNotRange {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NotRange_1() {
                let characters = "0123456789";
                let flags = 0;
                let characters = prepare_range_string(characters, flags);

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mn : Option<Box<dyn Match>> = Some(Box::new(MatchNotRange::new(me, characters, 0)));

                let matcher : &dyn Match = mn.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(!matcher.matches("0"));
                assert!(!matcher.matches("1"));
                assert!(!matcher.matches("2"));
                assert!(!matcher.matches("3"));
                assert!(!matcher.matches("4"));
                assert!(!matcher.matches("5"));
                assert!(!matcher.matches("6"));
                assert!(!matcher.matches("7"));
                assert!(!matcher.matches("8"));
                assert!(!matcher.matches("9"));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("a"));
                assert!(!matcher.matches("01"));
            }
        }


        mod TESTING_MatchWild1 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Wild_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let m1 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1::new(me)));

                let matcher : &dyn Match = m1.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(matcher.matches("0"));
                assert!(matcher.matches("1"));
                assert!(matcher.matches("2"));
                assert!(matcher.matches("3"));
                assert!(matcher.matches("4"));
                assert!(matcher.matches("5"));
                assert!(matcher.matches("6"));
                assert!(matcher.matches("7"));
                assert!(matcher.matches("8"));
                assert!(matcher.matches("9"));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("a"));
                assert!(!matcher.matches("01"));
            }

            #[test]
            fn TEST_Wild_2() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw2 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1::new(me)));
                let mw1 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1::new(mw2)));

                let matcher : &dyn Match = mw1.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(!matcher.matches("0"));
                assert!(!matcher.matches("1"));
                assert!(!matcher.matches("2"));
                assert!(!matcher.matches("3"));
                assert!(!matcher.matches("4"));
                assert!(!matcher.matches("5"));
                assert!(!matcher.matches("6"));
                assert!(!matcher.matches("7"));
                assert!(!matcher.matches("8"));
                assert!(!matcher.matches("9"));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(matcher.matches("01"));
                assert!(!matcher.matches("012"));
            }
        }


        mod TESTING_MatchWildN {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_WildN_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN::new(me)));

                let matcher : &dyn Match = mw.as_deref().unwrap();

                assert!(matcher.matches(""));
                assert!(matcher.matches("0"));
                assert!(matcher.matches("ab"));
                assert!(matcher.matches("012"));
                assert!(matcher.matches("abcd"));
                assert!(matcher.matches("01234"));
            }
        }


        mod TESTING_MISC {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Literal_WildN() {
                let literal = "ma".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN::new(me)));
                let ml : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(mw, literal, 0)));

                let matcher : &dyn Match = ml.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(!matcher.matches("m"));
                assert!(matcher.matches("ma"));
                assert!(!matcher.matches("me"));
                assert!(matcher.matches("mad"));
                assert!(matcher.matches("made"));
            }

            #[test]
            fn TEST_Literal_WildN_Literal() {
                let literal2 = "d".into();
                let literal1 = "m".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let ml2 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(me, literal2, 0)));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN::new(ml2)));
                let ml1 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(mw, literal1, 0)));

                let matcher : &dyn Match = ml1.as_deref().unwrap();

                assert!(!matcher.matches(""));
                assert!(!matcher.matches("m"));
                assert!(!matcher.matches("d"));
                assert!(!matcher.matches("ma"));
                assert!(matcher.matches("md"));
                assert!(!matcher.matches("mar"));
                assert!(matcher.matches("mad"));
                assert!(matcher.matches("mold"));
                assert!(matcher.matches("mould"));
                assert!(!matcher.matches("mouldy"));
            }
        }
    }
}

mod utils {

    use super::{
        match_structures::*,
        traits::Match,
    };

    use std::{
        fmt as std_fmt,
        mem as std_mem,
    };


    pub(crate) fn prepare_range_string(
        s : &str,
        flags : i64,
    ) -> String {
        let mut chars : Vec<char> = if 0 != (super::constants::IGNORE_CASE & flags) {
            // Two ways to do this:
            //
            // 1. If we only care about ASCII, just double letter chars; or
            // 2. Convert string to upper and lower case and form from them

            let mut ci_chars = Vec::with_capacity(s.len() * 2);

            if true {
                // 1.

                for c in s.chars() {
                    ci_chars.push(c.to_ascii_lowercase());
                    ci_chars.push(c.to_ascii_uppercase());
                }
            } else {
                // 2.

                ci_chars.append(&mut s.to_lowercase().chars().collect());
                ci_chars.append(&mut s.to_uppercase().chars().collect());
            }

            ci_chars
        } else {
            s.chars().collect()
        };

        chars.sort_unstable();

        chars.dedup();

        chars.into_iter().collect()
    }


    pub(crate) struct MatcherSequence {
        /// The head of the chain.
        matcher0 :     Option<Box<dyn Match>>,
        /// The number of matchers (excluding the end-element).
        num_matchers : usize,
    }

    // API functions
    impl MatcherSequence {
        pub(crate) fn new() -> Self {
            let matcher0 : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
            let num_matchers = 0;

            Self {
                matcher0,
                num_matchers,
            }
        }
    }

    // Mutating methods
    impl MatcherSequence {
        #![allow(non_snake_case)]


        /// T.B.C.
        ///
        /// # Returns:
        /// `total_minimum_required : usize` - the total minimum required of
        /// this and all following instances.
        #[must_use]
        pub(crate) fn prepend_Literal(
            &mut self,
            literal : String,
            flags : i64,
            following_minimum_required : usize,
        ) -> usize {
            let literal_len = literal.len();

            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(next, literal, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            literal_len + following_minimum_required
        }

        /// T.B.C.
        ///
        /// # Returns:
        /// `total_minimum_required : usize` - the total minimum required of
        /// this and all following instances.
        #[must_use]
        pub(crate) fn prepend_NotRange(
            &mut self,
            characters : String,
            flags : i64,
            following_minimum_required : usize,
        ) -> usize {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchNotRange::new(next, characters, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            1 + following_minimum_required
        }

        /// T.B.C.
        ///
        /// # Returns:
        /// `total_minimum_required : usize` - the total minimum required of
        /// this and all following instances.
        #[must_use]
        pub(crate) fn prepend_Range(
            &mut self,
            characters : String,
            flags : i64,
            following_minimum_required : usize,
        ) -> usize {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchRange::new(next, characters, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            1 + following_minimum_required
        }

        /// T.B.C.
        ///
        /// # Returns:
        /// `total_minimum_required : usize` - the total minimum required of
        /// this and all following instances.
        #[must_use]
        pub(crate) fn prepend_Wild1(
            &mut self,
            following_minimum_required : usize,
        ) -> usize {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchWild1::new(next)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            1 + following_minimum_required
        }

        /// T.B.C.
        ///
        /// # Returns:
        /// `total_minimum_required : usize` - the total minimum required of
        /// this and all following instances.
        #[must_use]
        pub(crate) fn prepend_WildN(
            &mut self,
            following_minimum_required : usize,
        ) -> usize {
            #![allow(clippy::identity_op)] // for clarity of semantics of return value

            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchWildN::new(next)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            0 + following_minimum_required
        }
    }

    // Non-mutating methods
    impl MatcherSequence {
        /// Number of matchers (excluding the mandatory `MatchEnd` instance).
        #[cfg(test)]
        pub(crate) fn len(&self) -> usize {
            self.num_matchers
        }

        pub(crate) fn matches(
            &self,
            input : &str,
        ) -> bool {
            let matcher : &dyn Match = self.matcher0.as_deref().unwrap();

            matcher.matches(input)
        }
    }

    // Trait implementations

    impl std_fmt::Debug for MatcherSequence {
        fn fmt(
            &self,
            f : &mut std_fmt::Formatter<'_>,
        ) -> std_fmt::Result {
            const TYPE_NAME : &str = "MatcherSequence";

            f.debug_struct(TYPE_NAME)
                .field("matcher0", &self.matcher0)
                .field("num_matchers", &self.num_matchers)
                .finish()
        }
    }


    #[cfg(test)]
    mod tests {
        #![allow(non_snake_case)]

        use super::{
            MatcherSequence,
            prepare_range_string,
            super::constants,
        };


        mod TEST_MatcherSequence {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_MatcherSequence_EMPTY_1() {
                let matchers = MatcherSequence::new();

                assert_eq!(0, matchers.len());

                assert!(matchers.matches(""));
                assert!(!matchers.matches(" "));
                assert!(!matchers.matches("a"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Literal_1() {
                let mut matchers = MatcherSequence::new();
                let flags = 0;
                let mut minimum_required = 0;

                minimum_required = matchers.prepend_Literal("ma".into(), flags, minimum_required);

                assert_eq!(2, minimum_required);

                assert_eq!(1, matchers.len());

                assert!(!matchers.matches(""));
                assert!(!matchers.matches(" "));
                assert!(!matchers.matches("a"));
                assert!(matchers.matches("ma"));
                assert!(!matchers.matches("mad"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Range_1() {
                let mut matchers = MatcherSequence::new();
                let flags = 0;
                let mut minimum_required = 0;

                {
                    let characters = r"abcdef";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_Range(characters, flags, minimum_required);
                }

                assert_eq!(1, minimum_required);

                assert_eq!(1, matchers.len());

                assert!(!matchers.matches(""));
                assert!(!matchers.matches(" "));
                assert!(matchers.matches("a"));
                assert!(matchers.matches("b"));
                assert!(matchers.matches("c"));
                assert!(matchers.matches("d"));
                assert!(matchers.matches("e"));
                assert!(matchers.matches("f"));
                assert!(!matchers.matches("g"));
                assert!(!matchers.matches("A"));
                assert!(!matchers.matches("B"));
                assert!(!matchers.matches("C"));
                assert!(!matchers.matches("D"));
                assert!(!matchers.matches("E"));
                assert!(!matchers.matches("F"));
                assert!(!matchers.matches("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_NotRange_1() {
                let mut matchers = MatcherSequence::new();
                let flags = 0;
                let mut minimum_required = 0;

                {
                    let characters = r"abcdef";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_NotRange(characters, flags, minimum_required);
                }

                assert_eq!(1, minimum_required);

                assert_eq!(1, matchers.len());

                assert!(!matchers.matches(""));
                assert!(matchers.matches(" "));
                assert!(!matchers.matches("a"));
                assert!(!matchers.matches("b"));
                assert!(!matchers.matches("c"));
                assert!(!matchers.matches("d"));
                assert!(!matchers.matches("e"));
                assert!(!matchers.matches("f"));
                assert!(matchers.matches("g"));
                assert!(matchers.matches("A"));
                assert!(matchers.matches("B"));
                assert!(matchers.matches("C"));
                assert!(matchers.matches("D"));
                assert!(matchers.matches("E"));
                assert!(matchers.matches("F"));
                assert!(matchers.matches("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Range_HAVING__IGNORE_CASE__1() {
                let mut matchers = MatcherSequence::new();
                let flags = constants::IGNORE_CASE;
                let mut minimum_required = 0;

                {
                    let characters = r"abcdef";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_Range(characters, flags, minimum_required);
                }

                assert_eq!(1, minimum_required);

                assert_eq!(1, matchers.len());

                assert!(!matchers.matches(""));
                assert!(!matchers.matches(" "));
                assert!(matchers.matches("a"));
                assert!(matchers.matches("b"));
                assert!(matchers.matches("c"));
                assert!(matchers.matches("d"));
                assert!(matchers.matches("e"));
                assert!(matchers.matches("f"));
                assert!(!matchers.matches("g"));
                assert!(matchers.matches("A"));
                assert!(matchers.matches("B"));
                assert!(matchers.matches("C"));
                assert!(matchers.matches("D"));
                assert!(matchers.matches("E"));
                assert!(matchers.matches("F"));
                assert!(!matchers.matches("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_NotRange_HAVING__IGNORE_CASE__1() {
                let mut matchers = MatcherSequence::new();
                let flags = constants::IGNORE_CASE;
                let mut minimum_required = 0;

                {
                    let characters = r"abcdef";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_NotRange(characters, flags, minimum_required);
                }

                assert_eq!(1, minimum_required);

                assert_eq!(1, matchers.len());

                assert!(!matchers.matches(""));
                assert!(matchers.matches(" "));
                assert!(!matchers.matches("a"));
                assert!(!matchers.matches("b"));
                assert!(!matchers.matches("c"));
                assert!(!matchers.matches("d"));
                assert!(!matchers.matches("e"));
                assert!(!matchers.matches("f"));
                assert!(matchers.matches("g"));
                assert!(!matchers.matches("A"));
                assert!(!matchers.matches("B"));
                assert!(!matchers.matches("C"));
                assert!(!matchers.matches("D"));
                assert!(!matchers.matches("E"));
                assert!(!matchers.matches("F"));
                assert!(matchers.matches("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_MULTIPLE_ELEMENTS_HAVING__IGNORE_CASE___1() {
                let mut matchers = MatcherSequence::new();
                let flags = constants::IGNORE_CASE;
                let mut minimum_required = 0;

                // match a full Windows executable Path, albeit one that may
                // not have a directory, or even a stem

                {
                    let characters = r".exe";
                    let literal = characters.into();

                    minimum_required = matchers.prepend_Literal(literal, flags, minimum_required);
                }

                assert_eq!(4, minimum_required);

                {
                    minimum_required = matchers.prepend_WildN(minimum_required);
                }

                assert_eq!(4, minimum_required);

                {
                    let characters = r"\/";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_Range(characters, flags, minimum_required);
                }

                assert_eq!(5, minimum_required);

                {
                    let characters = r":";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_Literal(characters, flags, minimum_required);
                }

                assert_eq!(6, minimum_required);

                {
                    let characters = r"abcdefghijklmnopqrstuvwxyz";
                    let characters = prepare_range_string(characters, flags);

                    minimum_required = matchers.prepend_Range(characters, flags, minimum_required);
                }

                assert_eq!(7, minimum_required);


                assert!(!matchers.matches(""));
                assert!(!matchers.matches("program.exe"));
                assert!(!matchers.matches(r"C:/"));
                assert!(!matchers.matches(r"C:\"));
                assert!(matchers.matches(r"C:/directory/program.exe"));
                assert!(matchers.matches(r"C:\directory\program.exe"));
                assert!(matchers.matches(r"C:/program.exe"));
                assert!(matchers.matches(r"C:\program.exe"));
            }
        }


        mod TEST_prepare_range_string {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_prepare_range_string_EMPTY() {
                let input = "";
                let flags = 0;
                let expected = "";
                let actual = prepare_range_string(input, flags);

                assert_eq!(expected, actual);
            }

            #[test]
            fn TEST_prepare_range_string_NUMBERS() {
                let input = "7890123456";

                {
                    let flags = 0;
                    let expected = "0123456789";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }

                {
                    let flags = constants::IGNORE_CASE;
                    let expected = "0123456789";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }
            }

            #[test]
            fn TEST_prepare_range_string_NUMBERS_WITH_DUPLICATES() {
                let input = "7890123456789";

                {
                    let flags = 0;
                    let expected = "0123456789";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }

                {
                    let flags = constants::IGNORE_CASE;
                    let expected = "0123456789";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }
            }

            #[test]
            fn TEST_prepare_range_string_CHARACTERS() {
                let input = "mnopabcd";

                {
                    let flags = 0;
                    let expected = "abcdmnop";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }

                {
                    let flags = constants::IGNORE_CASE;
                    let expected = "ABCDMNOPabcdmnop";
                    let actual = prepare_range_string(input, flags);

                    assert_eq!(expected, actual);
                }
            }
        }
    }
}


// /////////////////////////////////////////////////////////
// types

/// A specialized [`Result`] type for **shwild**.
pub type Result<T> = std_result::Result<T, Error>;


/// Type that holds a compiled match pattern, against which multiple strings
/// may be evaluated.
///
/// # Examples:
///
/// ```
/// let matcher = shwild::CompiledMatcher::from_pattern_and_flags("a[bc]c?", shwild::IGNORE_CASE).unwrap();
///
/// assert!(matcher.matches("abcd"));
/// assert!(matcher.matches("accd"));
/// assert!(matcher.matches("accx"));
/// assert!(!matcher.matches("accxyx"));
/// assert!(matcher.matches("ABCD"));
/// assert!(matcher.matches("AbCd"));
/// assert!(!matcher.matches("aacd"));
/// assert!(matcher.matches("accm"));
/// assert!(!matcher.matches("abc"));
/// ```
#[derive(Debug)]
pub struct CompiledMatcher {
    matchers : utils::MatcherSequence,
}

// API functions
impl CompiledMatcher {
    /// Creates an instance from
    pub fn from_pattern_and_flags(
        pattern : &str,
        flags : i64,
    ) -> Result<Self> {
        let mut matchers = utils::MatcherSequence::new();

        Self::parse_(&mut matchers, pattern, flags).map(|_| {
            Self {
                matchers,
            }
        })
    }
}

// Mutating methods
impl CompiledMatcher {
}

// Non-mutating methods
impl CompiledMatcher {
    /// Number of matchers (excluding the mandatory `MatchEnd` instance).
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.matchers.len()
    }

    /// Determines whether the given `input` matches the instance's compiled
    /// pattern.
    ///
    /// # Parameters:
    /// - `input` - the string to be evaluated;
    pub fn matches(
        &self,
        input : &str,
    ) -> bool {
        self.matchers.matches(input)
    }
}

#[derive(Debug)]
enum ParseState {
    None,
    InLiteral,
    InNotRange,
    InRange,
}

// Implementation
impl CompiledMatcher {
    fn parse_(
        matchers : &mut utils::MatcherSequence,
        pattern : &str,
        flags : i64,
    ) -> Result<(
        usize, // minimum_required
        usize, // num_matchers
    )> {
        let mut minimum_required = 0;
        let mut num_matchers = 0;
        let mut state = ParseState::None;
        let mut s = vec![];
        let mut escaped = false;
        let mut continuum_prior = None;
        let mut num_bytes = 0;

        for c in pattern.chars() {
            debug_assert!(continuum_prior.is_none() || matches!(state, ParseState::InNotRange | ParseState::InRange));

            match c {
                '[' => {
                    match state {
                        ParseState::None => {
                            state = ParseState::InRange;
                        },
                        ParseState::InLiteral => {
                            match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                Ok((following_mr, following_nm)) => {
                                    minimum_required = following_mr;
                                    num_matchers += following_nm;
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                            };

                            debug_assert!(!s.is_empty());

                            minimum_required =
                                matchers.prepend_Literal(String::from_iter(s.iter()), flags, minimum_required);

                            num_matchers += 1;

                            s.clear();

                            return Ok((minimum_required, num_matchers));
                        },
                        _ => {
                            s.push(c);
                        },
                    };
                },
                '^' => {
                    match state {
                        ParseState::InRange if s.is_empty() => {
                            state = ParseState::InNotRange;
                        },
                        _ => {
                            s.push(c);
                        },
                    };
                },
                ']' => {
                    match state {
                        ParseState::InNotRange | ParseState::InRange => {
                            num_bytes += 1;
                            match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                Ok((following_mr, following_nm)) => {
                                    minimum_required = following_mr;
                                    num_matchers += following_nm;
                                },
                                Err(e) => {
                                    return Err(e);
                                },
                            };

                            if let Some(_c) = continuum_prior {
                                // don't care about `_c` because that will already be pushed into `s`

                                s.push('-');
                            }

                            let characters = crate::utils::prepare_range_string(&String::from_iter(s.iter()), flags);

                            minimum_required = if matches!(state, ParseState::InRange) {
                                matchers.prepend_Range(characters, flags, minimum_required)
                            } else {
                                matchers.prepend_NotRange(characters, flags, minimum_required)
                            };

                            num_matchers += 1;

                            s.clear();

                            return Ok((minimum_required, num_matchers));
                        },
                        _ => {
                            s.push(c);
                        },
                    };
                },
                '\\' => {
                    match state {
                        ParseState::InNotRange | ParseState::InRange => {
                            s.push(c);
                        },
                        _ => {
                            if escaped {
                                s.push('\\');
                                escaped = false;
                            } else {
                                escaped = true;
                            }
                        },
                    };
                },
                '-' => {
                    if escaped {
                        s.push(c);

                        escaped = false;
                    } else {
                        match state {
                            ParseState::InNotRange | ParseState::InRange if !s.is_empty() => {
                                continuum_prior = Some(*s.last().unwrap());
                            },
                            _ => {
                                s.push(c);
                            },
                        };
                    }
                },
                '?' => {
                    if escaped {
                        s.push(c);
                        escaped = false;
                    } else {
                        match state {
                            ParseState::None => {
                                num_bytes += 1;
                                match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok((following_mr, following_nm)) => {
                                        minimum_required = following_mr;
                                        num_matchers += following_nm;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                minimum_required = matchers.prepend_Wild1(minimum_required);

                                num_matchers += 1;

                                return Ok((minimum_required, num_matchers));
                            },
                            ParseState::InLiteral => {
                                match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok((following_mr, following_nm)) => {
                                        minimum_required = following_mr;
                                        num_matchers += following_nm;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                debug_assert!(!s.is_empty());

                                minimum_required =
                                    matchers.prepend_Literal(String::from_iter(s.iter()), flags, minimum_required);

                                num_matchers += 1;

                                s.clear();

                                return Ok((minimum_required, num_matchers));
                            },
                            _ => {
                                s.push(c);
                            },
                        };
                    }
                },
                '*' => {
                    if escaped {
                        s.push(c);
                        escaped = false;
                    } else {
                        match state {
                            ParseState::None => {
                                num_bytes += 1;
                                match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok((following_mr, following_nm)) => {
                                        minimum_required = following_mr;
                                        num_matchers += following_nm;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                minimum_required = matchers.prepend_WildN(minimum_required);

                                num_matchers += 1;

                                return Ok((minimum_required, num_matchers));
                            },
                            ParseState::InLiteral => {
                                match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok((following_mr, following_nm)) => {
                                        minimum_required = following_mr;
                                        num_matchers += following_nm;
                                    },
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                debug_assert!(!s.is_empty());

                                minimum_required =
                                    matchers.prepend_Literal(String::from_iter(s.iter()), flags, minimum_required);

                                num_matchers += 1;

                                s.clear();

                                return Ok((minimum_required, num_matchers));
                            },
                            _ => {
                                s.push(c);
                            },
                        };
                    }
                },
                _ => {
                    match state {
                        ParseState::InNotRange | ParseState::InRange if !s.is_empty() => {
                            match continuum_prior {
                                Some(prior_character) => {
                                    match Self::push_continuum_(&mut s, prior_character, c, flags) {
                                        Ok(_) => (),
                                        Err(e) => {
                                            return Err(e);
                                        },
                                    };

                                    continuum_prior = None;
                                },
                                _ => {
                                    s.push(c);
                                },
                            };
                        },
                        ParseState::None => {
                            s.push(c);

                            state = ParseState::InLiteral;
                        },
                        _ => {
                            s.push(c);
                        },
                    };
                },
            };

            num_bytes += c.len_utf8();
        }

        match state {
            ParseState::None => {},
            ParseState::InLiteral => {
                let literal = String::from_iter(s.iter());

                minimum_required = matchers.prepend_Literal(literal, flags, minimum_required);
            },
            ParseState::InNotRange | ParseState::InRange => {
                return Err(Error::ParseError {
                    line :    0,
                    column :  usize::MAX,
                    message : format!("incomplete range"),
                });
            },
        };

        Ok((minimum_required, num_matchers))
    }

    fn push_character_range_(
        s : &mut Vec<char>,
        c_from : char,
        c_to : char,
    ) {
        if c_to < c_from {
            Self::push_character_range_(s, c_to, c_from);
        } else {
            // Doing it long-hand as follows, as an observed faster
            // alternative to
            //
            //   s.append((c_from..=c_to).into_iter().collect::<Vec<_>>().as_mut());
            //
            // although there doesn't seem to be much in it.

            let n = 1 + (c_to as usize - c_from as usize);

            s.reserve(s.len() + n);

            for c in c_from..=c_to {
                s.push(c);
            }
        }
    }

    fn push_continuum_(
        s : &mut Vec<char>,
        prior_character : char,
        posterior_character : char,
        flags : i64,
    ) -> Result<()> {
        {
            let _ = flags;
        }

        if !prior_character.is_ascii_alphabetic() || !posterior_character.is_ascii_alphabetic() {
            return Err(Error::ParseError {
                line : 0,
                column : usize::MAX,
                message : format!("the character range {prior_character}-{posterior_character} does not define a supported (ASCII) range continuum"),
            });
        }

        if prior_character.is_ascii_lowercase() == posterior_character.is_ascii_lowercase() {
            Self::push_character_range_(s, prior_character, posterior_character);
        } else {
            {
                let prior_lower = prior_character.to_ascii_lowercase();
                let posterior_lower = posterior_character.to_ascii_lowercase();

                Self::push_character_range_(s, prior_lower, posterior_lower);
            }

            {
                let prior_upper = prior_character.to_ascii_uppercase();
                let posterior_upper = posterior_character.to_ascii_uppercase();

                Self::push_character_range_(s, prior_upper, posterior_upper);
            }
        }

        Ok(())
    }
}

// Trait implementations

// NONE DEFINED AT THIS TIME

// /////////////////////////////////////////////////////////
// API functions

/// Determines whether the given `input` matches the given `pattern`,
/// according to the given `flags`.
///
/// # Parameters:
/// - `pattern` - the pattern to be used to evaluate `input`;
/// - `input` - the string to be evaluated;
/// - `flags` - flags that moderate the evaluation;
///
/// # Returns:
/// - `Ok(true)` - `pattern` represents a valid wildcard specification that
///   matches `input`;
/// - `Ok(false)` - `pattern` represents a valid wildcard specification that
///   does not match `input`;
/// - `Err(Error)` - `pattern` does not represent a valid wildcard
/// specification;
pub fn matches(
    pattern : &str,
    input : &str,
    flags : i64,
) -> Result<bool> {
    CompiledMatcher::from_pattern_and_flags(pattern, flags).and_then(|matcher| Ok(matcher.matches(input)))
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate as shwild;


    mod TEST_CompiledMatcher_PARSING {
        #![allow(non_snake_case)]

        use super::*;

        use crate::constants::*;


        #[test]
        fn TEST_CompiledMatcher_parse_EMPTY() {
            let pattern = "";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(0, matcher.len());

                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(0, matcher.len());

                assert!(matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_1() {
            let pattern = "abcd";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("abcd"));
                assert!(matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_2() {
            let pattern = r"ab-d";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("ab-d"));
                assert!(!matcher.matches("AB-D"));
                assert!(!matcher.matches("ab-de"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("ab-d"));
                assert!(matcher.matches("AB-D"));
                assert!(!matcher.matches("ab-de"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_3() {
            let pattern = r"ab\-d";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("ab-d"));
                assert!(!matcher.matches("AB-D"));
                assert!(!matcher.matches("ab-de"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(matcher.matches("ab-d"));
                assert!(matcher.matches("AB-D"));
                assert!(!matcher.matches("ab-de"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_1() {
            let pattern = "[abcd]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(matcher.matches("a"));
                assert!(matcher.matches("b"));
                assert!(matcher.matches("c"));
                assert!(matcher.matches("d"));
                assert!(!matcher.matches("e"));
                assert!(!matcher.matches("A"));
                assert!(!matcher.matches("B"));
                assert!(!matcher.matches("C"));
                assert!(!matcher.matches("D"));
                assert!(!matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(matcher.matches("a"));
                assert!(matcher.matches("b"));
                assert!(matcher.matches("c"));
                assert!(matcher.matches("d"));
                assert!(!matcher.matches("e"));
                assert!(matcher.matches("A"));
                assert!(matcher.matches("B"));
                assert!(matcher.matches("C"));
                assert!(matcher.matches("D"));
                assert!(!matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_NOTRANGE_1() {
            let pattern = "[^abcd]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("["));
                assert!(matcher.matches("]"));
                assert!(matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(matcher.matches("e"));
                assert!(matcher.matches("A"));
                assert!(matcher.matches("B"));
                assert!(matcher.matches("C"));
                assert!(matcher.matches("D"));
                assert!(matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("["));
                assert!(matcher.matches("]"));
                assert!(matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(matcher.matches("e"));
                assert!(!matcher.matches("A"));
                assert!(!matcher.matches("B"));
                assert!(!matcher.matches("C"));
                assert!(!matcher.matches("D"));
                assert!(matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_WILD1_1() {
            let pattern = "?";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(!matcher.matches(""));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("["));
                assert!(matcher.matches("]"));
                assert!(matcher.matches("^"));
                assert!(matcher.matches("a"));
                assert!(matcher.matches("b"));
                assert!(matcher.matches("c"));
                assert!(matcher.matches("d"));
                assert!(matcher.matches("e"));
                assert!(matcher.matches("A"));
                assert!(matcher.matches("B"));
                assert!(matcher.matches("C"));
                assert!(matcher.matches("D"));
                assert!(matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_WILDN_1() {
            let pattern = "*";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(1, matcher.len());

                assert!(matcher.matches(""));
                assert!(matcher.matches(" "));
                assert!(matcher.matches("["));
                assert!(matcher.matches("]"));
                assert!(matcher.matches("^"));
                assert!(matcher.matches("a"));
                assert!(matcher.matches("b"));
                assert!(matcher.matches("c"));
                assert!(matcher.matches("d"));
                assert!(matcher.matches("e"));
                assert!(matcher.matches("A"));
                assert!(matcher.matches("B"));
                assert!(matcher.matches("C"));
                assert!(matcher.matches("D"));
                assert!(matcher.matches("E"));
                assert!(matcher.matches("ab"));
                assert!(matcher.matches("abc"));
                assert!(matcher.matches("abcd"));
                assert!(matcher.matches("ABCD"));
                assert!(matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_THEN_WILDN_THEN_RANGE_1() {
            let pattern = "ma*[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(3, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(!matcher.matches("e"));
                assert!(!matcher.matches("A"));
                assert!(!matcher.matches("B"));
                assert!(!matcher.matches("C"));
                assert!(!matcher.matches("D"));
                assert!(!matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));


                assert!(!matcher.matches("ma"));
                assert!(matcher.matches("mad"));
                assert!(matcher.matches("made"));
                assert!(matcher.matches("madder"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_THEN_WILD1_1() {
            let pattern = "[abc]?";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(2, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(!matcher.matches("e"));

                assert!(matcher.matches("aa"));
                assert!(matcher.matches("ax"));
                assert!(matcher.matches("bb"));
                assert!(matcher.matches("by"));
                assert!(matcher.matches("cc"));
                assert!(matcher.matches("cz"));
                assert!(!matcher.matches("da"));
                assert!(!matcher.matches("ee"));

                assert!(!matcher.matches("aa "));
                assert!(!matcher.matches("ax "));
                assert!(!matcher.matches("bb "));
                assert!(!matcher.matches("by "));
                assert!(!matcher.matches("cc "));
                assert!(!matcher.matches("cz "));
                assert!(!matcher.matches("da "));
                assert!(!matcher.matches("ee "));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_THEN_LITERAL_THEN_WILDN_THEN_RANGE_1() {
            let pattern = "[mb]a*[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(4, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(!matcher.matches("e"));
                assert!(!matcher.matches("A"));
                assert!(!matcher.matches("B"));
                assert!(!matcher.matches("C"));
                assert!(!matcher.matches("D"));
                assert!(!matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("ae"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));


                assert!(!matcher.matches("ma"));
                assert!(matcher.matches("bad"));
                assert!(matcher.matches("bar"));
                assert!(matcher.matches("bald"));
                assert!(matcher.matches("bard"));
                assert!(!matcher.matches("cad"));
                assert!(!matcher.matches("car"));
                assert!(matcher.matches("mad"));
                assert!(matcher.matches("mar"));
                assert!(matcher.matches("bade"));
                assert!(!matcher.matches("lade"));
                assert!(matcher.matches("made"));
                assert!(matcher.matches("badder"));
                assert!(!matcher.matches("ladder"));
                assert!(matcher.matches("madder"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_THEN_LITERAL_THEN_WILD1_THEN_RANGE_1() {
            let pattern = "[mb]a?[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(4, matcher.len());

                assert!(!matcher.matches(""));
                assert!(!matcher.matches(" "));
                assert!(!matcher.matches("["));
                assert!(!matcher.matches("]"));
                assert!(!matcher.matches("^"));
                assert!(!matcher.matches("a"));
                assert!(!matcher.matches("b"));
                assert!(!matcher.matches("c"));
                assert!(!matcher.matches("d"));
                assert!(!matcher.matches("e"));
                assert!(!matcher.matches("A"));
                assert!(!matcher.matches("B"));
                assert!(!matcher.matches("C"));
                assert!(!matcher.matches("D"));
                assert!(!matcher.matches("E"));
                assert!(!matcher.matches("ab"));
                assert!(!matcher.matches("ae"));
                assert!(!matcher.matches("abc"));
                assert!(!matcher.matches("abcd"));
                assert!(!matcher.matches("ABCD"));
                assert!(!matcher.matches("abcde"));


                assert!(!matcher.matches("ma"));
                assert!(!matcher.matches("bad"));
                assert!(!matcher.matches("bar"));
                assert!(matcher.matches("bald"));
                assert!(matcher.matches("bard"));
                assert!(!matcher.matches("cad"));
                assert!(!matcher.matches("car"));
                assert!(!matcher.matches("mad"));
                assert!(!matcher.matches("mar"));
                assert!(matcher.matches("bade"));
                assert!(!matcher.matches("lade"));
                assert!(matcher.matches("made"));
                assert!(!matcher.matches("badder"));
                assert!(!matcher.matches("ladder"));
                assert!(!matcher.matches("madder"));
            }
        }

        // These all taken from https://github.com/synesissoftware/shwild/blob/master/test/unit/test.unit.shwild.pattern/test.unit.shwild.pattern.cpp
        #[test]
        fn TEST_CompiledMatcher_parse_CXX_TEST_1() {
            /* Matching literal strings. */
            {
                let pattern = "abcd";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(1, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(!matcher.matches("ABCD"));
                }

                {
                    let flags = IGNORE_CASE;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(1, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(matcher.matches("ABCD"));
                }
            }

            /* Using wildcards. */
            {
                let pattern = "a*c?";
                let flags = 0;
                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                assert_eq!(4, matcher.len());

                assert!(matcher.matches("abcd"));
                assert!(matcher.matches("a*c?"));
                assert!(matcher.matches("abbbbbbbbcd"));
                assert!(matcher.matches("acd"));
                assert!(!matcher.matches("abdc"));
                assert!(matcher.matches("abc?"));
            }

            /* Using escaped characters. */
            {
                let pattern = "a\\*c\\?";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(1, matcher.len());

                    assert!(!matcher.matches("abcd"));
                    assert!(matcher.matches("a*c?"));
                    assert!(!matcher.matches("abbbbbbbbcd"));
                    assert!(!matcher.matches("acd"));
                    assert!(!matcher.matches("abdc"));
                    assert!(!matcher.matches("abc?"));
                }

                /*
                {
                    let flags = SUPPRESS_BACKSLASH_ESCAPE;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(1, matcher.len());

                    assert!(!matcher.matches("abcd"));
                    assert!(matcher.matches("a\\*c\\?"));
                }
                 */
            }

            /* Matching ranges. */
            {
                let pattern = "a[bc]c[defghijklm]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("accm"));
                    assert!(!matcher.matches("abcn"));
                    assert!(!matcher.matches("a[bc]c[defghijklm]"));
                }

                /*
                {
                    let flags = SUPPRESS_RANGE_SUPPORT;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(1, matcher.len());

                    assert!(!matcher.matches("abcd"));
                    assert!(!matcher.matches("aacd"));
                    assert!(!matcher.matches("accm"));
                    assert!(!matcher.matches("abcn"));
                    assert!(matcher.matches("a[bc]c[defghijklm]"));
                }
                 */
            }

            /* Matching ranges with continuum. */
            {
                let pattern = "a[b-c]c[d-m]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(matcher.matches("abce"));
                    assert!(matcher.matches("abcf"));
                    assert!(matcher.matches("abcg"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("accm"));
                    assert!(!matcher.matches("abcn"));

                    assert!(!matcher.matches("a-cm"));
                    assert!(!matcher.matches("acc-"));
                }

                /*
                {
                    const shwild::Pattern   pattern2("a[b-c]c[d-m]", SHWILD_F_SUPPRESS_RANGE_CONTINUUM_SUPPORT);

                    BDUT_ASSERT_TRUE(matcher.matches("abcd"));
                    BDUT_ASSERT_TRUE(matcher.matches("a-cd"));
                    BDUT_ASSERT_TRUE(matcher.matches("accd"));
                    BDUT_ASSERT_FALSE(matcher.matches("aacd"));
                    BDUT_ASSERT_TRUE(matcher.matches("accm"));
                    BDUT_ASSERT_FALSE(matcher.matches("accl"));
                    BDUT_ASSERT_FALSE(matcher.matches("abcn"));
                }
                 */
            }

            /* Matching ranges with high-low continuum. */
            {
                let pattern = "a[c-b]c[m-d]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(matcher.matches("abce"));
                    assert!(matcher.matches("abcf"));
                    assert!(matcher.matches("abcg"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("accm"));
                    assert!(!matcher.matches("abcn"));

                    assert!(!matcher.matches("a-cm"));
                    assert!(!matcher.matches("acc-"));
                }

                /*
                {

                    const shwild::Pattern   pattern2("a[b-c]c[d-m]", SHWILD_F_SUPPRESS_RANGE_CONTINUUM_SUPPORT);

                    BDUT_ASSERT_TRUE(matcher.matches("abcd"));
                    BDUT_ASSERT_TRUE(matcher.matches("a-cd"));
                    BDUT_ASSERT_TRUE(matcher.matches("accd"));
                    BDUT_ASSERT_FALSE(matcher.matches("aacd"));
                    BDUT_ASSERT_TRUE(matcher.matches("accm"));
                    BDUT_ASSERT_FALSE(matcher.matches("accl"));
                    BDUT_ASSERT_FALSE(matcher.matches("abcn"));
                }
                 */
            }

            /* Matching ranges with cross-case continuum. */
            {
                let pattern = "a[b-C]c[m-D]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("aCcJ"));
                    assert!(!matcher.matches("abcn"));

                    assert!(matcher.matches("abcd"));
                    assert!(matcher.matches("abce"));
                    assert!(matcher.matches("abcf"));
                    assert!(matcher.matches("abcg"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("accm"));
                    assert!(!matcher.matches("abcn"));

                    assert!(!matcher.matches("a-cm"));
                    assert!(!matcher.matches("acc-"));
                }
            }

            /* Matching ranges with wildcards as literals. */
            {
                let pattern = "a[*]c[?]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(!matcher.matches("abcd"));
                    assert!(matcher.matches("a*c?"));
                    assert!(!matcher.matches("abbbbbbbbcd"));
                    assert!(!matcher.matches("acd"));
                    assert!(!matcher.matches("abdc"));
                    assert!(!matcher.matches("abc?"));
                }
            }

            /* Matching ranges with continuum and leading/trailing hyphens. */
            {
                let pattern = "a[-a-c]c[d-]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(matcher.matches("abcd"));
                    assert!(matcher.matches("aacd"));
                    assert!(matcher.matches("acc-"));
                    assert!(matcher.matches("a-c-"));
                    assert!(!matcher.matches("abce"));
                }
            }

            /* Matching ranges with inverse continuum. */
            {
                let pattern = "a[b-c]c[^d-m]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                    assert_eq!(4, matcher.len());

                    assert!(!matcher.matches("abcd"));
                    assert!(!matcher.matches("aacd"));
                    assert!(matcher.matches("abcc"));
                    assert!(!matcher.matches("accm"));
                    assert!(matcher.matches("abcn"));
                }
            }
        }
    }


    mod TEST_API {
        #![allow(non_snake_case)]

        use super::*;


        #[test]
        fn TEST_matches_EMPTY_PATTERN_1() {
            assert_eq!(Ok(true), shwild::matches("", "", 0));
            assert_eq!(Ok(false), shwild::matches("", " ", 0));
            assert_eq!(Ok(false), shwild::matches("", "_", 0));
            assert_eq!(Ok(false), shwild::matches("", "a", 0));
            assert_eq!(Ok(false), shwild::matches("", "abc", 0));
        }

        #[test]
        fn TEST_matches_INVALID_PATTERN_1() {
            match shwild::matches("[a-9]", "", 0) {
                Ok(_) => {
                    panic!("unexpected success");
                },
                Err(e) => {
                    assert_eq!(
                        "the character range a-9 does not define a supported (ASCII) range continuum",
                        format!("{e:?}")
                    );
                },
            };
        }

        #[test]
        fn TEST_matches_INVALID_PATTERN_INCOMPLETE_RANGE_1() {
            match shwild::matches("[a-z", "", 0) {
                Ok(_) => {
                    panic!("unexpected success");
                },
                Err(e) => {
                    assert_eq!("incomplete range", format!("{e:?}"));
                },
            };
        }

        #[test]
        fn TEST_matches_WINDOWS_PROGRAM_PATH_PATTERN_1() {
            let pattern = r"[A-Z]\?*\?*.[ce][ox][em]";

            assert_eq!(Ok(false), shwild::matches(pattern, "", 0));

            assert_eq!(Ok(true), shwild::matches(pattern, r"C:\directory\file.exe", 0));
            assert_eq!(Ok(true), shwild::matches(pattern, r"X:\dir\filestem.exe", 0));
            assert_eq!(Ok(true), shwild::matches(pattern, r"D:\dir\sub-dir\filestem.exe", 0));
        }
    }
}
