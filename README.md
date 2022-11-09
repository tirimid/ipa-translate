# ipa-translate
A crate for translating ASCII text to IPA

## Usage
```rust
use ipa_translate;

fn main() {
    let ipa = "prʲɪvʲet miːr";
    let xsampa = "pr_jIv_jet mi:r";
    let kirshenbaum = "pr<trl><pzd>Iv<pzd>et mi:r<trl>";

    assert_eq!(ipa_translate::xsampa_to_ipa(xsampa), ipa);
    assert_eq!(ipa_translate::kirshenbaum_to_ipa(kirshenbaum), ipa);
}
```
