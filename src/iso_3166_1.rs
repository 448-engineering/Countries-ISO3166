/// This is the country Code.
/// Working with country codes is much easier than working with country names
/// since some of them have special UTF-8 characters that can cause confusion.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum CountryIso31661 {
    AF,
    AX,
    AL,
    DZ,
    AS,
    AD,
    AO,
    AI,
    AQ,
    AG,
    AR,
    AM,
    AW,
    AU,
    AT,
    AZ,
    BS,
    BH,
    BD,
    BB,
    BY,
    BE,
    BZ,
    BJ,
    BM,
    BT,
    BO,
    BQ,
    BA,
    BW,
    BV,
    BR,
    IO,
    BN,
    BG,
    BF,
    BI,
    KH,
    CM,
    CA,
    CV,
    KY,
    CF,
    TD,
    CL,
    CN,
    CX,
    CC,
    CO,
    KM,
    CG,
    CD,
    CK,
    CR,
    CI,
    HR,
    CU,
    CW,
    CY,
    CZ,
    DK,
    DJ,
    DM,
    DO,
    EC,
    EG,
    SV,
    GQ,
    ER,
    EE,
    ET,
    FK,
    FO,
    FJ,
    FI,
    FR,
    GF,
    PF,
    TF,
    GA,
    GM,
    GE,
    DE,
    GH,
    GI,
    GR,
    GL,
    GD,
    GP,
    GU,
    GT,
    GG,
    GN,
    GW,
    GY,
    HT,
    HM,
    VA,
    HN,
    HK,
    HU,
    IS,
    IN,
    ID,
    IR,
    IQ,
    IE,
    IM,
    IL,
    IT,
    JM,
    JP,
    JE,
    JO,
    KZ,
    KE,
    KI,
    KP,
    KR,
    KW,
    KG,
    LA,
    LV,
    LB,
    LS,
    LR,
    LY,
    LI,
    LT,
    LU,
    MO,
    MK,
    MG,
    MW,
    MY,
    MV,
    ML,
    MT,
    MH,
    MQ,
    MR,
    MU,
    YT,
    MX,
    FM,
    MD,
    MC,
    MN,
    ME,
    MS,
    MA,
    MZ,
    MM,
    NA,
    NR,
    NP,
    NL,
    NC,
    NZ,
    NI,
    NE,
    NG,
    NU,
    NF,
    MP,
    NO,
    OM,
    PK,
    PW,
    PS,
    PA,
    PG,
    PY,
    PE,
    PH,
    PN,
    PL,
    PT,
    PR,
    QA,
    RE,
    RO,
    RU,
    RW,
    BL,
    SH,
    KN,
    LC,
    MF,
    PM,
    VC,
    WS,
    SM,
    ST,
    SA,
    SN,
    RS,
    SC,
    SL,
    SG,
    SX,
    SK,
    SI,
    SB,
    SO,
    ZA,
    GS,
    SS,
    ES,
    LK,
    SD,
    SR,
    SJ,
    SZ,
    SE,
    CH,
    SY,
    TW,
    TJ,
    TZ,
    TH,
    TL,
    TG,
    TK,
    TO,
    TT,
    TN,
    TR,
    TM,
    TC,
    TV,
    UG,
    UA,
    AE,
    GB,
    US,
    UM,
    UY,
    UZ,
    VU,
    VE,
    VN,
    VG,
    VI,
    WF,
    EH,
    YE,
    ZM,
    ZW,
    UnsupportedIso31661,
}

impl Into<&str> for CountryIso31661 {
    fn into(self) -> &'static str {
        match self {
            Self::AF => "Afghanistan",
            Self::AX => "Åland Islands",
            Self::AL => "Albania",
            Self::DZ => "Algeria",
            Self::AS => "American Samoa",
            Self::AD => "Andorra",
            Self::AO => "Angola",
            Self::AI => "Anguilla",
            Self::AQ => "Antarctica",
            Self::AG => "Antigua and Barbuda",
            Self::AR => "Argentina",
            Self::AM => "Armenia",
            Self::AW => "Aruba",
            Self::AU => "Australia",
            Self::AT => "Austria",
            Self::AZ => "Azerbaijan",
            Self::BS => "Bahamas",
            Self::BH => "Bahrain",
            Self::BD => "Bangladesh",
            Self::BB => "Barbados",
            Self::BY => "Belarus",
            Self::BE => "Belgium",
            Self::BZ => "Belize",
            Self::BJ => "Benin",
            Self::BM => "Bermuda",
            Self::BT => "Bhutan",
            Self::BO => "Bolivia, Plurinational State of",
            Self::BQ => "Bonaire, Sint Eustatius and Saba",
            Self::BA => "Bosnia and Herzegovina",
            Self::BW => "Botswana",
            Self::BV => "Bouvet Island",
            Self::BR => "Brazil",
            Self::IO => "British Indian Ocean Territory",
            Self::BN => "Brunei Darussalam",
            Self::BG => "Bulgaria",
            Self::BF => "Burkina Faso",
            Self::BI => "Burundi",
            Self::KH => "Cambodia",
            Self::CM => "Cameroon",
            Self::CA => "Canada",
            Self::CV => "Cape Verde",
            Self::KY => "Cayman Islands",
            Self::CF => "Central African Republic",
            Self::TD => "Chad",
            Self::CL => "Chile",
            Self::CN => "China",
            Self::CX => "Christmas Island",
            Self::CC => "Cocos (Keeling) Islands",
            Self::CO => "Colombia",
            Self::KM => "Comoros",
            Self::CG => "Congo",
            Self::CD => "Congo, the Democratic Republic of the",
            Self::CK => "Cook Islands",
            Self::CR => "Costa Rica",
            Self::CI => "Côte d'Ivoire",
            Self::HR => "Croatia",
            Self::CU => "Cuba",
            Self::CW => "Curaçao",
            Self::CY => "Cyprus",
            Self::CZ => "Czech Republic",
            Self::DK => "Denmark",
            Self::DJ => "Djibouti",
            Self::DM => "Dominica",
            Self::DO => "Dominican Republic",
            Self::EC => "Ecuador",
            Self::EG => "Egypt",
            Self::SV => "El Salvador",
            Self::GQ => "Equatorial Guinea",
            Self::ER => "Eritrea",
            Self::EE => "Estonia",
            Self::ET => "Ethiopia",
            Self::FK => "Falkland Islands (Malvinas)",
            Self::FO => "Faroe Islands",
            Self::FJ => "Fiji",
            Self::FI => "Finland",
            Self::FR => "France",
            Self::GF => "French Guiana",
            Self::PF => "French Polynesia",
            Self::TF => "French Southern Territories",
            Self::GA => "Gabon",
            Self::GM => "Gambia",
            Self::GE => "Georgia",
            Self::DE => "Germany",
            Self::GH => "Ghana",
            Self::GI => "Gibraltar",
            Self::GR => "Greece",
            Self::GL => "Greenland",
            Self::GD => "Grenada",
            Self::GP => "Guadeloupe",
            Self::GU => "Guam",
            Self::GT => "Guatemala",
            Self::GG => "Guernsey",
            Self::GN => "Guinea",
            Self::GW => "Guinea-Bissau",
            Self::GY => "Guyana",
            Self::HT => "Haiti",
            Self::HM => "Heard Island and McDonald Islands",
            Self::VA => "Holy See (Vatican City State)",
            Self::HN => "Honduras",
            Self::HK => "Hong Kong",
            Self::HU => "Hungary",
            Self::IS => "Iceland",
            Self::IN => "India",
            Self::ID => "Indonesia",
            Self::IR => "Iran, Islamic Republic of",
            Self::IQ => "Iraq",
            Self::IE => "Ireland",
            Self::IM => "Isle of Man",
            Self::IL => "Israel",
            Self::IT => "Italy",
            Self::JM => "Jamaica",
            Self::JP => "Japan",
            Self::JE => "Jersey",
            Self::JO => "Jordan",
            Self::KZ => "Kazakhstan",
            Self::KE => "Kenya",
            Self::KI => "Kiribati",
            Self::KP => "Korea, Democratic People's Republic of",
            Self::KR => "Korea, Republic of",
            Self::KW => "Kuwait",
            Self::KG => "Kyrgyzstan",
            Self::LA => "Lao People's Democratic Republic",
            Self::LV => "Latvia",
            Self::LB => "Lebanon",
            Self::LS => "Lesotho",
            Self::LR => "Liberia",
            Self::LY => "Libya",
            Self::LI => "Liechtenstein",
            Self::LT => "Lithuania",
            Self::LU => "Luxembourg",
            Self::MO => "Macao",
            Self::MK => "Macedonia, the Former Yugoslav Republic of",
            Self::MG => "Madagascar",
            Self::MW => "Malawi",
            Self::MY => "Malaysia",
            Self::MV => "Maldives",
            Self::ML => "Mali",
            Self::MT => "Malta",
            Self::MH => "Marshall Islands",
            Self::MQ => "Martinique",
            Self::MR => "Mauritania",
            Self::MU => "Mauritius",
            Self::YT => "Mayotte",
            Self::MX => "Mexico",
            Self::FM => "Micronesia, Federated States of",
            Self::MD => "Moldova, Republic of",
            Self::MC => "Monaco",
            Self::MN => "Mongolia",
            Self::ME => "Montenegro",
            Self::MS => "Montserrat",
            Self::MA => "Morocco",
            Self::MZ => "Mozambique",
            Self::MM => "Myanmar",
            Self::NA => "Namibia",
            Self::NR => "Nauru",
            Self::NP => "Nepal",
            Self::NL => "Netherlands",
            Self::NC => "New Caledonia",
            Self::NZ => "New Zealand",
            Self::NI => "Nicaragua",
            Self::NE => "Niger",
            Self::NG => "Nigeria",
            Self::NU => "Niue",
            Self::NF => "Norfolk Island",
            Self::MP => "Northern Mariana Islands",
            Self::NO => "Norway",
            Self::OM => "Oman",
            Self::PK => "Pakistan",
            Self::PW => "Palau",
            Self::PS => "Palestine, State of",
            Self::PA => "Panama",
            Self::PG => "Papua New Guinea",
            Self::PY => "Paraguay",
            Self::PE => "Peru",
            Self::PH => "Philippines",
            Self::PN => "Pitcairn",
            Self::PL => "Poland",
            Self::PT => "Portugal",
            Self::PR => "Puerto Rico",
            Self::QA => "Qatar",
            Self::RE => "Réunion",
            Self::RO => "Romania",
            Self::RU => "Russian Federation",
            Self::RW => "Rwanda",
            Self::BL => "Saint Barthélemy",
            Self::SH => "Saint Helena, Ascension and Tristan da Cunha",
            Self::KN => "Saint Kitts and Nevis",
            Self::LC => "Saint Lucia",
            Self::MF => "Saint Martin (French part)",
            Self::PM => "Saint Pierre and Miquelon",
            Self::VC => "Saint Vincent and the Grenadines",
            Self::WS => "Samoa",
            Self::SM => "San Marino",
            Self::ST => "Sao Tome and Principe",
            Self::SA => "Saudi Arabia",
            Self::SN => "Senegal",
            Self::RS => "Serbia",
            Self::SC => "Seychelles",
            Self::SL => "Sierra Leone",
            Self::SG => "Singapore",
            Self::SX => "Sint Maarten (Dutch part)",
            Self::SK => "Slovakia",
            Self::SI => "Slovenia",
            Self::SB => "Solomon Islands",
            Self::SO => "Somalia",
            Self::ZA => "South Africa",
            Self::GS => "South Georgia and the South Sandwich Islands",
            Self::SS => "South Sudan",
            Self::ES => "Spain",
            Self::LK => "Sri Lanka",
            Self::SD => "Sudan",
            Self::SR => "Suriname",
            Self::SJ => "Svalbard and Jan Mayen",
            Self::SZ => "Swaziland",
            Self::SE => "Sweden",
            Self::CH => "Switzerland",
            Self::SY => "Syrian Arab Republic",
            Self::TW => "Taiwan, Province of China",
            Self::TJ => "Tajikistan",
            Self::TZ => "Tanzania, United Republic of",
            Self::TH => "Thailand",
            Self::TL => "Timor-Leste",
            Self::TG => "Togo",
            Self::TK => "Tokelau",
            Self::TO => "Tonga",
            Self::TT => "Trinidad and Tobago",
            Self::TN => "Tunisia",
            Self::TR => "Turkey",
            Self::TM => "Turkmenistan",
            Self::TC => "Turks and Caicos Islands",
            Self::TV => "Tuvalu",
            Self::UG => "Uganda",
            Self::UA => "Ukraine",
            Self::AE => "United Arab Emirates",
            Self::GB => "United Kingdom",
            Self::US => "United States",
            Self::UM => "United States Minor Outlying Islands",
            Self::UY => "Uruguay",
            Self::UZ => "Uzbekistan",
            Self::VU => "Vanuatu",
            Self::VE => "Venezuela, Bolivarian Republic of",
            Self::VN => "Viet Nam",
            Self::VG => "Virgin Islands, British",
            Self::VI => "Virgin Islands, U.S.",
            Self::WF => "Wallis and Futuna",
            Self::EH => "Western Sahara",
            Self::YE => "Yemen",
            Self::ZM => "Zambia",
            Self::ZW => "Zimbabwe",
            Self::UnsupportedIso31661 => "InvalidCountry",
        }
    }
}

impl From<&str> for CountryIso31661 {
    fn from(value: &str) -> Self {
        match value {
            "Afghanistan" => Self::AF,
            "Åland Islands" => Self::AX,
            "Albania" => Self::AL,
            "Algeria" => Self::DZ,
            "American Samoa" => Self::AS,
            "Andorra" => Self::AD,
            "Angola" => Self::AO,
            "Anguilla" => Self::AI,
            "Antarctica" => Self::AQ,
            "Antigua and Barbuda" => Self::AG,
            "Argentina" => Self::AR,
            "Armenia" => Self::AM,
            "Aruba" => Self::AW,
            "Australia" => Self::AU,
            "Austria" => Self::AT,
            "Azerbaijan" => Self::AZ,
            "Bahamas" => Self::BS,
            "Bahrain" => Self::BH,
            "Bangladesh" => Self::BD,
            "Barbados" => Self::BB,
            "Belarus" => Self::BY,
            "Belgium" => Self::BE,
            "Belize" => Self::BZ,
            "Benin" => Self::BJ,
            "Bermuda" => Self::BM,
            "Bhutan" => Self::BT,
            "Bolivia, Plurinational State of" => Self::BO,
            "Bonaire, Sint Eustatius and Saba" => Self::BQ,
            "Bosnia and Herzegovina" => Self::BA,
            "Botswana" => Self::BW,
            "Bouvet Island" => Self::BV,
            "Brazil" => Self::BR,
            "British Indian Ocean Territory" => Self::IO,
            "Brunei Darussalam" => Self::BN,
            "Bulgaria" => Self::BG,
            "Burkina Faso" => Self::BF,
            "Burundi" => Self::BI,
            "Cambodia" => Self::KH,
            "Cameroon" => Self::CM,
            "Canada" => Self::CA,
            "Cape Verde" => Self::CV,
            "Cayman Islands" => Self::KY,
            "Central African Republic" => Self::CF,
            "Chad" => Self::TD,
            "Chile" => Self::CL,
            "China" => Self::CN,
            "Christmas Island" => Self::CX,
            "Cocos (Keeling) Islands" => Self::CC,
            "Colombia" => Self::CO,
            "Comoros" => Self::KM,
            "Congo" => Self::CG,
            "Congo, the Democratic Republic of the" => Self::CD,
            "Cook Islands" => Self::CK,
            "Costa Rica" => Self::CR,
            "Côte d'Ivoire" => Self::CI,
            "Croatia" => Self::HR,
            "Cuba" => Self::CU,
            "Curaçao" => Self::CW,
            "Cyprus" => Self::CY,
            "Czech Republic" => Self::CZ,
            "Denmark" => Self::DK,
            "Djibouti" => Self::DJ,
            "Dominica" => Self::DM,
            "Dominican Republic" => Self::DO,
            "Ecuador" => Self::EC,
            "Egypt" => Self::EG,
            "El Salvador" => Self::SV,
            "Equatorial Guinea" => Self::GQ,
            "Eritrea" => Self::ER,
            "Estonia" => Self::EE,
            "Ethiopia" => Self::ET,
            "Falkland Islands (Malvinas)" => Self::FK,
            "Faroe Islands" => Self::FO,
            "Fiji" => Self::FJ,
            "Finland" => Self::FI,
            "France" => Self::FR,
            "French Guiana" => Self::GF,
            "French Polynesia" => Self::PF,
            "French Southern Territories" => Self::TF,
            "Gabon" => Self::GA,
            "Gambia" => Self::GM,
            "Georgia" => Self::GE,
            "Germany" => Self::DE,
            "Ghana" => Self::GH,
            "Gibraltar" => Self::GI,
            "Greece" => Self::GR,
            "Greenland" => Self::GL,
            "Grenada" => Self::GD,
            "Guadeloupe" => Self::GP,
            "Guam" => Self::GU,
            "Guatemala" => Self::GT,
            "Guernsey" => Self::GG,
            "Guinea" => Self::GN,
            "Guinea-Bissau" => Self::GW,
            "Guyana" => Self::GY,
            "Haiti" => Self::HT,
            "Heard Island and McDonald Islands" => Self::HM,
            "Holy See (Vatican City State)" => Self::VA,
            "Honduras" => Self::HN,
            "Hong Kong" => Self::HK,
            "Hungary" => Self::HU,
            "Iceland" => Self::IS,
            "India" => Self::IN,
            "Indonesia" => Self::ID,
            "Iran, Islamic Republic of" => Self::IR,
            "Iraq" => Self::IQ,
            "Ireland" => Self::IE,
            "Isle of Man" => Self::IM,
            "Israel" => Self::IL,
            "Italy" => Self::IT,
            "Jamaica" => Self::JM,
            "Japan" => Self::JP,
            "Jersey" => Self::JE,
            "Jordan" => Self::JO,
            "Kazakhstan" => Self::KZ,
            "Kenya" => Self::KE,
            "Kiribati" => Self::KI,
            "Korea, Democratic People's Republic of" => Self::KP,
            "Korea, Republic of" => Self::KR,
            "Kuwait" => Self::KW,
            "Kyrgyzstan" => Self::KG,
            "Lao People's Democratic Republic" => Self::LA,
            "Latvia" => Self::LV,
            "Lebanon" => Self::LB,
            "Lesotho" => Self::LS,
            "Liberia" => Self::LR,
            "Libya" => Self::LY,
            "Liechtenstein" => Self::LI,
            "Lithuania" => Self::LT,
            "Luxembourg" => Self::LU,
            "Macao" => Self::MO,
            "Macedonia, the Former Yugoslav Republic of" => Self::MK,
            "Madagascar" => Self::MG,
            "Malawi" => Self::MW,
            "Malaysia" => Self::MY,
            "Maldives" => Self::MV,
            "Mali" => Self::ML,
            "Malta" => Self::MT,
            "Marshall Islands" => Self::MH,
            "Martinique" => Self::MQ,
            "Mauritania" => Self::MR,
            "Mauritius" => Self::MU,
            "Mayotte" => Self::YT,
            "Mexico" => Self::MX,
            "Micronesia, Federated States of" => Self::FM,
            "Moldova, Republic of" => Self::MD,
            "Monaco" => Self::MC,
            "Mongolia" => Self::MN,
            "Montenegro" => Self::ME,
            "Montserrat" => Self::MS,
            "Morocco" => Self::MA,
            "Mozambique" => Self::MZ,
            "Myanmar" => Self::MM,
            "Namibia" => Self::NA,
            "Nauru" => Self::NR,
            "Nepal" => Self::NP,
            "Netherlands" => Self::NL,
            "New Caledonia" => Self::NC,
            "New Zealand" => Self::NZ,
            "Nicaragua" => Self::NI,
            "Niger" => Self::NE,
            "Nigeria" => Self::NG,
            "Niue" => Self::NU,
            "Norfolk Island" => Self::NF,
            "Northern Mariana Islands" => Self::MP,
            "Norway" => Self::NO,
            "Oman" => Self::OM,
            "Pakistan" => Self::PK,
            "Palau" => Self::PW,
            "Palestine, State of" => Self::PS,
            "Panama" => Self::PA,
            "Papua New Guinea" => Self::PG,
            "Paraguay" => Self::PY,
            "Peru" => Self::PE,
            "Philippines" => Self::PH,
            "Pitcairn" => Self::PN,
            "Poland" => Self::PL,
            "Portugal" => Self::PT,
            "Puerto Rico" => Self::PR,
            "Qatar" => Self::QA,
            "Réunion" => Self::RE,
            "Romania" => Self::RO,
            "Russian Federation" => Self::RU,
            "Rwanda" => Self::RW,
            "Saint Barthélemy" => Self::BL,
            "Saint Helena, Ascension and Tristan da Cunha" => Self::SH,
            "Saint Kitts and Nevis" => Self::KN,
            "Saint Lucia" => Self::LC,
            "Saint Martin (French part)" => Self::MF,
            "Saint Pierre and Miquelon" => Self::PM,
            "Saint Vincent and the Grenadines" => Self::VC,
            "Samoa" => Self::WS,
            "San Marino" => Self::SM,
            "Sao Tome and Principe" => Self::ST,
            "Saudi Arabia" => Self::SA,
            "Senegal" => Self::SN,
            "Serbia" => Self::RS,
            "Seychelles" => Self::SC,
            "Sierra Leone" => Self::SL,
            "Singapore" => Self::SG,
            "Sint Maarten (Dutch part)" => Self::SX,
            "Slovakia" => Self::SK,
            "Slovenia" => Self::SI,
            "Solomon Islands" => Self::SB,
            "Somalia" => Self::SO,
            "South Africa" => Self::ZA,
            "South Georgia and the South Sandwich Islands" => Self::GS,
            "South Sudan" => Self::SS,
            "Spain" => Self::ES,
            "Sri Lanka" => Self::LK,
            "Sudan" => Self::SD,
            "Suriname" => Self::SR,
            "Svalbard and Jan Mayen" => Self::SJ,
            "Swaziland" => Self::SZ,
            "Sweden" => Self::SE,
            "Switzerland" => Self::CH,
            "Syrian Arab Republic" => Self::SY,
            "Taiwan, Province of China" => Self::TW,
            "Tajikistan" => Self::TJ,
            "Tanzania, United Republic of" => Self::TZ,
            "Thailand" => Self::TH,
            "Timor-Leste" => Self::TL,
            "Togo" => Self::TG,
            "Tokelau" => Self::TK,
            "Tonga" => Self::TO,
            "Trinidad and Tobago" => Self::TT,
            "Tunisia" => Self::TN,
            "Turkey" => Self::TR,
            "Turkmenistan" => Self::TM,
            "Turks and Caicos Islands" => Self::TC,
            "Tuvalu" => Self::TV,
            "Uganda" => Self::UG,
            "Ukraine" => Self::UA,
            "United Arab Emirates" => Self::AE,
            "United Kingdom" => Self::GB,
            "United States" => Self::US,
            "United States Minor Outlying Islands" => Self::UM,
            "Uruguay" => Self::UY,
            "Uzbekistan" => Self::UZ,
            "Vanuatu" => Self::VU,
            "Venezuela, Bolivarian Republic of" => Self::VE,
            "Viet Nam" => Self::VN,
            "Virgin Islands, British" => Self::VG,
            "Virgin Islands, U.S." => Self::VI,
            "Wallis and Futuna" => Self::WF,
            "Western Sahara" => Self::EH,
            "Yemen" => Self::YE,
            "Zambia" => Self::ZM,
            "Zimbabwe" => Self::ZW,
            _ => Self::UnsupportedIso31661,
        }
    }
}
