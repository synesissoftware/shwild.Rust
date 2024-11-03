use regex::Regex;

fn main() {
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
