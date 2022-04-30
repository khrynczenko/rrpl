use regex::Regex;

pub trait TextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String;
}

#[derive(Debug, PartialEq)]
pub enum CaseInsensitivity {
    Enabled,
    Disabled,
}

impl From<bool> for CaseInsensitivity {
    fn from(case_insensitivity_is_enabled: bool) -> Self {
        if case_insensitivity_is_enabled {
            CaseInsensitivity::Enabled
        } else {
            CaseInsensitivity::Disabled
        }
    }
}

#[derive(Debug, Default)]
pub struct StdTextReplacer {}

impl TextReplacer for StdTextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String {
        text.replace(from, to)
    }
}

#[derive(Debug)]
pub struct RegexTextReplacer {
    case_insensitivity: CaseInsensitivity,
}

impl RegexTextReplacer {
    #[allow(dead_code)]
    pub fn new(case_insensitivity: CaseInsensitivity) -> Self {
        RegexTextReplacer { case_insensitivity }
    }
}

impl Default for RegexTextReplacer {
    fn default() -> Self {
        RegexTextReplacer {
            case_insensitivity: CaseInsensitivity::Disabled,
        }
    }
}

impl TextReplacer for RegexTextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String {
        let re = match self.case_insensitivity {
            CaseInsensitivity::Enabled => Regex::new(&format!(r"(?i){}", from)).unwrap(),
            CaseInsensitivity::Disabled => Regex::new(from).unwrap(),
        };

        return re.replace_all(text, String::from(to)).to_string();
    }
}

pub fn make_text_replacer(case_insensitivity: CaseInsensitivity) -> Box<dyn TextReplacer> {
    match case_insensitivity {
        CaseInsensitivity::Enabled => Box::new(RegexTextReplacer::new(case_insensitivity)),
        CaseInsensitivity::Disabled => Box::new(StdTextReplacer::default()),
    }
}

#[cfg(test)]
mod tests {

    mod std_text_replacer {
        use super::super::{StdTextReplacer, TextReplacer};

        #[test]
        fn replacing_words_in_text() {
            let text = "Some text with some words";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("Some text with some souce");

            assert_eq!(
                StdTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }

        #[test]
        fn replacing_words_in_text_without_words_to_replace() {
            let text = "nothing to replace here";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("nothing to replace here");

            assert_eq!(
                StdTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }

        #[test]
        fn replacing_words_in_empty_text() {
            let text = "";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("");

            assert_eq!(
                StdTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }
    }

    mod regex_text_replacer {
        use super::super::{CaseInsensitivity, RegexTextReplacer, TextReplacer};

        #[test]
        fn replacing_words_in_text() {
            let text = "Some text with some words";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("Some text with some souce");

            assert_eq!(
                RegexTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }

        #[test]
        fn replacing_words_in_text_case_insensitively() {
            let text = "Some text with some Words, wOrDs, and other words";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("Some text with some souce, souce, and other souce");

            let replacer = RegexTextReplacer::new(CaseInsensitivity::Enabled);

            assert_eq!(replacer.replace(from, to, text), expected_output);
        }

        #[test]
        fn replacing_words_in_text_without_words_to_replace() {
            let text = "nothing to replace here";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("nothing to replace here");

            assert_eq!(
                RegexTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }

        #[test]
        fn replacing_words_in_empty_text() {
            let text = "";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("");

            assert_eq!(
                RegexTextReplacer::default().replace(from, to, text),
                expected_output
            );
        }
    }
}
