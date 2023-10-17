use ipa_translate;

// this only really exists because it would be *really* embarrassing if the
// README contained invalid code...
#[test]
fn readme_test() {
	let ipa = "prʲɪvʲet";
	
	let xsampa = "pr_jIv_jet";
	let praat = r"pr\^j\icv\^jet";
	let branner = "prj^Ivj^et";
	let sil = "prj^i=vj^et";

	// forward translation.
	assert_eq!(ipa_translate::xsampa_to_ipa(xsampa), ipa);
	assert_eq!(ipa_translate::praat_to_ipa(praat), ipa);
	assert_eq!(ipa_translate::branner_to_ipa(branner), ipa);
	assert_eq!(ipa_translate::sil_to_ipa(sil), ipa);

	// inverse translation.
	assert_eq!(ipa_translate::ipa_to_xsampa(ipa), xsampa);
	assert_eq!(ipa_translate::ipa_to_praat(ipa), praat);
	assert_eq!(ipa_translate::ipa_to_branner(ipa), branner);
	assert_eq!(ipa_translate::ipa_to_sil(ipa), sil);
}
