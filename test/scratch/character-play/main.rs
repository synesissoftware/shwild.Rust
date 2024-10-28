

use regex::Regex;

fn main() {

    fn describe_string(s : &str)
    {
        eprintln!("s='{s}' (len={}, #chars={}); chars={:?}; bytes={:?}; utf16={:?} / {:?}", s.len(), s.chars().count(), s.chars(), s.as_bytes(), s.encode_utf16().collect::<Vec<u16>>(), s.encode_utf16().map(|u| format!("{u:#04x}")).collect::<Vec<String>>());
    }

    {
        describe_string("");
        describe_string("S");
        describe_string("s");
        describe_string("é");
        describe_string("é");
        describe_string("🐻");
        describe_string("👀");
        describe_string("🛑");
        describe_string("🐻👀🛑");
        describe_string("👁️");
        describe_string("-👁️-");
        describe_string("👁");
        describe_string("\u{1f440}");
        describe_string("\u{1f441}");
        describe_string("-\u{1f440}-");
        describe_string("-\u{1f441}-");
    }


    {
        let re = Regex::new("abc").unwrap();

        assert!(re.is_match("abc"));
        assert!(re.is_match("abcd"));
    }

    {
        let re = Regex::new("abc$").unwrap();

        assert!(re.is_match("abc"));
        assert!(!re.is_match("abcd"));
    }

    {
        let re = Regex::new("a🐻c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🐻cd"));
    }

    {
        let re = Regex::new("aéc").unwrap();

        assert!(re.is_match("aéc"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("aécd"));
    }

    {
        let re = Regex::new("aéc").unwrap();

        assert!(re.is_match("aéc"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("aécd"));
    }

    {
        let re = Regex::new("a[🐻👀🛑]c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(re.is_match("a👀c"));
        assert!(re.is_match("a🛑c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🛑cd"));
    }

    {
        let re = Regex::new("a👁️c").unwrap();

        assert!(!re.is_match("a🐻c"));
        assert!(!re.is_match("a👀c"));
        assert!(!re.is_match("a🛑c"));
        assert!(re.is_match("a👁️c"));
        assert!(!re.is_match("abc"));
        assert!(!re.is_match("a🛑cd"));
    }

    {
        let re = Regex::new("a[🐻👀🛑]c").unwrap();

        assert!(re.is_match("a🐻c"));
        assert!(re.is_match("a👀c"));
        assert!(re.is_match("a🛑c"));
        assert!(!re.is_match("abc"));
        assert!(re.is_match("a🛑cd"));
    }
}
