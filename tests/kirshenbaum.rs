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
        ("mm<pzd>nn<pzd>", "mmʲnnʲ"),
        ("pp<pzd>tt<pzd>kk<pzd>bb<pzd>dd<pzd>gg<pzd>", "ppʲttʲkkʲbbʲddʲɡɡʲ"),
        ("tsts<pzd>", "tstsʲ"),
        ("ff<pzd>ss<pzd>s.xx<pzd>vv<pzd>zz<pzd>z.Q", "ffʲssʲʂxxʲvvʲzzʲʐɣ"),
        ("l<pzd>j", "lʲj"),
        ("r<trl><pzd>r<trl>", "rʲr"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::kirshenbaum_to_ipa(group.0), group.1);
    }
}

#[test]
fn arabic_phonemes() {
    let phonemes = vec![
        ("iui:u:aa:awaj", "iuiːuːaaːawaj"),
        ("ptt<H>kq?bdd<H>dZg", "pttˤkqʔbddˤdʒɡ"),
        ("fTss<H>SxXHh<?>vDzD<H>z<H>Qg\"H<vcd>", "fθssˤʃxχħhvðzðˤzˤɣʁʕ"),
        ("mn", "mn"),
        ("r<trl>", "r"),
        ("ljw", "ljw"),
    ];

    for group in phonemes {
        assert_eq!(ipa_translate::kirshenbaum_to_ipa(group.0), group.1);
    }
}
