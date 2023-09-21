use ipa_translate;

#[test]
fn english_phonemes() {
	let phonemes = vec![
		("i=i:u<u:o<:ee<:e=e>:o=a<u>a=:", "ɪiːʊuːɔːeɛːəɜːɒæʌɑː"),
		("mnn>", "mnŋ"),
		("ptts=kbddz=g<", "pttʃkbddʒɡ"),
		("ft=ss=xvd=zz=h", "fθsʃxvðzʒh"),
		("lrjw", "lrjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::sil_to_ipa(group.0), group.1);
	}
}

#[test]
fn russian_phonemes() {
	let phonemes = vec![
		("iI=ueoa", "iɨueoa"),
		("mmj^nnj^", "mmʲnnʲ"),
		("ppj^ttj^kkj^bbj^ddj^g<g<j^", "ppʲttʲkkʲbbʲddʲɡɡʲ"),
		("tstsj^tc<", "tstsʲtɕ"),
		("ffj^ssj^s<c<:xxj^vvj^zzj^z<z>:g=", "ffʲssʲʂɕːxxʲvvʲzzʲʐʑːɣ"),
		("l~~lj^j", "ɫlʲj"),
		("rj^r", "rʲr"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::sil_to_ipa(group.0), group.1);
	}
}

#[test]
fn arabic_phonemes() {
	let phonemes = vec![
		("iui:u:aa:awaj", "iuiːuːaaːawaj"),
		("ptt?<^kq?=bdd?<^dz=g<", "pttˤkqʔbddˤdʒɡ"),
		("ft=ss?<^s=xx=h>hvd=zd=?<^z?<^g=R>?<", "fθssˤʃxχħhvðzðˤzˤɣʁʕ"),
		("mn", "mn"),
		("r", "r"),
		("ll~~jw", "lɫjw"),
	];

	for group in phonemes {
		assert_eq!(ipa_translate::sil_to_ipa(group.0), group.1);
	}
}
