use ipa_translate;

#[test]
fn english_phonemes() {
    let phonemes = vec![
        ("Ii:Uu:O:eE:@V\":A.&VA:", "ɪiːʊuːɔːeɛːəɜːɒæʌɑː"),
        ("mnN", "mnŋ"),
        ("pttSkbddZg", "pttʃkbddʒɡ"),
        ("fTsSxvDzZh<?>", "fθsʃxvðzʒh"),
        ("lr<trl>jw", "lrjw"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::kirshenbaum_to_ipa(group.0), group.1);
    }
}

#[test]
fn russian_phonemes() {
    let phonemes = vec![
        ("ii\"ueoa", "iɨueoa"),
        ("mn", "mn"),
        ("ptkbdg", "ptkbdɡ"),
        ("ts", "ts"),
        ("fss.xvzz.Q", "fsʂxvzʐɣ"),
        ("lj", "lj"),
        ("r<trl>", "r"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::kirshenbaum_to_ipa(group.0), group.1);
    }
}

#[test]
fn arabic_phonemes() {
    let phonemes = vec![
        ("iui:u:aa:awaj", "iuiːuːaaːawaj"),
        ("ptkq?bddZg", "ptkqʔbddʒɡ"),
        ("fTsSxXHh<?>vDzQg\"H<vcd>", "fθsʃxχħhvðzɣʁʕ"),
        ("mn", "mn"),
        ("r<trl>", "r"),
        ("ljw", "ljw"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::kirshenbaum_to_ipa(group.0), group.1);
    }
}
