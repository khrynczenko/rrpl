pub trait TextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String;
}

pub struct StdTextReplacer {}

impl TextReplacer for StdTextReplacer {
    fn replace(&self, from: &str, to: &str, text: &str) -> String {
        text.replace(from, to)
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

            assert_eq!(StdTextReplacer {}.replace(from, to, text), expected_output);
        }

        #[test]
        fn replacing_words_in_text_without_words_to_replace() {
            let text = "nothing to replace here";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("nothing to replace here");

            assert_eq!(StdTextReplacer {}.replace(from, to, text), expected_output);
        }

        #[test]
        fn replacing_words_in_empty_text() {
            let text = "";
            let from = "words";
            let to = "souce";

            let expected_output = String::from("");

            assert_eq!(StdTextReplacer {}.replace(from, to, text), expected_output);
        }
    }
}
