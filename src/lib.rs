// Definition of the shwild Rust package

use std::{
    error as std_error,
    fmt as std_fmt,
    result as std_result,
};


#[derive(PartialEq)]
pub enum Error {
    Ok,
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

    pub(crate) const IGNORE_CASE : i64 = 0x0200;
}


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
        /// - `false` -

        fn try_match(
            &self,
            slice : &str,
        ) -> bool;

        /// Obtains the minimum required remaining length for the chain
        /// beginning with the called instance, which may be called to
        /// optimise implementations of certain `Match` instances.
        fn minimum_required(&self) -> usize;

        /// T.B.C.
        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>>;
    }
}

mod match_structures {

    use super::traits::Match;

    use std::mem as std_mem;


    #[derive(Debug)]
    pub(crate) struct MatchEnd {}

    #[derive(Debug)]
    pub(crate) struct MatchLiteral {
        next :              Option<Box<dyn Match>>,
        literal :           String,
        literal_uppercase : Option<String>,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchNotRange {
        next :       Option<Box<dyn Match>>,
        characters : String,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchRange {
        next :       Option<Box<dyn Match>>,
        characters : String,
        // flags : i64,
    }

    #[derive(Debug)]
    pub(crate) struct MatchWild1 {
        pub(crate) next : Option<Box<dyn Match>>,
    }

    #[derive(Debug)]
    pub(crate) struct MatchWildN {
        pub(crate) next : Option<Box<dyn Match>>,
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

            Self {
                next,
                literal,
                literal_uppercase,
                // flags,
            }
        }
    }

    impl MatchNotRange {
        pub(crate) fn new(
            next : Option<Box<dyn Match>>,
            characters : &str,
            flags : i64,
        ) -> Self {
            let characters = super::utils::prepare_range_string(characters, flags);

            Self {
                next,
                characters,
                // flags,
            }
        }
    }

    impl MatchRange {
        pub(crate) fn new(
            next : Option<Box<dyn Match>>,
            characters : &str,
            flags : i64,
        ) -> Self {
            let characters = super::utils::prepare_range_string(characters, flags);

            Self {
                next,
                characters,
                // flags,
            }
        }
    }

    // Trait implementations

    impl Match for MatchEnd {
        fn try_match(
            &self,
            slice : &str,
        ) -> bool {
            slice.is_empty()
        }

        fn minimum_required(&self) -> usize {
            0
        }

        fn set_next(
            &mut self,
            _next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            panic!("should never be called");
        }
    }

    impl Match for MatchLiteral {
        fn try_match(
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

            next.try_match(&slice[self.literal.len()..])
        }

        fn minimum_required(&self) -> usize {
            let next = self.next.as_ref().unwrap();

            self.literal.len() + next.minimum_required()
        }

        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            let mut next = next;

            std_mem::swap(&mut self.next, &mut next);

            next
        }
    }

    impl Match for MatchNotRange {
        fn try_match(
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

            next.try_match(&slice[c0.len_utf8()..])
        }

        fn minimum_required(&self) -> usize {
            let next = self.next.as_ref().unwrap();

            1 + next.minimum_required()
        }

        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            let mut next = next;

            std_mem::swap(&mut self.next, &mut next);

            next
        }
    }

    impl Match for MatchRange {
        fn try_match(
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

            next.try_match(&slice[c0.len_utf8()..])
        }

        fn minimum_required(&self) -> usize {
            let next = self.next.as_ref().unwrap();

            1 + next.minimum_required()
        }

        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            let mut next = next;

            std_mem::swap(&mut self.next, &mut next);

            next
        }
    }

    impl Match for MatchWild1 {
        fn try_match(
            &self,
            slice : &str,
        ) -> bool {
            if slice.is_empty() {
                return false;
            }

            let c0 = slice.chars().next().unwrap();

            let next = self.next.as_ref().unwrap();

            next.try_match(&slice[c0.len_utf8()..])
        }

        fn minimum_required(&self) -> usize {
            let next = self.next.as_ref().unwrap();

            1 + next.minimum_required()
        }

        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            let mut next = next;

            std_mem::swap(&mut self.next, &mut next);

            next
        }
    }

    impl Match for MatchWildN {
        fn try_match(
            &self,
            slice : &str,
        ) -> bool {
            let mut offset = 0;

            // TODO: consider using `#char_indices()`

            let next = self.next.as_ref().unwrap();

            for c in slice.chars() {
                if next.try_match(&slice[offset..]) {
                    return true;
                } else {
                    offset += c.len_utf8();
                }
            }

            if next.try_match(&slice[offset..]) {
                return true;
            }

            false
        }

        fn minimum_required(&self) -> usize {
            #![allow(clippy::identity_op)]

            let next = self.next.as_ref().unwrap();

            0 + next.minimum_required()
        }

        fn set_next(
            &mut self,
            next : Option<Box<dyn Match>>,
        ) -> Option<Box<dyn Match>> {
            let mut next = next;

            std_mem::swap(&mut self.next, &mut next);

            next
        }
    }


    #[cfg(test)]
    mod tests {
        #![allow(non_snake_case)]

        use super::*;


        mod TESTING_MatchEnd {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_End_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

                let matcher : &dyn Match = me.as_deref().unwrap();

                assert!(matcher.try_match(""));
                assert!(!matcher.try_match("a"));
                assert_eq!(0, matcher.minimum_required());
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

                assert_eq!(2, matcher.minimum_required());
                assert!(matcher.try_match("he"));
                assert!(!matcher.try_match("hen"));
                assert!(!matcher.try_match("he "));
            }

            #[test]
            fn TEST_Literal_2() {
                let literal2 = "ad".into();

                let literal1 = "he".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let ml2 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(me, literal2, 0)));
                let ml1 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(ml2, literal1, 0)));

                let matcher : &dyn Match = ml1.as_deref().unwrap();

                assert_eq!(4, matcher.minimum_required());
                assert!(matcher.try_match("head"));
                assert!(!matcher.try_match("heads"));
                assert!(!matcher.try_match("hea"));
            }
        }


        mod TESTING_MatchRange {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Range_1() {
                let characters = "0123456789";

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mr : Option<Box<dyn Match>> = Some(Box::new(MatchRange::new(me, characters, 0)));

                let matcher : &dyn Match = mr.as_deref().unwrap();

                assert_eq!(1, matcher.minimum_required());
                assert!(!matcher.try_match(""));
                assert!(matcher.try_match("0"));
                assert!(matcher.try_match("1"));
                assert!(matcher.try_match("2"));
                assert!(matcher.try_match("3"));
                assert!(matcher.try_match("4"));
                assert!(matcher.try_match("5"));
                assert!(matcher.try_match("6"));
                assert!(matcher.try_match("7"));
                assert!(matcher.try_match("8"));
                assert!(matcher.try_match("9"));
                assert!(!matcher.try_match(" "));
                assert!(!matcher.try_match("a"));
                assert!(!matcher.try_match("01"));
            }
        }


        mod TESTING_MatchNotRange {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_NotRange_1() {
                let characters = "0123456789";

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mn : Option<Box<dyn Match>> = Some(Box::new(MatchNotRange::new(me, characters, 0)));

                let matcher : &dyn Match = mn.as_deref().unwrap();

                assert_eq!(1, matcher.minimum_required());

                assert!(!matcher.try_match(""));
                assert!(!matcher.try_match("0"));
                assert!(!matcher.try_match("1"));
                assert!(!matcher.try_match("2"));
                assert!(!matcher.try_match("3"));
                assert!(!matcher.try_match("4"));
                assert!(!matcher.try_match("5"));
                assert!(!matcher.try_match("6"));
                assert!(!matcher.try_match("7"));
                assert!(!matcher.try_match("8"));
                assert!(!matcher.try_match("9"));
                assert!(matcher.try_match(" "));
                assert!(matcher.try_match("a"));
                assert!(!matcher.try_match("01"));
            }
        }


        mod TESTING_MatchWild1 {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Wild_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let m1 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1 {
                    next : me
                }));

                let matcher : &dyn Match = m1.as_deref().unwrap();

                assert_eq!(1, matcher.minimum_required());

                assert!(!matcher.try_match(""));
                assert!(matcher.try_match("0"));
                assert!(matcher.try_match("1"));
                assert!(matcher.try_match("2"));
                assert!(matcher.try_match("3"));
                assert!(matcher.try_match("4"));
                assert!(matcher.try_match("5"));
                assert!(matcher.try_match("6"));
                assert!(matcher.try_match("7"));
                assert!(matcher.try_match("8"));
                assert!(matcher.try_match("9"));
                assert!(matcher.try_match(" "));
                assert!(matcher.try_match("a"));
                assert!(!matcher.try_match("01"));
            }

            #[test]
            fn TEST_Wild_2() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw2 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1 {
                    next : me
                }));
                let mw1 : Option<Box<dyn Match>> = Some(Box::new(MatchWild1 {
                    next : mw2
                }));

                let matcher : &dyn Match = mw1.as_deref().unwrap();

                assert_eq!(2, matcher.minimum_required());

                assert!(!matcher.try_match(""));
                assert!(!matcher.try_match("0"));
                assert!(!matcher.try_match("1"));
                assert!(!matcher.try_match("2"));
                assert!(!matcher.try_match("3"));
                assert!(!matcher.try_match("4"));
                assert!(!matcher.try_match("5"));
                assert!(!matcher.try_match("6"));
                assert!(!matcher.try_match("7"));
                assert!(!matcher.try_match("8"));
                assert!(!matcher.try_match("9"));
                assert!(!matcher.try_match(" "));
                assert!(!matcher.try_match("a"));
                assert!(matcher.try_match("01"));
                assert!(!matcher.try_match("012"));
            }
        }


        mod TESTING_MatchWildN {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_WildN_1() {
                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN {
                    next : me
                }));

                let matcher : &dyn Match = mw.as_deref().unwrap();

                assert_eq!(0, matcher.minimum_required());

                assert!(matcher.try_match(""));
                assert!(matcher.try_match("0"));
                assert!(matcher.try_match("ab"));
                assert!(matcher.try_match("012"));
                assert!(matcher.try_match("abcd"));
                assert!(matcher.try_match("01234"));
            }
        }


        mod TESTING_MISC {
            #![allow(non_snake_case)]

            use super::*;


            #[test]
            fn TEST_Literal_WildN() {
                let literal = "ma".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN {
                    next : me
                }));
                let ml : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(mw, literal, 0)));

                let matcher : &dyn Match = ml.as_deref().unwrap();

                assert_eq!(2, matcher.minimum_required());

                assert!(!matcher.try_match(""));
                assert!(!matcher.try_match("m"));
                assert!(matcher.try_match("ma"));
                assert!(!matcher.try_match("me"));
                assert!(matcher.try_match("mad"));
                assert!(matcher.try_match("made"));
            }

            #[test]
            fn TEST_Literal_WildN_Literal() {
                let literal2 = "d".into();
                let literal1 = "m".into();

                let me : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));
                let ml2 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(me, literal2, 0)));
                let mw : Option<Box<dyn Match>> = Some(Box::new(MatchWildN {
                    next : ml2
                }));
                let ml1 : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(mw, literal1, 0)));

                let matcher : &dyn Match = ml1.as_deref().unwrap();

                assert_eq!(2, matcher.minimum_required());

                assert!(!matcher.try_match(""));
                assert!(!matcher.try_match("m"));
                assert!(!matcher.try_match("d"));
                assert!(!matcher.try_match("ma"));
                assert!(matcher.try_match("md"));
                assert!(!matcher.try_match("mar"));
                assert!(matcher.try_match("mad"));
                assert!(matcher.try_match("mold"));
                assert!(matcher.try_match("mould"));
                assert!(!matcher.try_match("mouldy"));
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

            // for c in s.chars() {

            //     if c.is_alphabetic() {

            //         ci_chars.append(other)
            //         ci_chars.push(c.to_lowercase());
            //         ci_chars.push(c.to_uppercase());
            //     } else {

            //         ci_chars.push(c);
            //     }
            // }

            ci_chars.append(&mut s.to_lowercase().chars().collect());
            ci_chars.append(&mut s.to_uppercase().chars().collect());

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


        pub(crate) fn prepend_Literal(
            &mut self,
            literal : String,
            flags : i64,
        ) -> &mut Self {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchLiteral::new(next, literal, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            self
        }

        pub(crate) fn prepend_NotRange(
            &mut self,
            characters : &str,
            flags : i64,
        ) -> &mut Self {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchNotRange::new(next, characters, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            self
        }

        pub(crate) fn prepend_Range(
            &mut self,
            characters : &str,
            flags : i64,
        ) -> &mut Self {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchRange::new(next, characters, flags)));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            self
        }

        pub(crate) fn prepend_Wild1(&mut self) -> &mut Self {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchWild1 {
                next,
            }));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            self
        }

        pub(crate) fn prepend_WildN(&mut self) -> &mut Self {
            let mut next : Option<Box<dyn Match>> = Some(Box::new(MatchEnd {}));

            std_mem::swap(&mut self.matcher0, &mut next);

            // NOW: `next` is the head of the list, and `self.matcher0` is `MatchEnd`

            let mut matcher : Option<Box<dyn Match>> = Some(Box::new(MatchWildN {
                next,
            }));

            std_mem::swap(&mut self.matcher0, &mut matcher);

            self.num_matchers += 1;

            self
        }
    }

    // Non-mutating methods
    impl MatcherSequence {
        #[cfg(test)]
        pub(crate) fn len(&self) -> usize {
            self.num_matchers
        }

        pub(crate) fn try_match(
            &self,
            input : &str,
        ) -> bool {
            let matcher : &dyn Match = self.matcher0.as_deref().unwrap();

            matcher.try_match(input)
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

        use super::*;


        mod TEST_MatcherSequence {
            #![allow(non_snake_case)]

            use super::*;

            use crate::constants::IGNORE_CASE;


            #[test]
            fn TEST_MatcherSequence_EMPTY_1() {
                let matchers = MatcherSequence::new();

                assert_eq!(0, matchers.len());

                assert!(matchers.try_match(""));
                assert!(!matchers.try_match(" "));
                assert!(!matchers.try_match("a"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Literal_1() {
                let mut matchers = MatcherSequence::new();

                matchers.prepend_Literal("ma".into(), 0);

                assert_eq!(1, matchers.len());

                assert!(!matchers.try_match(""));
                assert!(!matchers.try_match(" "));
                assert!(!matchers.try_match("a"));
                assert!(matchers.try_match("ma"));
                assert!(!matchers.try_match("mad"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Range_1() {
                let mut matchers = MatcherSequence::new();

                matchers.prepend_Range("abcdef".into(), 0);

                assert_eq!(1, matchers.len());

                assert!(!matchers.try_match(""));
                assert!(!matchers.try_match(" "));
                assert!(matchers.try_match("a"));
                assert!(matchers.try_match("b"));
                assert!(matchers.try_match("c"));
                assert!(matchers.try_match("d"));
                assert!(matchers.try_match("e"));
                assert!(matchers.try_match("f"));
                assert!(!matchers.try_match("g"));
                assert!(!matchers.try_match("A"));
                assert!(!matchers.try_match("B"));
                assert!(!matchers.try_match("C"));
                assert!(!matchers.try_match("D"));
                assert!(!matchers.try_match("E"));
                assert!(!matchers.try_match("F"));
                assert!(!matchers.try_match("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_NotRange_1() {
                let mut matchers = MatcherSequence::new();

                matchers.prepend_NotRange("abcdef".into(), 0);

                assert_eq!(1, matchers.len());

                assert!(!matchers.try_match(""));
                assert!(matchers.try_match(" "));
                assert!(!matchers.try_match("a"));
                assert!(!matchers.try_match("b"));
                assert!(!matchers.try_match("c"));
                assert!(!matchers.try_match("d"));
                assert!(!matchers.try_match("e"));
                assert!(!matchers.try_match("f"));
                assert!(matchers.try_match("g"));
                assert!(matchers.try_match("A"));
                assert!(matchers.try_match("B"));
                assert!(matchers.try_match("C"));
                assert!(matchers.try_match("D"));
                assert!(matchers.try_match("E"));
                assert!(matchers.try_match("F"));
                assert!(matchers.try_match("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_Range_HAVING__IGNORE_CASE__1() {
                let mut matchers = MatcherSequence::new();

                matchers.prepend_Range("abcdef".into(), IGNORE_CASE);

                assert_eq!(1, matchers.len());

                assert!(!matchers.try_match(""));
                assert!(!matchers.try_match(" "));
                assert!(matchers.try_match("a"));
                assert!(matchers.try_match("b"));
                assert!(matchers.try_match("c"));
                assert!(matchers.try_match("d"));
                assert!(matchers.try_match("e"));
                assert!(matchers.try_match("f"));
                assert!(!matchers.try_match("g"));
                assert!(matchers.try_match("A"));
                assert!(matchers.try_match("B"));
                assert!(matchers.try_match("C"));
                assert!(matchers.try_match("D"));
                assert!(matchers.try_match("E"));
                assert!(matchers.try_match("F"));
                assert!(!matchers.try_match("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_NotRange_HAVING__IGNORE_CASE__1() {
                let mut matchers = MatcherSequence::new();

                matchers.prepend_NotRange("abcdef".into(), IGNORE_CASE);

                assert_eq!(1, matchers.len());

                assert!(!matchers.try_match(""));
                assert!(matchers.try_match(" "));
                assert!(!matchers.try_match("a"));
                assert!(!matchers.try_match("b"));
                assert!(!matchers.try_match("c"));
                assert!(!matchers.try_match("d"));
                assert!(!matchers.try_match("e"));
                assert!(!matchers.try_match("f"));
                assert!(matchers.try_match("g"));
                assert!(!matchers.try_match("A"));
                assert!(!matchers.try_match("B"));
                assert!(!matchers.try_match("C"));
                assert!(!matchers.try_match("D"));
                assert!(!matchers.try_match("E"));
                assert!(!matchers.try_match("F"));
                assert!(matchers.try_match("G"));
            }

            #[test]
            fn TEST_MatcherSequence_WITH_MULTIPLE_ELEMENTS_HAVING__IGNORE_CASE___1() {
                let mut matchers = MatcherSequence::new();
                let flags = IGNORE_CASE;

                // match a full Windows executable Path, albeit one that may
                // not have a directory, or even a stem

                matchers.prepend_Literal(r".exe".into(), flags);
                matchers.prepend_WildN();
                matchers.prepend_Range(r"\/", flags);
                matchers.prepend_Literal(r":".into(), flags);
                matchers.prepend_Range(r"abcdefghijklmnopqrstuvwxyz", flags);

                assert!(!matchers.try_match(""));
                assert!(!matchers.try_match("program.exe"));
                assert!(!matchers.try_match("C:/"));
                assert!(matchers.try_match("C:/directory/program.exe"));
                assert!(matchers.try_match("C:/program.exe"));
            }
        }
    }
}


/// A specialized [`Result`] type for **shwild**.
pub type Result<T> = std_result::Result<T, Error>;


/// T.B.C.
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
    pub fn matches(
        &self,
        input : &str,
    ) -> Result<bool> {
        Ok(self.matchers.try_match(input))
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
    ) -> Result<
        usize, // num_matchers
    > {
        let mut num_matchers = 0;
        let mut state = ParseState::None;
        let mut s = vec![];
        let mut escaped = false;
        let mut continuum_prior = None;
        let mut num_bytes = 0;

        for c in pattern.chars() {
            debug_assert!(continuum_prior.is_none() || matches!(state, ParseState::InRange | ParseState::InNotRange));

            match c {
                '[' => {
                    match state {
                        ParseState::None => {
                            state = ParseState::InRange;
                        },
                        ParseState::InLiteral => {
                            num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                Ok(num_matchers) => num_matchers,
                                Err(e) => {
                                    return Err(e);
                                },
                            };

                            if !s.is_empty() {
                                matchers.prepend_Literal(String::from_iter(s.iter()), flags);

                                s.clear();
                            }

                            return Ok(num_matchers + 1);
                        },
                        _ => {
                            s.push(c);
                        },
                    }
                },
                '^' => {
                    match state {
                        ParseState::InRange if s.is_empty() => {
                            state = ParseState::InNotRange;
                        },
                        _ => {
                            s.push(c);
                        },
                    }
                },
                ']' => {
                    match state {
                        ParseState::InRange => {
                            num_bytes += 1;
                            num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                Ok(num_matchers) => num_matchers,
                                Err(e) => {
                                    return Err(e);
                                },
                            };

                            if let Some(_c) = continuum_prior {
                                // don't care about `_c` because that will already be pushed into `s`

                                s.push('-');
                            }

                            matchers.prepend_Range(&String::from_iter(s.iter()), flags);

                            s.clear();

                            return Ok(num_matchers + 1);
                        },
                        ParseState::InNotRange => {
                            num_bytes += 1;
                            num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                Ok(num_matchers) => num_matchers,
                                Err(e) => {
                                    return Err(e);
                                },
                            };

                            if let Some(_c) = continuum_prior {
                                // don't care about `_c` because that will already be pushed into `s`

                                s.push('-');
                            }

                            matchers.prepend_NotRange(&String::from_iter(s.iter()), flags);

                            s.clear();

                            return Ok(num_matchers + 1);
                        },
                        _ => {
                            s.push(c);
                        },
                    }
                },
                '\\' => {
                    match state {
                        ParseState::InRange | ParseState::InNotRange => {
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
                    }
                },
                '-' => {
                    if escaped {
                        s.push(c);

                        escaped = false;
                    } else {
                        match state {
                            ParseState::InRange | ParseState::InNotRange if !s.is_empty() => {
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
                                num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok(num_matchers) => num_matchers,
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };


                                matchers.prepend_Wild1();

                                return Ok(num_matchers + 1);
                            },
                            ParseState::InLiteral => {
                                num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok(num_matchers) => num_matchers,
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                if !s.is_empty() {
                                    matchers.prepend_Literal(String::from_iter(s.iter()), flags);

                                    s.clear();
                                }

                                return Ok(num_matchers + 1);
                            },
                            _ => {
                                s.push(c);
                            },
                        }
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
                                num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok(num_matchers) => num_matchers,
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };


                                matchers.prepend_WildN();

                                return Ok(num_matchers + 1);
                            },
                            ParseState::InLiteral => {
                                num_matchers += match Self::parse_(matchers, &pattern[num_bytes..], flags) {
                                    Ok(num_matchers) => num_matchers,
                                    Err(e) => {
                                        return Err(e);
                                    },
                                };

                                if !s.is_empty() {
                                    matchers.prepend_Literal(String::from_iter(s.iter()), flags);

                                    s.clear();
                                }

                                return Ok(num_matchers + 1);
                            },
                            _ => {
                                s.push(c);
                            },
                        }
                    }
                },
                _ => {
                    match state {
                        ParseState::InRange | ParseState::InNotRange if !s.is_empty() => {
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
                            }
                        },
                        ParseState::None => {
                            s.push(c);

                            state = ParseState::InLiteral;
                        },
                        _ => {
                            s.push(c);
                        },
                    }
                },
            }

            num_bytes += c.len_utf8();
        }

        match state {
            ParseState::None => {},
            ParseState::InLiteral => {
                matchers.prepend_Literal(String::from_iter(s.iter()), flags);
            },
            ParseState::InNotRange => {
                matchers.prepend_NotRange(&String::from_iter(s.iter()), flags);
            },
            ParseState::InRange => {
                matchers.prepend_Range(&String::from_iter(s.iter()), flags);
            },
        }

        Ok(num_matchers)
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
            let r = if prior_character > posterior_character {
                posterior_character..=prior_character
            } else {
                prior_character..=posterior_character
            };

            s.append(r.into_iter().collect::<Vec<_>>().as_mut());
        } else {
            {
                let prior_lower = prior_character.to_ascii_lowercase();
                let posterior_lower = posterior_character.to_ascii_lowercase();
                let r_lower = if prior_lower > posterior_lower {
                    posterior_lower..=prior_lower
                } else {
                    prior_lower..=posterior_lower
                };

                s.append(r_lower.into_iter().collect::<Vec<_>>().as_mut());
            }

            {
                let prior_upper = prior_character.to_ascii_uppercase();
                let posterior_upper = posterior_character.to_ascii_uppercase();
                let r_upper = if prior_upper > posterior_upper {
                    posterior_upper..=prior_upper
                } else {
                    prior_upper..=posterior_upper
                };

                s.append(r_upper.into_iter().collect::<Vec<_>>().as_mut());
            }
        }

        Ok(())
    }
}

// Trait implementations


// API functions

/// T.B.C.
pub fn matches(
    pattern : &str,
    input : &str,
    flags : i64,
) -> Result<bool> {
    match CompiledMatcher::from_pattern_and_flags(pattern, flags) {
        Ok(matcher) => matcher.matches(input),
        Err(e) => Err(e),
    }
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

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(true), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_1() {
            let pattern = "abcd";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("abcd"));
                assert_eq!(Ok(true), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_2() {
            let pattern = r"ab-d";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("ab-d"));
                assert_eq!(Ok(false), matcher.matches("AB-D"));
                assert_eq!(Ok(false), matcher.matches("ab-de"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("ab-d"));
                assert_eq!(Ok(true), matcher.matches("AB-D"));
                assert_eq!(Ok(false), matcher.matches("ab-de"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_3() {
            let pattern = r"ab\-d";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("ab-d"));
                assert_eq!(Ok(false), matcher.matches("AB-D"));
                assert_eq!(Ok(false), matcher.matches("ab-de"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("ab-d"));
                assert_eq!(Ok(true), matcher.matches("AB-D"));
                assert_eq!(Ok(false), matcher.matches("ab-de"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_1() {
            let pattern = "[abcd]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("["));
                assert_eq!(Ok(false), matcher.matches("]"));
                assert_eq!(Ok(false), matcher.matches("^"));
                assert_eq!(Ok(true), matcher.matches("a"));
                assert_eq!(Ok(true), matcher.matches("b"));
                assert_eq!(Ok(true), matcher.matches("c"));
                assert_eq!(Ok(true), matcher.matches("d"));
                assert_eq!(Ok(false), matcher.matches("e"));
                assert_eq!(Ok(false), matcher.matches("A"));
                assert_eq!(Ok(false), matcher.matches("B"));
                assert_eq!(Ok(false), matcher.matches("C"));
                assert_eq!(Ok(false), matcher.matches("D"));
                assert_eq!(Ok(false), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("["));
                assert_eq!(Ok(false), matcher.matches("]"));
                assert_eq!(Ok(false), matcher.matches("^"));
                assert_eq!(Ok(true), matcher.matches("a"));
                assert_eq!(Ok(true), matcher.matches("b"));
                assert_eq!(Ok(true), matcher.matches("c"));
                assert_eq!(Ok(true), matcher.matches("d"));
                assert_eq!(Ok(false), matcher.matches("e"));
                assert_eq!(Ok(true), matcher.matches("A"));
                assert_eq!(Ok(true), matcher.matches("B"));
                assert_eq!(Ok(true), matcher.matches("C"));
                assert_eq!(Ok(true), matcher.matches("D"));
                assert_eq!(Ok(false), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_NOTRANGE_1() {
            let pattern = "[^abcd]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(true), matcher.matches(" "));
                assert_eq!(Ok(true), matcher.matches("["));
                assert_eq!(Ok(true), matcher.matches("]"));
                assert_eq!(Ok(true), matcher.matches("^"));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("b"));
                assert_eq!(Ok(false), matcher.matches("c"));
                assert_eq!(Ok(false), matcher.matches("d"));
                assert_eq!(Ok(true), matcher.matches("e"));
                assert_eq!(Ok(true), matcher.matches("A"));
                assert_eq!(Ok(true), matcher.matches("B"));
                assert_eq!(Ok(true), matcher.matches("C"));
                assert_eq!(Ok(true), matcher.matches("D"));
                assert_eq!(Ok(true), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }

            {
                let flags = IGNORE_CASE;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();

                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(true), matcher.matches(" "));
                assert_eq!(Ok(true), matcher.matches("["));
                assert_eq!(Ok(true), matcher.matches("]"));
                assert_eq!(Ok(true), matcher.matches("^"));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("b"));
                assert_eq!(Ok(false), matcher.matches("c"));
                assert_eq!(Ok(false), matcher.matches("d"));
                assert_eq!(Ok(true), matcher.matches("e"));
                assert_eq!(Ok(false), matcher.matches("A"));
                assert_eq!(Ok(false), matcher.matches("B"));
                assert_eq!(Ok(false), matcher.matches("C"));
                assert_eq!(Ok(false), matcher.matches("D"));
                assert_eq!(Ok(true), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_WILD1_1() {
            let pattern = "?";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(true), matcher.matches(" "));
                assert_eq!(Ok(true), matcher.matches("["));
                assert_eq!(Ok(true), matcher.matches("]"));
                assert_eq!(Ok(true), matcher.matches("^"));
                assert_eq!(Ok(true), matcher.matches("a"));
                assert_eq!(Ok(true), matcher.matches("b"));
                assert_eq!(Ok(true), matcher.matches("c"));
                assert_eq!(Ok(true), matcher.matches("d"));
                assert_eq!(Ok(true), matcher.matches("e"));
                assert_eq!(Ok(true), matcher.matches("A"));
                assert_eq!(Ok(true), matcher.matches("B"));
                assert_eq!(Ok(true), matcher.matches("C"));
                assert_eq!(Ok(true), matcher.matches("D"));
                assert_eq!(Ok(true), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_WILDN_1() {
            let pattern = "*";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(true), matcher.matches(""));
                assert_eq!(Ok(true), matcher.matches(" "));
                assert_eq!(Ok(true), matcher.matches("["));
                assert_eq!(Ok(true), matcher.matches("]"));
                assert_eq!(Ok(true), matcher.matches("^"));
                assert_eq!(Ok(true), matcher.matches("a"));
                assert_eq!(Ok(true), matcher.matches("b"));
                assert_eq!(Ok(true), matcher.matches("c"));
                assert_eq!(Ok(true), matcher.matches("d"));
                assert_eq!(Ok(true), matcher.matches("e"));
                assert_eq!(Ok(true), matcher.matches("A"));
                assert_eq!(Ok(true), matcher.matches("B"));
                assert_eq!(Ok(true), matcher.matches("C"));
                assert_eq!(Ok(true), matcher.matches("D"));
                assert_eq!(Ok(true), matcher.matches("E"));
                assert_eq!(Ok(true), matcher.matches("ab"));
                assert_eq!(Ok(true), matcher.matches("abc"));
                assert_eq!(Ok(true), matcher.matches("abcd"));
                assert_eq!(Ok(true), matcher.matches("ABCD"));
                assert_eq!(Ok(true), matcher.matches("abcde"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_LITERAL_THEN_WILDN_THEN_RANGE_1() {
            let pattern = "ma*[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("["));
                assert_eq!(Ok(false), matcher.matches("]"));
                assert_eq!(Ok(false), matcher.matches("^"));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("b"));
                assert_eq!(Ok(false), matcher.matches("c"));
                assert_eq!(Ok(false), matcher.matches("d"));
                assert_eq!(Ok(false), matcher.matches("e"));
                assert_eq!(Ok(false), matcher.matches("A"));
                assert_eq!(Ok(false), matcher.matches("B"));
                assert_eq!(Ok(false), matcher.matches("C"));
                assert_eq!(Ok(false), matcher.matches("D"));
                assert_eq!(Ok(false), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));


                assert_eq!(Ok(false), matcher.matches("ma"));
                assert_eq!(Ok(true), matcher.matches("mad"));
                assert_eq!(Ok(true), matcher.matches("made"));
                assert_eq!(Ok(true), matcher.matches("madder"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_THEN_LITERAL_THEN_WILDN_THEN_RANGE_1() {
            let pattern = "[mb]a*[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("["));
                assert_eq!(Ok(false), matcher.matches("]"));
                assert_eq!(Ok(false), matcher.matches("^"));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("b"));
                assert_eq!(Ok(false), matcher.matches("c"));
                assert_eq!(Ok(false), matcher.matches("d"));
                assert_eq!(Ok(false), matcher.matches("e"));
                assert_eq!(Ok(false), matcher.matches("A"));
                assert_eq!(Ok(false), matcher.matches("B"));
                assert_eq!(Ok(false), matcher.matches("C"));
                assert_eq!(Ok(false), matcher.matches("D"));
                assert_eq!(Ok(false), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("ae"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));


                assert_eq!(Ok(false), matcher.matches("ma"));
                assert_eq!(Ok(true), matcher.matches("bad"));
                assert_eq!(Ok(true), matcher.matches("bar"));
                assert_eq!(Ok(true), matcher.matches("bald"));
                assert_eq!(Ok(true), matcher.matches("bard"));
                assert_eq!(Ok(false), matcher.matches("cad"));
                assert_eq!(Ok(false), matcher.matches("car"));
                assert_eq!(Ok(true), matcher.matches("mad"));
                assert_eq!(Ok(true), matcher.matches("mar"));
                assert_eq!(Ok(true), matcher.matches("bade"));
                assert_eq!(Ok(false), matcher.matches("lade"));
                assert_eq!(Ok(true), matcher.matches("made"));
                assert_eq!(Ok(true), matcher.matches("badder"));
                assert_eq!(Ok(false), matcher.matches("ladder"));
                assert_eq!(Ok(true), matcher.matches("madder"));
            }
        }

        #[test]
        fn TEST_CompiledMatcher_parse_RANGE_THEN_LITERAL_THEN_WILD1_THEN_RANGE_1() {
            let pattern = "[mb]a?[der]";

            {
                let flags = 0;

                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(false), matcher.matches(""));
                assert_eq!(Ok(false), matcher.matches(" "));
                assert_eq!(Ok(false), matcher.matches("["));
                assert_eq!(Ok(false), matcher.matches("]"));
                assert_eq!(Ok(false), matcher.matches("^"));
                assert_eq!(Ok(false), matcher.matches("a"));
                assert_eq!(Ok(false), matcher.matches("b"));
                assert_eq!(Ok(false), matcher.matches("c"));
                assert_eq!(Ok(false), matcher.matches("d"));
                assert_eq!(Ok(false), matcher.matches("e"));
                assert_eq!(Ok(false), matcher.matches("A"));
                assert_eq!(Ok(false), matcher.matches("B"));
                assert_eq!(Ok(false), matcher.matches("C"));
                assert_eq!(Ok(false), matcher.matches("D"));
                assert_eq!(Ok(false), matcher.matches("E"));
                assert_eq!(Ok(false), matcher.matches("ab"));
                assert_eq!(Ok(false), matcher.matches("ae"));
                assert_eq!(Ok(false), matcher.matches("abc"));
                assert_eq!(Ok(false), matcher.matches("abcd"));
                assert_eq!(Ok(false), matcher.matches("ABCD"));
                assert_eq!(Ok(false), matcher.matches("abcde"));


                assert_eq!(Ok(false), matcher.matches("ma"));
                assert_eq!(Ok(false), matcher.matches("bad"));
                assert_eq!(Ok(false), matcher.matches("bar"));
                assert_eq!(Ok(true), matcher.matches("bald"));
                assert_eq!(Ok(true), matcher.matches("bard"));
                assert_eq!(Ok(false), matcher.matches("cad"));
                assert_eq!(Ok(false), matcher.matches("car"));
                assert_eq!(Ok(false), matcher.matches("mad"));
                assert_eq!(Ok(false), matcher.matches("mar"));
                assert_eq!(Ok(true), matcher.matches("bade"));
                assert_eq!(Ok(false), matcher.matches("lade"));
                assert_eq!(Ok(true), matcher.matches("made"));
                assert_eq!(Ok(false), matcher.matches("badder"));
                assert_eq!(Ok(false), matcher.matches("ladder"));
                assert_eq!(Ok(false), matcher.matches("madder"));
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


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(false), matcher.matches("ABCD"));
                }

                {
                    let flags = IGNORE_CASE;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("ABCD"));
                }
            }

            /* Using wildcards. */
            {
                let pattern = "a*c?";
                let flags = 0;
                let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                // assert_eq!(0, matcher.len());

                assert_eq!(Ok(true), matcher.matches("abcd"));
                assert_eq!(Ok(true), matcher.matches("a*c?"));
                assert_eq!(Ok(true), matcher.matches("abbbbbbbbcd"));
                assert_eq!(Ok(true), matcher.matches("acd"));
                assert_eq!(Ok(false), matcher.matches("abdc"));
                assert_eq!(Ok(true), matcher.matches("abc?"));
            }

            /* Using escaped characters. */
            {
                let pattern = "a\\*c\\?";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(false), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("a*c?"));
                    assert_eq!(Ok(false), matcher.matches("abbbbbbbbcd"));
                    assert_eq!(Ok(false), matcher.matches("acd"));
                    assert_eq!(Ok(false), matcher.matches("abdc"));
                    assert_eq!(Ok(false), matcher.matches("abc?"));
                }

                /*
                {
                    let flags = SUPPRESS_BACKSLASH_ESCAPE;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(false), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("a\\*c\\?"));
                }
                 */
            }

            /* Matching ranges. */
            {
                let pattern = "a[bc]c[defghijklm]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("accm"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));
                    assert_eq!(Ok(false), matcher.matches("a[bc]c[defghijklm]"));
                }

                /*
                {
                    let flags = SUPPRESS_RANGE_SUPPORT;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(false), matcher.matches("abcd"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(false), matcher.matches("accm"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));
                    assert_eq!(Ok(true), matcher.matches("a[bc]c[defghijklm]"));
                }
                 */
            }

            /* Matching ranges with continuum. */
            {
                let pattern = "a[b-c]c[d-m]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("abce"));
                    assert_eq!(Ok(true), matcher.matches("abcf"));
                    assert_eq!(Ok(true), matcher.matches("abcg"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("accm"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));

                    assert_eq!(Ok(false), matcher.matches("a-cm"));
                    assert_eq!(Ok(false), matcher.matches("acc-"));
                }

                /*
                {

                    const shwild::Pattern   pattern2("a[b-c]c[d-m]", SHWILD_F_SUPPRESS_RANGE_CONTINUUM_SUPPORT);

                    BDUT_ASSERT_TRUE(pattern2.match("abcd"));
                    BDUT_ASSERT_TRUE(pattern2.match("a-cd"));
                    BDUT_ASSERT_TRUE(pattern2.match("accd"));
                    BDUT_ASSERT_FALSE(pattern2.match("aacd"));
                    BDUT_ASSERT_TRUE(pattern2.match("accm"));
                    BDUT_ASSERT_FALSE(pattern2.match("accl"));
                    BDUT_ASSERT_FALSE(pattern2.match("abcn"));
                }
                 */
            }

            /* Matching ranges with high-low continuum. */
            {
                let pattern = "a[c-b]c[m-d]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("abce"));
                    assert_eq!(Ok(true), matcher.matches("abcf"));
                    assert_eq!(Ok(true), matcher.matches("abcg"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("accm"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));

                    assert_eq!(Ok(false), matcher.matches("a-cm"));
                    assert_eq!(Ok(false), matcher.matches("acc-"));
                }

                /*
                {

                    const shwild::Pattern   pattern2("a[b-c]c[d-m]", SHWILD_F_SUPPRESS_RANGE_CONTINUUM_SUPPORT);

                    BDUT_ASSERT_TRUE(pattern2.match("abcd"));
                    BDUT_ASSERT_TRUE(pattern2.match("a-cd"));
                    BDUT_ASSERT_TRUE(pattern2.match("accd"));
                    BDUT_ASSERT_FALSE(pattern2.match("aacd"));
                    BDUT_ASSERT_TRUE(pattern2.match("accm"));
                    BDUT_ASSERT_FALSE(pattern2.match("accl"));
                    BDUT_ASSERT_FALSE(pattern2.match("abcn"));
                }
                 */
            }

            /* Matching ranges with cross-case continuum. */
            {
                let pattern = "a[b-C]c[m-D]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("aCcJ"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("abce"));
                    assert_eq!(Ok(true), matcher.matches("abcf"));
                    assert_eq!(Ok(true), matcher.matches("abcg"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("accm"));
                    assert_eq!(Ok(false), matcher.matches("abcn"));

                    assert_eq!(Ok(false), matcher.matches("a-cm"));
                    assert_eq!(Ok(false), matcher.matches("acc-"));
                }
            }

            /* Matching ranges with wildcards as literals. */
            {
                let pattern = "a[*]c[?]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(false), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("a*c?"));
                    assert_eq!(Ok(false), matcher.matches("abbbbbbbbcd"));
                    assert_eq!(Ok(false), matcher.matches("acd"));
                    assert_eq!(Ok(false), matcher.matches("abdc"));
                    assert_eq!(Ok(false), matcher.matches("abc?"));
                }
            }

            /* Matching ranges with continuum and leading/trailing hyphens. */
            {
                let pattern = "a[-a-c]c[d-]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(true), matcher.matches("abcd"));
                    assert_eq!(Ok(true), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("acc-"));
                    assert_eq!(Ok(true), matcher.matches("a-c-"));
                    assert_eq!(Ok(false), matcher.matches("abce"));
                }
            }

            /* Matching ranges with inverse continuum. */
            {
                let pattern = "a[b-c]c[^d-m]";

                {
                    let flags = 0;
                    let matcher = shwild::CompiledMatcher::from_pattern_and_flags(pattern, flags).unwrap();


                    // assert_eq!(0, matcher.len());

                    assert_eq!(Ok(false), matcher.matches("abcd"));
                    assert_eq!(Ok(false), matcher.matches("aacd"));
                    assert_eq!(Ok(true), matcher.matches("abcc"));
                    assert_eq!(Ok(false), matcher.matches("accm"));
                    assert_eq!(Ok(true), matcher.matches("abcn"));
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
        fn TEST_matches_EMPTY_PATTERN_2() {
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
    }
}
