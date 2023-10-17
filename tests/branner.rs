use ipa_translate;

#[test]
fn english_phonemes() {
	let phonemes = vec![
		("Ii:Uu:c&:eE:@E&:a\"&ae)v&a\":", "ɪiːʊuːɔːeɛːəɜːɒæʌɑː"),
		("mnng)", "mnŋ"),
		("pttSkbdd3\"g", "pttʃkbddʒɡ"),
		("fO-sSxvd-z3\"h", "fθsʃxvðzʒh"),
		("lrjw", "lrjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::branner_to_ipa(group.0), group.1);
		assert_eq!(ipa_translate::ipa_to_branner(group.1), group.0);
	}
}

#[test]
fn russian_phonemes() {
	let phonemes = vec![
		("ii-ueoa", "iɨueoa"),
		("mmj^nnj^", "mmʲnnʲ"),
		("ppj^ttj^kkj^bbj^ddj^ggj^", "ppʲttʲkkʲbbʲddʲɡɡʲ"),
		("tstsj^tci)", "tstsʲtɕ"),
		("ffj^ssj^sr)ci):xxj^vvj^zzj^zr)zi):g\"", "ffʲssʲʂɕːxxʲvvʲzzʲʐʑːɣ"),
		("l~)lj^j", "ɫlʲj"),
		("rj^r", "rʲr"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::branner_to_ipa(group.0), group.1);
		assert_eq!(ipa_translate::ipa_to_branner(group.1), group.0);
	}
}

#[test]
fn arabic_phonemes() {
	let phonemes = vec![
		("iui:u:aa:awaj", "iuiːuːaaːawaj"),
		("ptt&g^kq?bdd&g^d3\"g", "pttˤkqʔbddˤdʒɡ"),
		("fO-ss&g^SxXh-hvd-zd-&g^z&g^g\"R%?&", "fθssˤʃxχħhvðzðˤzˤɣʁʕ"),
		("mn", "mn"),
		("r", "r"),
		("ll~)jw", "lɫjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::branner_to_ipa(group.0), group.1);
		assert_eq!(ipa_translate::ipa_to_branner(group.1), group.0);
	}
}
