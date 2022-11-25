use ipa_translate;

// this only really exists because it would be *really* embarrassing if the
// README contained invalid code...
#[test]
fn readme_test() {
    let ipa = "prʲɪvʲet";
    let xsampa = "pr_jIv_jet";
    let praat = r"pr\^j\icv\^jet";

    assert_eq!(ipa_translate::xsampa_to_ipa(xsampa), ipa);
    assert_eq!(ipa_translate::praat_to_ipa(praat), ipa);
}
