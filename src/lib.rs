// this is not currently used.
// the `phonetics` module will be used when I implement Kirshenbaum properly.
//mod phonetics;

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
/// Implemented according to [this Wikipedia article](https://en.wikipedia.org/wiki/X-SAMPA).
///
/// Usage example:
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
/// Implemented according to [this Praat manual entry](https://www.fon.hum.uva.nl/praat/manual/Phonetic_symbols.html).
///
/// Usage example:
/// ```
/// let praat_text = r"p\rta\:ft\^h";
/// let ipa_text = "pɹaːtʰ";
/// assert_eq!(ipa_translate::praat_to_ipa(praat_text), ipa_text);
/// ```
pub fn praat_to_ipa(text: &str) -> String {
    load_translation_pairs!(PAIRS = "../translations/praat.rs");
    translate_using_pairs(&PAIRS, text)
}

/// Translates a "Branner" ASCII string to a unicode IPA string.
/// Implemented according to [this Wikipedia article](https://en.wikipedia.org/wiki/Comparison_of_ASCII_encodings_of_the_International_Phonetic_Alphabet).
///
/// Usage example:
/// ```
/// let branner_text = "br&ae):nE&r^";
/// let ipa_text = "bɹæːnɜ˞";
/// assert_eq!(ipa_translate::branner_to_ipa(branner_text), ipa_text);
/// ```
pub fn branner_to_ipa(text: &str) -> String {
    load_translation_pairs!(PAIRS = "../translations/branner.rs");
    translate_using_pairs(&PAIRS, text)
}

/// Translates a SIL ASCII string to a unicode IPA string.
/// Implemented according to [this KeymanHelp page](https://help.keyman.com/keyboard/sil_ipa/1.8.6/sil_ipa).
///
/// Usage example:
/// ```
/// let sil_text = "si=l";
/// let ipa_text = "sɪl";
/// assert_eq!(ipa_translate::sil_to_ipa(sil_text), ipa_text);
/// ```
pub fn sil_to_ipa(text: &str) -> String {
    load_translation_pairs!(PAIRS = "../translations/sil.rs");
    translate_using_pairs(&PAIRS, text)
}
