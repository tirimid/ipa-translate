use ipa_translate;

#[test]
fn english_phonemes() {
    let phonemes = vec![
        ("Ii:Uu:O:eE:@3:Q{VA:", "ɪiːʊuːɔːeɛːəɜːɒæʌɑː"),
        ("mnN", "mnŋ"),
        ("pttSkbddZg", "pttʃkbddʒɡ"),
        ("fTsSxvDzZh", "fθsʃxvðzʒh"),
        ("lrjw", "lrjw"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::xsampa_to_ipa(group.0), group.1);
    }
}

#[test]
fn russian_phonemes() {
    let phonemes = vec![
        ("i1ueoa", "iɨueoa"),
        ("mm_jnn_j", "mmʲnnʲ"),
        ("pp_jtt_jkk_jbb_jdd_jgg_j", "ppʲttʲkkʲbbʲddʲɡɡʲ"),
        ("tsts_jts\\", "tstsʲtɕ"),
        ("ff_jss_js`s\\:xx_jvv_jzz_jz`z\\:G", "ffʲssʲʂɕːxxʲvvʲzzʲʐʑːɣ"),
        ("5l_jj", "ɫlʲj"),
        ("r_jr", "rʲr"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::xsampa_to_ipa(group.0), group.1);
    }
}

#[test]
fn arabic_phonemes() {
    let phonemes = vec![
        ("iui:u:aa:awaj", "iuiːuːaaːawaj"),
        ("ptt_?\\kq?bdd_?\\dZg", "pttˤkqʔbddˤdʒɡ"),
        ("fTss_?\\SxXX\\hvDzD_?\\z_?\\GR?\\", "fθssˤʃxχħhvðzðˤzˤɣʁʕ"),
        ("mn", "mn"),
        ("r", "r"),
        ("l5jw", "lɫjw"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::xsampa_to_ipa(group.0), group.1);
    }
}
