mod phonetics;

use lazy_static::lazy_static;
use regex::Regex;

type TranslationPair = (Regex, &'static str);

macro_rules! load_translation_pairs {
    ($pair_var:ident = $pair_file:literal) => {
        lazy_static! {
            static ref $pair_var: Vec<TranslationPair> = {
                include!($pair_file)
                    .into_iter()
                    .map(|(r, e)| (Regex::new(r).unwrap(), e))
                    .collect::<Vec<_>>()
            };
        }
    };
}

fn translate_using_pairs(pairs: &Vec<TranslationPair>, text: &str) -> String {
    let mut text = text.to_string();
    for pair in pairs.iter() {
        text = pair.0.replace_all(&text, pair.1).to_string();
    }

    text
}

/// Translates an X-SAMPA ASCII string to a unicode IPA string.
/// Everything on the Wikipedia article "X-SAMPA" should be implemented here.
///
/// Example:
/// ```
/// let xsampa_text = "eks s{mp@";
/// let ipa_text = "eks sæmpə";
/// assert_eq!(ipa_translate::xsampa_to_ipa(xsampa_text), ipa_text);
/// ```
pub fn xsampa_to_ipa(text: &str) -> String {
    load_translation_pairs!(PAIRS = "../translations/xsampa.rs");
    translate_using_pairs(&PAIRS, text)
}

/// Translates a Praat ASCII string to a unicode IPA string.
/// Everything in the Praat manual entry under "Phonetic symbols" should be
/// implemented here.
///
/// Example:
/// ```
/// let praat_text = r"p\rta\:ft\^h";
/// let ipa_text = "pɹaːtʰ";
/// assert_eq!(ipa_translate::praat_to_ipa(praat_text), ipa_text);
/// ```
pub fn praat_to_ipa(text: &str) -> String {
    load_translation_pairs!(PAIRS = "../translations/praat.rs");
    translate_using_pairs(&PAIRS, text)
}

/// Translates a Kirshenbaum ASCII string to a unicode IPA string.
/// This is not implemented fully according to specification.
/// Blah blah blah list the changes here eventually.
///
/// Example:
/// ```
/// let kirshenbaum_text = "k{lmd,cnt,unr,vwl}:rS@nbO:m";
/// let ipa_text = "kɜːrʃənbɔːm";
// assert_eq!(ipa_translate::kirshenbaum_to_ipa(kirshenbaum_text), ipa_text);
/// ```
fn kirshenbaum_to_ipa(text: &str) -> String {
    String::new()
}
