use regex::Regex;

pub trait TextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String;
}

#[derive(Debug, PartialEq)]
pub enum WholeWordsOnly {
    Enabled,
    Disabled,
}

impl From<bool> for WholeWordsOnly {
    fn from(whole_words_only_is_enabled: bool) -> Self {
        if whole_words_only_is_enabled {
            WholeWordsOnly::Enabled
        } else {
            WholeWordsOnly::Disabled
        }
    }
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
    whole_words: WholeWordsOnly,
}

impl RegexTextReplacer {
    #[allow(dead_code)]
    pub fn new(case_insensitivity: CaseInsensitivity, whole_words: WholeWordsOnly) -> Self {
        RegexTextReplacer {
            case_insensitivity,
            whole_words,
        }
    }
}

impl Default for RegexTextReplacer {
    fn default() -> Self {
        RegexTextReplacer {
            case_insensitivity: CaseInsensitivity::Disabled,
            whole_words: WholeWordsOnly::Disabled,
        }
    }
}

impl TextReplacer for RegexTextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String {
        let re = match (&self.case_insensitivity, &self.whole_words) {
            (CaseInsensitivity::Enabled, WholeWordsOnly::Enabled) => {
                Regex::new(&format!(r"(?i)\b{}", from)).unwrap()
            }
            (CaseInsensitivity::Disabled, WholeWordsOnly::Enabled) => {
                Regex::new(&format!(r"\b{}", from)).unwrap()
            }
            (CaseInsensitivity::Enabled, WholeWordsOnly::Disabled) => {
                Regex::new(&format!(r"(?i){}", from)).unwrap()
            }
            (CaseInsensitivity::Disabled, WholeWordsOnly::Disabled) => Regex::new(from).unwrap(),
        };

        return re.replace_all(text, String::from(to)).to_string();
    }
}

pub fn make_text_replacer(
    case_insensitivity: CaseInsensitivity,
    whole_words: WholeWordsOnly,
) -> Box<dyn TextReplacer> {
    match (case_insensitivity, whole_words) {
        (CaseInsensitivity::Enabled, whole_words) => Box::new(RegexTextReplacer::new(
            CaseInsensitivity::Enabled,
            whole_words,
        )),
        (case_insensitivity, WholeWordsOnly::Enabled) => Box::new(RegexTextReplacer::new(
            case_insensitivity,
            WholeWordsOnly::Enabled,
        )),
        (CaseInsensitivity::Disabled, _) => Box::new(StdTextReplacer::default()),
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
        use super::super::{CaseInsensitivity, RegexTextReplacer, TextReplacer, WholeWordsOnly};

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
        fn replacing_words_in_text_for_whole_words_only() {
            let text = "Some text with some words, words. But not xwordsx.";
            let from = "words";
            let to = "souce";

            let expected_output =
                String::from("Some text with some souce, souce. But not xwordsx.");

            let replacer =
                RegexTextReplacer::new(CaseInsensitivity::Disabled, WholeWordsOnly::Enabled);

            assert_eq!(replacer.replace(from, to, text), expected_output);
        }

        #[test]
        fn replacing_words_in_text_case_insensitively() {
            let text = "Some text with some Words, wOrDs, and other words";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("Some text with some souce, souce, and other souce");

            let replacer =
                RegexTextReplacer::new(CaseInsensitivity::Enabled, WholeWordsOnly::Disabled);

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
