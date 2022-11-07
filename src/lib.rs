use lazy_static::lazy_static;
use regex::Regex;

macro_rules! translation_pairs {
    ($($r:literal => $i:literal,)*) => {
        vec![$((Regex::new($r).unwrap(), $i),)*]
    };
}

pub fn xsampa_to_ipa(text: &str) -> String {
    lazy_static! {
        //static ref PAIRS: Vec<(Regex, &'static str)>
    }

    let mut text = text.to_string();
    for pair in PAIRS.iter() {
        text = pair.0.replace_all(&text, pair.1).to_string();
    }

    text
}
