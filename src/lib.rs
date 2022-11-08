use lazy_static::lazy_static;
use regex::Regex;

macro_rules! simple_translation {
    ($fn_name:ident, $path:literal $(,)*) => {
        pub fn $fn_name(text: &str) -> String {
            lazy_static! {
                static ref PAIRS: Vec<(Regex, &'static str)> = {
                    include!($path)
                        .into_iter()
                        .map(|(r, e)| (Regex::new(r).unwrap(), e))
                        .collect::<Vec<_>>()
                };
            }

            let mut text = text.to_string();
            for pair in PAIRS.iter() {
                text = pair.0.replace_all(&text, pair.1).to_string();
            }

            text
        }
    };
}

simple_translation!(xsampa_to_ipa, "../translations/xsampa.in");
simple_translation!(kirshenbaum_to_ipa, "../translations/kirshenbaum.in");
