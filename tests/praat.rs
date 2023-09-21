use ipa_translate;

#[test]
fn english_phonemes() {
	let phonemes = vec![
		(r"\ici\:f\hsu\:f\ct\:fe\ef\:f\sw", "ɪiːʊuːɔːeɛːə"),
		(r"\er\:f\ab\ae\vt\as\:f", "ɜːɒæʌɑː"),
		(r"mn\ng", "mnŋ"),
		(r"ptt\shkbdd\zh\gs", "pttʃkbddʒɡ"),
		(r"f\tfs\shxv\dhz\zhh", "fθsʃxvðzʒh"),
		(r"lrjw", "lrjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::praat_to_ipa(group.0), group.1);
	}
}

#[test]
fn russian_phonemes() {
	let phonemes = vec![
		(r"i\i-ueoa", "iɨueoa"),
		(r"mm\^jnn\^j", "mmʲnnʲ"),
		(r"pp\^jtt\^jkk\^jbb\^jdd\^j\gs\gs\^j", "ppʲttʲkkʲbbʲddʲɡɡʲ"),
		(r"tsts\^jt\cc", "tstsʲtɕ"),
		(r"ff\^jss\^j\s.\cc\:fxx\^jvv\^j", "ffʲssʲʂɕːxxʲvvʲ"),
		(r"zz\^j\z.\zc\:f\gf", "zzʲʐʑːɣ"),
		(r"\l~l\^jj", "ɫlʲj"),
		(r"r\^jr", "rʲr"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::praat_to_ipa(group.0), group.1);
	}
}

#[test]
fn arabic_phonemes() {
	let phonemes = vec![
		(r"iui\:fu\:faa\:fawaj", "iuiːuːaaːawaj"),
		(r"ptt\^9kq\?gbdd\^9d\zh\gs", "pttˁkqʔbddˁdʒɡ"),
		(r"f\tfss\^9\shx\cf\h-hv\dh", "fθssˁʃxχħhvð"),
		(r"z\dh\^9z\^9\gf\ri\9e", "zðˁzˁɣʁʕ"),
		(r"mn", "mn"),
		(r"r", "r"),
		(r"l\l~jw", "lɫjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::praat_to_ipa(group.0), group.1);
	}
}
