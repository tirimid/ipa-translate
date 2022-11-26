# ipa-translate
A crate for translating ASCII text to IPA

## Usage
```rust
use ipa_translate;

fn main() {
    let ipa = "prʲɪvʲet";
    let xsampa = "pr_jIv_jet";
    let praat = r"pr\^j\icv\^jet";
    let branner = "prj^Ivj^et";

    assert_eq!(ipa_translate::xsampa_to_ipa(xsampa), ipa);
    assert_eq!(ipa_translate::praat_to_ipa(praat), ipa);
    assert_eq!(ipa_translate::branner_to_ipa(branner), ipa);
}
```
