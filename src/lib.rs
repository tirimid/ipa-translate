use lazy_static::lazy_static;

type TranslationPair = (&'static str, &'static str);

macro_rules! load_translation_pairs {
	($($pair_var:ident = $pair_file:literal,)*) => {
		lazy_static! {
			$(static ref $pair_var: Vec<TranslationPair> = include!($pair_file);)*
		}
	};
}

load_translation_pairs! {
	PAIRS_XSAMPA = "../translations/xsampa.rs",
	PAIRS_PRAAT = "../translations/praat.rs",
	PAIRS_BRANNER = "../translations/branner.rs",
	PAIRS_SIL = "../translations/sil.rs",
}

fn translate_using_pairs(pairs: &Vec<TranslationPair>, text: &str) -> String {
	let mut text = text.to_string();
	for pair in pairs {
		text = text.replace(pair.0, pair.1);
	}
	text
}

fn invert_using_pairs(pairs: &Vec<TranslationPair>, text: &str) -> String {
	let mut text = text.to_string();
	for pair in pairs {
		text = text.replace(pair.1, pair.0);
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
	translate_using_pairs(&PAIRS_XSAMPA, text)
}

/// Inverse counterpart to `xsampa_to_ipa()`.
///
/// Usage example:
/// ```
/// let xsampa_text = "eks s{mp@";
/// let ipa_text = "eks sæmpə";
/// assert_eq!(ipa_translate::ipa_to_xsampa(ipa_text), xsampa_text);
/// ```
pub fn ipa_to_xsampa(text: &str) -> String {
	invert_using_pairs(&PAIRS_XSAMPA, text)
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
	translate_using_pairs(&PAIRS_PRAAT, text)
}

/// Inverse counterpart to `praat_to_ipa()`.
///
/// Usage example:
/// ```
/// let praat_text = r"p\rta\:ft\^h";
/// let ipa_text = "pɹaːtʰ";
/// assert_eq!(ipa_translate::ipa_to_praat(ipa_text), praat_text);
/// ```
pub fn ipa_to_praat(text: &str) -> String {
	invert_using_pairs(&PAIRS_PRAAT, text)
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
	translate_using_pairs(&PAIRS_BRANNER, text)
}

/// Inverse counterpart to `branner_to_ipa()`.
///
/// Usage example:
/// ```
/// let branner_text = "br&ae):nE&r^";
/// let ipa_text = "bɹæːnɜ˞";
/// assert_eq!(ipa_translate::ipa_to_branner(ipa_text), branner_text);
/// ```
pub fn ipa_to_branner(text: &str) -> String {
	invert_using_pairs(&PAIRS_BRANNER, text)
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
	translate_using_pairs(&PAIRS_SIL, text)
}

/// Inverse counterpart to `sil_to_ipa()`.
///
/// Usage example:
/// ```
/// let sil_text = "si=l";
/// let ipa_text = "sɪl";
/// assert_eq!(ipa_translate::ipa_to_sil(ipa_text), sil_text);
/// ```
pub fn ipa_to_sil(text: &str) -> String {
	invert_using_pairs(&PAIRS_SIL, text)
}
