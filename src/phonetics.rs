use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub enum Place {
    Bilabial = 0,
    Labiodental = 1,
    LabialVelar = 2,
    LabialPalatal = 3,
    Dental = 4,
    Alveolar = 5,
    Postalveolar = 6,
    Alveolopalatal = 7,
    Retroflex = 8,
    Palatal = 9,
    Velar = 10,
    Uvular = 11,
    Pharyngeal = 12,
    Glottal = 13,
    Epiglottal = 14,
}

#[derive(Clone, Copy)]
pub enum Manner {
    Plosive = 0,
    Nasal = 1,
    Trill = 2,
    Flap = 3,
    Fricative = 4,
    Approximant = 5,
    Click = 6,
    Implosive = 7,
}

#[derive(Clone, Copy)]
pub enum Position {
    Front = 0,
    NearFront = 1,
    Center = 2,
    NearBack = 3,
    Back = 4,
}

#[derive(Clone, Copy)]
pub enum Openness {
    Close = 0,
    NearClose = 1,
    CloseMid = 2,
    Mid = 3,
    OpenMid = 4,
    NearOpen = 5,
    Open = 6,
}

// the feature for "velarized or pharyngealized" is not added here.
#[derive(Clone, Copy)]
pub enum Feature {
    // non-spacing diacritics.
    Voiceless = 1 << 0,
    Voiced = 1 << 1,
    MoreRounded = 1 << 2,
    LessRounded = 1 << 3,
    Advanced = 1 << 4,
    Retracted = 1 << 5,
    Centralized = 1 << 6,
    MidCentralized = 1 << 7,
    Syllabic = 1 << 8,
    NonSyllabic = 1 << 9,
    Rhotic = 1 << 10,
    BreathyVoiced = 1 << 11,
    CreakyVoiced = 1 << 12,
    Linguolabial = 1 << 13,
    Raised = 1 << 14,
    Lowered = 1 << 15,
    AdvancedTongueRoot = 1 << 16,
    RetractedTongueRoot = 1 << 17,
    Dental = 1 << 18,
    Apical = 1 << 19,
    Laminal = 1 << 20,
    Nasalized = 1 << 21,
    NoAudibleRelease = 1 << 22,

    // spacing diacritics.
    Aspirated = 1 << 23,
    Palatalized = 1 << 24,
    Labialized = 1 << 25,
    Velarized = 1 << 26,
    Pharyngealized = 1 << 27,
    NasalRelease = 1 << 28,
    LateralRelease = 1 << 29,
    Ejective = 1 << 30,
    Long = 1 << 31,
    HalfLong = 1 << 32,
}

macro_rules! non_pat_match {
    ($val:ident {$e:expr => $ret:expr, $($e_arg:expr => $ret_arg:expr,)*}) => {
        if $val == $e {
            $ret
        }

        $(
            else if $val == $e_arg {
                $ret_arg
            }
        )*

        else {
            panic!("value did not match any non-pattern options");
        }
    };
}

impl From<u128> for Feature {
    fn from(u: u128) -> Self {
        non_pat_match! {
            u {
                1 << 0 => Self::Voiceless,
                1 << 1 => Self::Voiced,
                1 << 2 => Self::MoreRounded,
                1 << 3 => Self::LessRounded,
                1 << 4 => Self::Advanced,
                1 << 5 => Self::Retracted,
                1 << 6 => Self::Centralized,
                1 << 7 => Self::MidCentralized,
                1 << 8 => Self::Syllabic,
                1 << 9 => Self::NonSyllabic,
                1 << 10 => Self::Rhotic,
                1 << 11 => Self::BreathyVoiced,
                1 << 12 => Self::CreakyVoiced,
                1 << 13 => Self::Linguolabial,
                1 << 14 => Self::Raised,
                1 << 15 => Self::Lowered,
                1 << 16 => Self::AdvancedTongueRoot,
                1 << 17 => Self::RetractedTongueRoot,
                1 << 18 => Self::Dental,
                1 << 19 => Self::Apical,
                1 << 20 => Self::Laminal,
                1 << 21 => Self::Nasalized,
                1 << 22 => Self::NoAudibleRelease,
                1 << 23 => Self::Aspirated,
                1 << 24 => Self::Palatalized,
                1 << 25 => Self::Labialized,
                1 << 26 => Self::Velarized,
                1 << 27 => Self::Pharyngealized,
                1 << 28 => Self::NasalRelease,
                1 << 29 => Self::LateralRelease,
                1 << 30 => Self::Ejective,
                1 << 31 => Self::Long,
                1 << 32 => Self::HalfLong,
            }
        }
    }
}

// conversion of features to diacritics.
impl Into<char> for Feature {
    fn into(self) -> char {
        match self {
            Self::Voiceless => '̥',
            Self::Voiced => '̬',
            Self::Aspirated => 'ʰ',
            Self::MoreRounded => '̹',
            Self::LessRounded => '̜',
            Self::Advanced => '̟',
            Self::Retracted => '̠',
            Self::Centralized => '̈',
            Self::MidCentralized => '̽',
            Self::Syllabic => '̩',
            Self::NonSyllabic => '̯',
            Self::Rhotic => '˞',
            Self::BreathyVoiced => '̤',
            Self::CreakyVoiced => '̰',
            Self::Linguolabial => '̼',
            Self::Labialized => 'ʷ',
            Self::Palatalized => 'ʲ',
            Self::Velarized => 'ˠ',
            Self::Pharyngealized => 'ˤ',
            Self::Raised => '̝',
            Self::Lowered => '̞',
            Self::AdvancedTongueRoot => '̘',
            Self::RetractedTongueRoot => '̙',
            Self::Dental => '̪',
            Self::Apical => '̺',
            Self::Laminal => '̻',
            Self::Nasalized => '̃',
            Self::NasalRelease => 'ⁿ',
            Self::LateralRelease => 'ˡ',
            Self::NoAudibleRelease => '̚',
            Self::Ejective => 'ʼ',
            Self::Long => 'ː',
            Self::HalfLong => 'ˑ',
        }
    }
}

macro_rules! ipa_chart {
    ($([$($pairs:literal)*] $(,)*)*) => {
        [
            $([
                $((
                    $pairs.chars().nth(0).unwrap(),
                    $pairs.chars().nth(1).unwrap(),
                ),)*
            ],)*
        ]
    };
}

lazy_static! {
    static ref CONSONANT_CHART: [[(char, char); 15]; 8] = ipa_chart![
        ["pb""  ""  ""  ""  ""td""  ""  ""ʈɖ""cɟ""kg""qɢ""  ""ʔ "" ʡ"],
        [" m"" ɱ""  ""  ""  "" n""  ""  "" ɳ"" ɲ"" ŋ"" ɴ""  ""  ""  "],
        [" ʙ""  ""  ""  ""  "" r""  ""  ""  ""  ""  "" ʀ""  ""  ""  "],
        ["  "" ⱱ""  ""  ""  "" ɾ""  ""  "" ɽ""  ""  ""  ""  ""  ""  "],
        ["ɸβ""fv""ʍw""  ""θð""sz""ʃʒ""ɕʑ""ʂʐ""çʝ""xɣ""χʁ""ħʕ""hɦ""ʜʢ"],
        ["  "" ʋ""  "" ɥ""  "" ɹ""  ""  "" ɻ"" j"" ɰ""  ""  ""  ""  "],
        ["ʘ ""  ""  ""  ""ǀ ""ǂ ""  ""  ""ǃ ""  ""  ""  ""  ""  ""  "],
        [" ɓ""  ""  ""  ""  "" ɗ""  ""  ""  "" ʄ"" ɠ"" ʛ""  ""  ""  "],
    ];

    static ref LATERAL_CONSONANT_CHART: [[(char, char); 15]; 8] = ipa_chart![
        ["  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  "" ɺ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  ""ɬɮ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  "" l""  ""  "" ɭ"" ʎ"" ʟ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  ""ǁ ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
        ["  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  ""  "],
    ];

    static ref VOWEL_CHART: [[(char, char); 5]; 7] = ipa_chart![
        ["iy""  ""ɨʉ""  ""ɯu"],
        ["  ""ɪʏ""  "" ʊ""  "],
        ["eø""  ""ɘɵ""  ""ɤo"],
        ["  ""  ""əə""  ""  "],
        ["ɛœ""  ""ɜɞ""  ""ʌɔ"],
        ["æ ""  ""ɐɐ""  ""  "],
        ["aɶ""  ""ä ""  ""ɑɒ"],
    ];
}

pub struct Consonant {
    place: Place,
    manner: Manner,
    features: u128,
    voiced: bool,
    lateral: bool,
}

pub struct Vowel {
    openness: Openness,
    position: Position,
    features: u128,
    rounded: bool,
}

pub trait ToIpa {
    // a `None` is returned if a given phoneme does not have an IPA symbol.
    // or... if conversion to that symbol is not supported.
    fn to_ipa(&self) -> Option<String>;
}

macro_rules! flag_set_method {
    ($fn_name:ident, $field_name:ident $(,)*) => {
        pub fn $fn_name(mut self) -> Self {
            self.$field_name = true;
            self
        }
    };

    ($fn_field_name:ident $(,)*) => {
        flag_set_method!($fn_field_name, $fn_field_name);
    }
}

impl Consonant {
    pub fn new(place: Place, manner: Manner) -> Self {
        Self {
            place,
            manner,
            features: 0,
            voiced: false,
            lateral: false,
        }
    }

    pub fn feature(mut self, feat: Feature) -> Self {
        self.features |= feat as u128;
        self
    }

    flag_set_method!(voiced);
    flag_set_method!(lateral);
}

impl ToIpa for Consonant {
    fn to_ipa(&self) -> Option<String> {
        let mut text = String::new();
        let features = (0..127)
            .filter(|n| self.features & 1 << n > 0)
            .map(|n| Feature::from(1 << n))
            .collect::<Vec<_>>();

        let row = self.manner as usize;
        let col = self.place as usize;
        let chart = match self.lateral {
            true => *LATERAL_CONSONANT_CHART,
            false => *CONSONANT_CHART,
        };

        let c = match self.voiced {
            true => chart[row][col].1,
            false => chart[row][col].0,
        };

        text.push(c);
        for feat in features {
            text.push(feat.into());
        }

        Some(text)
    }
}

impl Vowel {
    pub fn new(openness: Openness, position: Position) -> Self {
        Self {
            openness,
            position,
            features: 0,
            rounded: false,
        }
    }

    pub fn feature(mut self, feat: Feature) -> Self {
        self.features |= feat as u128;
        self
    }

    flag_set_method!(rounded);
}

impl ToIpa for Vowel {
    fn to_ipa(&self) -> Option<String> {
        let mut text = String::new();
        let features = (0..127)
            .filter(|n| self.features & 1 << n > 0)
            .map(|n| Feature::from(1 << n))
            .collect::<Vec<_>>();

        let row = self.openness as usize;
        let col = self.position as usize;
        let v = match self.rounded {
            true => VOWEL_CHART[row][col].1,
            false => VOWEL_CHART[row][col].0,
        };

        text.push(v);
        for feat in features {
            text.push(feat.into());
        }

        Some(text)
    }
}

pub enum Phoneme {
    Consonant(Consonant),
    Vowel(Vowel),
}

impl ToIpa for Phoneme {
    fn to_ipa(&self) -> Option<String> {
        match self {
            Self::Consonant(ref c) => c.to_ipa(),
            Self::Vowel(ref v) => v.to_ipa(),
        }
    }
}
