// (C) 2020 Srimanta Barua <srimanta.barua1@gmail.com>

use crate::types::Tag;

macro_rules! scripts {
    ($pt:vis $name:ident { $( $feat:ident $stag:expr ),* } ) => {
        #[derive(Clone, Copy, Debug)]
        $pt enum $name {
            $( $feat ),*
        }

        impl $name {
            pub(crate) const fn tag(&self) -> Tag {
                match self {
                    $( $name::$feat => Tag::from($stag) ),*
                }
            }
        }
    };
}

scripts!(pub Script {
    Adlam                            b"adlm",
    Ahom                             b"ahom",
    AnatolianHieroglyphs             b"hluw",
    Arabic                           b"arab",
    Armenian                         b"armn",
    Avestan                          b"avst",
    Balinese                         b"bali",
    Bamum                            b"bamu",
    BassaVah                         b"bass",
    Batak                            b"batk",
    Bengali                          b"beng",
    BengaliV2                        b"bng2",
    Bhaiksuki                        b"bhks",
    Bopomofo                         b"bopo",
    Brahmi                           b"brah",
    Braille                          b"brai",
    Buginese                         b"bugi",
    Buhid                            b"buhd",
    ByzantineMusic                   b"byzm",
    CanadianSyllabics                b"cans",
    Carian                           b"cari",
    CaucasianAlbanian                b"aghb",
    Chakma                           b"cakm",
    Cham                             b"cham",
    Cherokee                         b"cher",
    Chorasmian                       b"chrs",
    CJKIdeographic                   b"hani",
    Coptic                           b"copt",
    CypriotSyllabary                 b"cprt",
    Cyrillic                         b"cyrl",
    Default                          b"DFLT",
    Deseret                          b"dsrt",
    Devanagari                       b"deva",
    DevanagariV2                     b"dev2",
    DivesAkuru                       b"diak",
    Dogra                            b"dogr",
    Duployan                         b"dupl",
    EgyptianHieroglyphs              b"egyp",
    Elbasan                          b"elba",
    Elymaic                          b"elym",
    Ethiopic                         b"ethi",
    Georgian                         b"geor",
    Glagolitic                       b"glag",
    Gothic                           b"goth",
    Grantha                          b"gran",
    Greek                            b"grek",
    Gujarati                         b"gujr",
    GujaratiV2                       b"gjr2",
    GunjalaGondi                     b"gong",
    Gurmukhi                         b"guru",
    GurmukhiV2                       b"gur2",
    Hangul                           b"hang",
    HangulJamo                       b"jamo",
    HanifiRohingya                   b"rohg",
    Hanunoo                          b"hano",
    Hatran                           b"hatr",
    Hebrew                           b"hebr",
    Hiragana                         b"kana",
    ImperialAramaic                  b"armi",
    InscriptionalPahlavi             b"phli",
    InscriptionalParthian            b"prti",
    Javanese                         b"java",
    Kaithi                           b"kthi",
    Kannada                          b"knda",
    KannadaV2                        b"knd2",
    Katakana                         b"kana",
    KayahLi                          b"kali",
    Kharosthi                        b"khar",
    KhitanSmallScript                b"kits",
    Khmer                            b"khmr",
    Khojki                           b"khoj",
    Khudawadi                        b"sind",
    Lao                              b"lao ",
    Latin                            b"latn",
    Lepcha                           b"lepc",
    Limbu                            b"limb",
    LinearA                          b"lina",
    LinearB                          b"linb",
    LisuFraser                       b"lisu",
    Lycian                           b"lyci",
    Lydian                           b"lydi",
    Mahajani                         b"mahj",
    Makasar                          b"maka",
    Malayalam                        b"mlym",
    MalayalamV2                      b"mlm2",
    Mandaic                          b"mand",
    Manichaean                       b"mani",
    Marchen                          b"marc",
    MasaramGondi                     b"gonm",
    MathematicalAlphanumericSymbols  b"math",
    Medefaidrin                      b"medf",
    MeiteiMayek                      b"mtei",
    MendeKikakui                     b"mend",
    MeroiticCursive                  b"merc",
    MeroiticHieroglyphs              b"mero",
    Miao                             b"plrd",
    Modi                             b"modi",
    Mongolian                        b"mong",
    Mro                              b"mroo",
    Multani                          b"mult",
    MusicalSymbols                   b"musc",
    Myanmar                          b"mymr",
    MyanmarV2                        b"mym2",
    Nabataean                        b"nbat",
    Nandinagari                      b"nand",
    Newa                             b"newa",
    NewTaiLue                        b"talu",
    NKo                              b"nko ",
    Nushu                            b"nshu",
    NyiakengPuachueHmong             b"hmnp",
    Odia                             b"orya",
    OdiaV2                           b"ory2",
    Ogham                            b"ogam",
    OlChiki                          b"olck",
    OldItalic                        b"ital",
    OldHungarian                     b"hung",
    OldNorthArabian                  b"narb",
    OldPermic                        b"perm",
    OldPersianCuneiform              b"xpeo",
    OldSogdian                       b"sogo",
    OldSouthArabian                  b"sarb",
    OldTurkic                        b"orkh",
    Osage                            b"osge",
    Osmanya                          b"osma",
    PahawhHmong                      b"hmng",
    Palmyrene                        b"palm",
    PauCinHau                        b"pauc",
    Phagspa                          b"phag",
    Phoenician                       b"phnx",
    PsalterPahlavi                   b"phlp",
    Rejang                           b"rjng",
    Runic                            b"runr",
    Samaritan                        b"samr",
    Saurashtra                       b"saur",
    Sharada                          b"shrd",
    Shavian                          b"shaw",
    Siddham                          b"sidd",
    SignWriting                      b"sgnw",
    Sinhala                          b"sinh",
    Sogdian                          b"sogd",
    SoraSompeng                      b"sora",
    Soyombo                          b"soyo",
    SumeroAkkadianCuneiform          b"xsux",
    Sundanese                        b"sund",
    SylotiNagri                      b"sylo",
    Syriac                           b"syrc",
    Tagalog                          b"tglg",
    Tagbanwa                         b"tagb",
    TaiLe                            b"tale",
    TaiTham                          b"lana",
    TaiViet                          b"tavt",
    Takri                            b"takr",
    Tamil                            b"taml",
    TamilV2                          b"tml2",
    Tangut                           b"tang",
    Telugu                           b"telu",
    TeluguV2                         b"tel2",
    Thaana                           b"thaa",
    Thai                             b"thai",
    Tibetan                          b"tibt",
    Tifinagh                         b"tfng",
    Tirhuta                          b"tirh",
    UgariticCuneiform                b"ugar",
    Vai                              b"vai ",
    Wancho                           b"wcho",
    WarangCiti                       b"wara",
    Yezidi                           b"yezi",
    Yi                               b"yi  ",
    ZanabazarSquare                  b"zanb"
});

impl Default for Script {
    fn default() -> Script {
        Script::Default
    }
}
