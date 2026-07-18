//! Defines country metadata values.

/// Country represents the production country of a media file.
#[derive(Debug, Clone, Copy)]
pub struct Country {
    /// The full human-readable name of the country.
    pub name: &'static str,
    /// The two-letter ISO 3166-1 alpha-2 country code.
    pub iso_3166_1_a2: &'static str,
    /// The three-letter ISO 3166-1 alpha-3 country code.
    pub iso_3166_1_a3: &'static str,
}

const COUNTRY_AFGHANISTAN: Country = Country {
    name: "Afghanistan",
    iso_3166_1_a2: "AF",
    iso_3166_1_a3: "AFG",
};
const COUNTRY_ALANDISLANDS: Country = Country {
    name: "Aland Islands",
    iso_3166_1_a2: "AX",
    iso_3166_1_a3: "ALA",
};
const COUNTRY_ALBANIA: Country = Country {
    name: "Albania",
    iso_3166_1_a2: "AL",
    iso_3166_1_a3: "ALB",
};
const COUNTRY_ALGERIA: Country = Country {
    name: "Algeria",
    iso_3166_1_a2: "DZ",
    iso_3166_1_a3: "DZA",
};
const COUNTRY_AMERICAN_SAMOA: Country = Country {
    name: "American Samoa",
    iso_3166_1_a2: "AS",
    iso_3166_1_a3: "ASM",
};
const COUNTRY_ANDORRA: Country = Country {
    name: "Andorra",
    iso_3166_1_a2: "AD",
    iso_3166_1_a3: "AND",
};
const COUNTRY_ANGOLA: Country = Country {
    name: "Angola",
    iso_3166_1_a2: "AO",
    iso_3166_1_a3: "AGO",
};
const COUNTRY_ANGUILLA: Country = Country {
    name: "Anguilla",
    iso_3166_1_a2: "AI",
    iso_3166_1_a3: "AIA",
};
const COUNTRY_ANTARCTICA: Country = Country {
    name: "Antarctica",
    iso_3166_1_a2: "AQ",
    iso_3166_1_a3: "ATA",
};
const COUNTRY_ANTIGUAANDBARBUDA: Country = Country {
    name: "Antigua and Barbuda",
    iso_3166_1_a2: "AG",
    iso_3166_1_a3: "ATG",
};
const COUNTRY_ARGENTINA: Country = Country {
    name: "Argentina",
    iso_3166_1_a2: "AR",
    iso_3166_1_a3: "ARG",
};
const COUNTRY_ARMENIA: Country = Country {
    name: "Armenia",
    iso_3166_1_a2: "AM",
    iso_3166_1_a3: "ARM",
};
const COUNTRY_ARUBA: Country = Country {
    name: "Aruba",
    iso_3166_1_a2: "AW",
    iso_3166_1_a3: "ABW",
};
const COUNTRY_AUSTRALIA: Country = Country {
    name: "Australia",
    iso_3166_1_a2: "AU",
    iso_3166_1_a3: "AUS",
};
const COUNTRY_AUSTRIA: Country = Country {
    name: "Austria",
    iso_3166_1_a2: "AT",
    iso_3166_1_a3: "AUT",
};
const COUNTRY_AZERBAIJAN: Country = Country {
    name: "Azerbaijan",
    iso_3166_1_a2: "AZ",
    iso_3166_1_a3: "AZE",
};
const COUNTRY_BAHAMAS: Country = Country {
    name: "Bahamas",
    iso_3166_1_a2: "BS",
    iso_3166_1_a3: "BHS",
};
const COUNTRY_BAHRAIN: Country = Country {
    name: "Bahrain",
    iso_3166_1_a2: "BH",
    iso_3166_1_a3: "BHR",
};
const COUNTRY_BANGLADESH: Country = Country {
    name: "Bangladesh",
    iso_3166_1_a2: "BD",
    iso_3166_1_a3: "BGD",
};
const COUNTRY_BARBADOS: Country = Country {
    name: "Barbados",
    iso_3166_1_a2: "BB",
    iso_3166_1_a3: "BRB",
};
const COUNTRY_BELARUS: Country = Country {
    name: "Belarus",
    iso_3166_1_a2: "BY",
    iso_3166_1_a3: "BLR",
};
const COUNTRY_BELGIUM: Country = Country {
    name: "Belgium",
    iso_3166_1_a2: "BE",
    iso_3166_1_a3: "BEL",
};
const COUNTRY_BELIZE: Country = Country {
    name: "Belize",
    iso_3166_1_a2: "BZ",
    iso_3166_1_a3: "BLZ",
};
const COUNTRY_BENIN: Country = Country {
    name: "Benin",
    iso_3166_1_a2: "BJ",
    iso_3166_1_a3: "BEN",
};
const COUNTRY_BERMUDA: Country = Country {
    name: "Bermuda",
    iso_3166_1_a2: "BM",
    iso_3166_1_a3: "BMU",
};
const COUNTRY_BHUTAN: Country = Country {
    name: "Bhutan",
    iso_3166_1_a2: "BT",
    iso_3166_1_a3: "BTN",
};
const COUNTRY_BOLIVIA: Country = Country {
    name: "Bolivia (Plurinational State of)",
    iso_3166_1_a2: "BO",
    iso_3166_1_a3: "BOL",
};
const COUNTRY_BONAIRE: Country = Country {
    name: "Bonaire, Sint Eustatius and Saba",
    iso_3166_1_a2: "BQ",
    iso_3166_1_a3: "BES",
};
const COUNTRY_BOSNIAANDHERZEGOVINA: Country = Country {
    name: "Bosnia and Herzegovina",
    iso_3166_1_a2: "BA",
    iso_3166_1_a3: "BIH",
};
const COUNTRY_BOTSWANA: Country = Country {
    name: "Botswana",
    iso_3166_1_a2: "BW",
    iso_3166_1_a3: "BWA",
};
const COUNTRY_BOUVETISLAND: Country = Country {
    name: "Bouvet Island",
    iso_3166_1_a2: "BV",
    iso_3166_1_a3: "BVT",
};
const COUNTRY_BRAZIL: Country = Country {
    name: "Brazil",
    iso_3166_1_a2: "BR",
    iso_3166_1_a3: "BRA",
};
const COUNTRY_BRITISHINDIANOCEANTERRITORY: Country = Country {
    name: "British Indian Ocean Territory",
    iso_3166_1_a2: "IO",
    iso_3166_1_a3: "IOT",
};
const COUNTRY_BRUNEIDARUSSALAM: Country = Country {
    name: "Brunei Darussalam",
    iso_3166_1_a2: "BN",
    iso_3166_1_a3: "BRN",
};
const COUNTRY_BULGARIA: Country = Country {
    name: "Bulgaria",
    iso_3166_1_a2: "BG",
    iso_3166_1_a3: "BGR",
};
const COUNTRY_BURKINAFASO: Country = Country {
    name: "Burkina Faso",
    iso_3166_1_a2: "BF",
    iso_3166_1_a3: "BFA",
};
const COUNTRY_BURUNDI: Country = Country {
    name: "Burundi",
    iso_3166_1_a2: "BI",
    iso_3166_1_a3: "BDI",
};
const COUNTRY_CABO_VERDE: Country = Country {
    name: "Cabo Verde",
    iso_3166_1_a2: "CV",
    iso_3166_1_a3: "CPV",
};
const COUNTRY_CAMBODIA: Country = Country {
    name: "Cambodia",
    iso_3166_1_a2: "KH",
    iso_3166_1_a3: "KHM",
};
const COUNTRY_CAMEROON: Country = Country {
    name: "Cameroon",
    iso_3166_1_a2: "CM",
    iso_3166_1_a3: "CMR",
};
const COUNTRY_CANADA: Country = Country {
    name: "Canada",
    iso_3166_1_a2: "CA",
    iso_3166_1_a3: "CAN",
};
const COUNTRY_CAYMANISLANDS: Country = Country {
    name: "Cayman Islands",
    iso_3166_1_a2: "KY",
    iso_3166_1_a3: "CYM",
};
const COUNTRY_CENTRALAFRICANREPUBLIC: Country = Country {
    name: "Central African Republic",
    iso_3166_1_a2: "CF",
    iso_3166_1_a3: "CAF",
};
const COUNTRY_CHAD: Country = Country {
    name: "Chad",
    iso_3166_1_a2: "TD",
    iso_3166_1_a3: "TCD",
};
const COUNTRY_CHILE: Country = Country {
    name: "Chile",
    iso_3166_1_a2: "CL",
    iso_3166_1_a3: "CHL",
};
const COUNTRY_CHINA: Country = Country {
    name: "China",
    iso_3166_1_a2: "CN",
    iso_3166_1_a3: "CHN",
};
const COUNTRY_CHRISTMASISLAND: Country = Country {
    name: "Christmas Island",
    iso_3166_1_a2: "CX",
    iso_3166_1_a3: "CXR",
};
const COUNTRY_COCOS_ISLANDS: Country = Country {
    name: "Cocos (Keeling) Islands",
    iso_3166_1_a2: "CC",
    iso_3166_1_a3: "CCK",
};
const COUNTRY_COLOMBIA: Country = Country {
    name: "Colombia",
    iso_3166_1_a2: "CO",
    iso_3166_1_a3: "COL",
};
const COUNTRY_COMOROS: Country = Country {
    name: "Comoros",
    iso_3166_1_a2: "KM",
    iso_3166_1_a3: "COM",
};
const COUNTRY_CONGO: Country = Country {
    name: "Congo",
    iso_3166_1_a2: "CG",
    iso_3166_1_a3: "COG",
};
const COUNTRY_COOKISLANDS: Country = Country {
    name: "Cook Islands",
    iso_3166_1_a2: "CK",
    iso_3166_1_a3: "COK",
};
const COUNTRY_COSTARICA: Country = Country {
    name: "Costa Rica",
    iso_3166_1_a2: "CR",
    iso_3166_1_a3: "CRI",
};
const COUNTRY_CÔTEDIVOIRE: Country = Country {
    name: "Côte d'Ivoire",
    iso_3166_1_a2: "CI",
    iso_3166_1_a3: "CIV",
};
const COUNTRY_CROATIA: Country = Country {
    name: "Croatia",
    iso_3166_1_a2: "HR",
    iso_3166_1_a3: "HRV",
};
const COUNTRY_CUBA: Country = Country {
    name: "Cuba",
    iso_3166_1_a2: "CU",
    iso_3166_1_a3: "CUB",
};
const COUNTRY_CURAÇAO: Country = Country {
    name: "Curaçao",
    iso_3166_1_a2: "CW",
    iso_3166_1_a3: "CUW",
};
const COUNTRY_CYPRUS: Country = Country {
    name: "Cyprus",
    iso_3166_1_a2: "CY",
    iso_3166_1_a3: "CYP",
};
const COUNTRY_CZECH: Country = Country {
    name: "Czechia",
    iso_3166_1_a2: "CZ",
    iso_3166_1_a3: "CZE",
};
const COUNTRY_DENMARK: Country = Country {
    name: "Denmark",
    iso_3166_1_a2: "DK",
    iso_3166_1_a3: "DNK",
};
const COUNTRY_DJIBOUTI: Country = Country {
    name: "Djibouti",
    iso_3166_1_a2: "DJ",
    iso_3166_1_a3: "DJI",
};
const COUNTRY_DOMINICA: Country = Country {
    name: "Dominica",
    iso_3166_1_a2: "DM",
    iso_3166_1_a3: "DMA",
};
const COUNTRY_DOMINICANREPUBLIC: Country = Country {
    name: "Dominican Republic",
    iso_3166_1_a2: "DO",
    iso_3166_1_a3: "DOM",
};
const COUNTRY_ECUADOR: Country = Country {
    name: "Ecuador",
    iso_3166_1_a2: "EC",
    iso_3166_1_a3: "ECU",
};
const COUNTRY_EGYPT: Country = Country {
    name: "Egypt",
    iso_3166_1_a2: "EG",
    iso_3166_1_a3: "EGY",
};
const COUNTRY_ELSALVADOR: Country = Country {
    name: "El Salvador",
    iso_3166_1_a2: "SV",
    iso_3166_1_a3: "SLV",
};
const COUNTRY_EQUATORIALGUINEA: Country = Country {
    name: "Equatorial Guinea",
    iso_3166_1_a2: "GQ",
    iso_3166_1_a3: "GNQ",
};
const COUNTRY_ERITREA: Country = Country {
    name: "Eritrea",
    iso_3166_1_a2: "ER",
    iso_3166_1_a3: "ERI",
};
const COUNTRY_ESTONIA: Country = Country {
    name: "Estonia",
    iso_3166_1_a2: "EE",
    iso_3166_1_a3: "EST",
};
const COUNTRY_ESWATINI: Country = Country {
    name: "Eswatini",
    iso_3166_1_a2: "SZ",
    iso_3166_1_a3: "SWZ",
};
const COUNTRY_ETHIOPIA: Country = Country {
    name: "Ethiopia",
    iso_3166_1_a2: "ET",
    iso_3166_1_a3: "ETH",
};
const COUNTRY_FALKLANDISLANDS: Country = Country {
    name: "Falkland Islands (Malvinas)",
    iso_3166_1_a2: "FK",
    iso_3166_1_a3: "FLK",
};
const COUNTRY_FAROEISLANDS: Country = Country {
    name: "Faroe Islands",
    iso_3166_1_a2: "FO",
    iso_3166_1_a3: "FRO",
};
const COUNTRY_FIJI: Country = Country {
    name: "Fiji",
    iso_3166_1_a2: "FJ",
    iso_3166_1_a3: "FJI",
};
const COUNTRY_FINLAND: Country = Country {
    name: "Finland",
    iso_3166_1_a2: "FI",
    iso_3166_1_a3: "FIN",
};
const COUNTRY_FRANCE: Country = Country {
    name: "France",
    iso_3166_1_a2: "FR",
    iso_3166_1_a3: "FRA",
};
const COUNTRY_FRENCHGUIANA: Country = Country {
    name: "French Guiana",
    iso_3166_1_a2: "GF",
    iso_3166_1_a3: "GUF",
};
const COUNTRY_FRENCHPOLYNESIA: Country = Country {
    name: "French Polynesia",
    iso_3166_1_a2: "PF",
    iso_3166_1_a3: "PYF",
};
const COUNTRY_FRENCHSOUTHERNTERRITORIES: Country = Country {
    name: "French Southern Territories",
    iso_3166_1_a2: "TF",
    iso_3166_1_a3: "ATF",
};
const COUNTRY_GABON: Country = Country {
    name: "Gabon",
    iso_3166_1_a2: "GA",
    iso_3166_1_a3: "GAB",
};
const COUNTRY_GAMBIA: Country = Country {
    name: "Gambia",
    iso_3166_1_a2: "GM",
    iso_3166_1_a3: "GMB",
};
const COUNTRY_GEORGIA: Country = Country {
    name: "Georgia",
    iso_3166_1_a2: "GE",
    iso_3166_1_a3: "GEO",
};
const COUNTRY_GERMANY: Country = Country {
    name: "Germany",
    iso_3166_1_a2: "DE",
    iso_3166_1_a3: "DEU",
};
const COUNTRY_GHANA: Country = Country {
    name: "Ghana",
    iso_3166_1_a2: "GH",
    iso_3166_1_a3: "GHA",
};
const COUNTRY_GIBRALTAR: Country = Country {
    name: "Gibraltar",
    iso_3166_1_a2: "GI",
    iso_3166_1_a3: "GIB",
};
const COUNTRY_GREECE: Country = Country {
    name: "Greece",
    iso_3166_1_a2: "GR",
    iso_3166_1_a3: "GRC",
};
const COUNTRY_GREENLAND: Country = Country {
    name: "Greenland",
    iso_3166_1_a2: "GL",
    iso_3166_1_a3: "GRL",
};
const COUNTRY_GRENADA: Country = Country {
    name: "Grenada",
    iso_3166_1_a2: "GD",
    iso_3166_1_a3: "GRD",
};
const COUNTRY_GUADELOUPE: Country = Country {
    name: "Guadeloupe",
    iso_3166_1_a2: "GP",
    iso_3166_1_a3: "GLP",
};
const COUNTRY_GUAM: Country = Country {
    name: "Guam",
    iso_3166_1_a2: "GU",
    iso_3166_1_a3: "GUM",
};
const COUNTRY_GUATEMALA: Country = Country {
    name: "Guatemala",
    iso_3166_1_a2: "GT",
    iso_3166_1_a3: "GTM",
};
const COUNTRY_GUERNSEY: Country = Country {
    name: "Guernsey",
    iso_3166_1_a2: "GG",
    iso_3166_1_a3: "GGY",
};
const COUNTRY_GUINEA: Country = Country {
    name: "Guinea",
    iso_3166_1_a2: "GN",
    iso_3166_1_a3: "GIN",
};
const COUNTRY_GUINEABISSAU: Country = Country {
    name: "Guinea-Bissau",
    iso_3166_1_a2: "GW",
    iso_3166_1_a3: "GNB",
};
const COUNTRY_GUYANA: Country = Country {
    name: "Guyana",
    iso_3166_1_a2: "GY",
    iso_3166_1_a3: "GUY",
};
const COUNTRY_HAITI: Country = Country {
    name: "Haiti",
    iso_3166_1_a2: "HT",
    iso_3166_1_a3: "HTI",
};
const COUNTRY_HEARDISLANDANDMCDONALDISLANDS: Country = Country {
    name: "Heard Island and McDonald Islands",
    iso_3166_1_a2: "HM",
    iso_3166_1_a3: "HMD",
};
const COUNTRY_HOLYSEE: Country = Country {
    name: "Holy See",
    iso_3166_1_a2: "VA",
    iso_3166_1_a3: "VAT",
};
const COUNTRY_HONDURAS: Country = Country {
    name: "Honduras",
    iso_3166_1_a2: "HN",
    iso_3166_1_a3: "HND",
};
const COUNTRY_HONGKONG: Country = Country {
    name: "Hong Kong",
    iso_3166_1_a2: "HK",
    iso_3166_1_a3: "HKG",
};
const COUNTRY_HUNGARY: Country = Country {
    name: "Hungary",
    iso_3166_1_a2: "HU",
    iso_3166_1_a3: "HUN",
};
const COUNTRY_ICELAND: Country = Country {
    name: "Iceland",
    iso_3166_1_a2: "IS",
    iso_3166_1_a3: "ISL",
};
const COUNTRY_INDIA: Country = Country {
    name: "India",
    iso_3166_1_a2: "IN",
    iso_3166_1_a3: "IND",
};
const COUNTRY_INDONESIA: Country = Country {
    name: "Indonesia",
    iso_3166_1_a2: "ID",
    iso_3166_1_a3: "IDN",
};
const COUNTRY_IRAN: Country = Country {
    name: "Iran (Islamic Republic of)",
    iso_3166_1_a2: "IR",
    iso_3166_1_a3: "IRN",
};
const COUNTRY_IRAQ: Country = Country {
    name: "Iraq",
    iso_3166_1_a2: "IQ",
    iso_3166_1_a3: "IRQ",
};
const COUNTRY_IRELAND: Country = Country {
    name: "Ireland",
    iso_3166_1_a2: "IE",
    iso_3166_1_a3: "IRL",
};
const COUNTRY_ISLEOFMAN: Country = Country {
    name: "Isle of Man",
    iso_3166_1_a2: "IM",
    iso_3166_1_a3: "IMN",
};
const COUNTRY_ISRAEL: Country = Country {
    name: "Israel",
    iso_3166_1_a2: "IL",
    iso_3166_1_a3: "ISR",
};
const COUNTRY_ITALY: Country = Country {
    name: "Italy",
    iso_3166_1_a2: "IT",
    iso_3166_1_a3: "ITA",
};
const COUNTRY_JAMAICA: Country = Country {
    name: "Jamaica",
    iso_3166_1_a2: "JM",
    iso_3166_1_a3: "JAM",
};
const COUNTRY_JAPAN: Country = Country {
    name: "Japan",
    iso_3166_1_a2: "JP",
    iso_3166_1_a3: "JPN",
};
const COUNTRY_JERSEY: Country = Country {
    name: "Jersey",
    iso_3166_1_a2: "JE",
    iso_3166_1_a3: "JEY",
};
const COUNTRY_JORDAN: Country = Country {
    name: "Jordan",
    iso_3166_1_a2: "JO",
    iso_3166_1_a3: "JOR",
};
const COUNTRY_KAZAKHSTAN: Country = Country {
    name: "Kazakhstan",
    iso_3166_1_a2: "KZ",
    iso_3166_1_a3: "KAZ",
};
const COUNTRY_KENYA: Country = Country {
    name: "Kenya",
    iso_3166_1_a2: "KE",
    iso_3166_1_a3: "KEN",
};
const COUNTRY_KIRIBATI: Country = Country {
    name: "Kiribati",
    iso_3166_1_a2: "KI",
    iso_3166_1_a3: "KIR",
};
const COUNTRY_KOREA: Country = Country {
    name: "Korea (Democratic People's Republic of)",
    iso_3166_1_a2: "KP",
    iso_3166_1_a3: "PRK",
};
const COUNTRY_KOREAREPUBLICOF: Country = Country {
    name: "Korea, Republic of",
    iso_3166_1_a2: "KR",
    iso_3166_1_a3: "KOR",
};
const COUNTRY_KUWAIT: Country = Country {
    name: "Kuwait",
    iso_3166_1_a2: "KW",
    iso_3166_1_a3: "KWT",
};
const COUNTRY_KYRGYZSTAN: Country = Country {
    name: "Kyrgyzstan",
    iso_3166_1_a2: "KG",
    iso_3166_1_a3: "KGZ",
};
const COUNTRY_LAOPEOPLESDEMOCRATICREPUBLIC: Country = Country {
    name: "Lao People's Democratic Republic",
    iso_3166_1_a2: "LA",
    iso_3166_1_a3: "LAO",
};
const COUNTRY_LATVIA: Country = Country {
    name: "Latvia",
    iso_3166_1_a2: "LV",
    iso_3166_1_a3: "LVA",
};
const COUNTRY_LEBANON: Country = Country {
    name: "Lebanon",
    iso_3166_1_a2: "LB",
    iso_3166_1_a3: "LBN",
};
const COUNTRY_LESOTHO: Country = Country {
    name: "Lesotho",
    iso_3166_1_a2: "LS",
    iso_3166_1_a3: "LSO",
};
const COUNTRY_LIBERIA: Country = Country {
    name: "Liberia",
    iso_3166_1_a2: "LR",
    iso_3166_1_a3: "LBR",
};
const COUNTRY_LIBYA: Country = Country {
    name: "Libya",
    iso_3166_1_a2: "LY",
    iso_3166_1_a3: "LBY",
};
const COUNTRY_LIECHTENSTEIN: Country = Country {
    name: "Liechtenstein",
    iso_3166_1_a2: "LI",
    iso_3166_1_a3: "LIE",
};
const COUNTRY_LITHUANIA: Country = Country {
    name: "Lithuania",
    iso_3166_1_a2: "LT",
    iso_3166_1_a3: "LTU",
};
const COUNTRY_LUXEMBOURG: Country = Country {
    name: "Luxembourg",
    iso_3166_1_a2: "LU",
    iso_3166_1_a3: "LUX",
};
const COUNTRY_MACAO: Country = Country {
    name: "Macao",
    iso_3166_1_a2: "MO",
    iso_3166_1_a3: "MAC",
};
const COUNTRY_MADAGASCAR: Country = Country {
    name: "Madagascar",
    iso_3166_1_a2: "MG",
    iso_3166_1_a3: "MDG",
};
const COUNTRY_MALAWI: Country = Country {
    name: "Malawi",
    iso_3166_1_a2: "MW",
    iso_3166_1_a3: "MWI",
};
const COUNTRY_MALAYSIA: Country = Country {
    name: "Malaysia",
    iso_3166_1_a2: "MY",
    iso_3166_1_a3: "MYS",
};
const COUNTRY_MALDIVES: Country = Country {
    name: "Maldives",
    iso_3166_1_a2: "MV",
    iso_3166_1_a3: "MDV",
};
const COUNTRY_MALI: Country = Country {
    name: "Mali",
    iso_3166_1_a2: "ML",
    iso_3166_1_a3: "MLI",
};
const COUNTRY_MALTA: Country = Country {
    name: "Malta",
    iso_3166_1_a2: "MT",
    iso_3166_1_a3: "MLT",
};
const COUNTRY_MARSHALLISLANDS: Country = Country {
    name: "Marshall Islands",
    iso_3166_1_a2: "MH",
    iso_3166_1_a3: "MHL",
};
const COUNTRY_MARTINIQUE: Country = Country {
    name: "Martinique",
    iso_3166_1_a2: "MQ",
    iso_3166_1_a3: "MTQ",
};
const COUNTRY_MAURITANIA: Country = Country {
    name: "Mauritania",
    iso_3166_1_a2: "MR",
    iso_3166_1_a3: "MRT",
};
const COUNTRY_MAURITIUS: Country = Country {
    name: "Mauritius",
    iso_3166_1_a2: "MU",
    iso_3166_1_a3: "MUS",
};
const COUNTRY_MAYOTTE: Country = Country {
    name: "Mayotte",
    iso_3166_1_a2: "YT",
    iso_3166_1_a3: "MYT",
};
const COUNTRY_MEXICO: Country = Country {
    name: "Mexico",
    iso_3166_1_a2: "MX",
    iso_3166_1_a3: "MEX",
};
const COUNTRY_MICRONESIA: Country = Country {
    name: "Micronesia (Federated States of)",
    iso_3166_1_a2: "FM",
    iso_3166_1_a3: "FSM",
};
const COUNTRY_MOLDOVAREPUBLICOF: Country = Country {
    name: "Moldova, Republic of",
    iso_3166_1_a2: "MD",
    iso_3166_1_a3: "MDA",
};
const COUNTRY_MONACO: Country = Country {
    name: "Monaco",
    iso_3166_1_a2: "MC",
    iso_3166_1_a3: "MCO",
};
const COUNTRY_MONGOLIA: Country = Country {
    name: "Mongolia",
    iso_3166_1_a2: "MN",
    iso_3166_1_a3: "MNG",
};
const COUNTRY_MONTENEGRO: Country = Country {
    name: "Montenegro",
    iso_3166_1_a2: "ME",
    iso_3166_1_a3: "MNE",
};
const COUNTRY_MONTSERRAT: Country = Country {
    name: "Montserrat",
    iso_3166_1_a2: "MS",
    iso_3166_1_a3: "MSR",
};
const COUNTRY_MOROCCO: Country = Country {
    name: "Morocco",
    iso_3166_1_a2: "MA",
    iso_3166_1_a3: "MAR",
};
const COUNTRY_MOZAMBIQUE: Country = Country {
    name: "Mozambique",
    iso_3166_1_a2: "MZ",
    iso_3166_1_a3: "MOZ",
};
const COUNTRY_MYANMAR: Country = Country {
    name: "Myanmar",
    iso_3166_1_a2: "MM",
    iso_3166_1_a3: "MMR",
};
const COUNTRY_NAMIBIA: Country = Country {
    name: "Namibia",
    iso_3166_1_a2: "NA",
    iso_3166_1_a3: "NAM",
};
const COUNTRY_NAURU: Country = Country {
    name: "Nauru",
    iso_3166_1_a2: "NR",
    iso_3166_1_a3: "NRU",
};
const COUNTRY_NEPAL: Country = Country {
    name: "Nepal",
    iso_3166_1_a2: "NP",
    iso_3166_1_a3: "NPL",
};
const COUNTRY_NETHERLANDS: Country = Country {
    name: "Netherlands",
    iso_3166_1_a2: "NL",
    iso_3166_1_a3: "NLD",
};
const COUNTRY_NEWCALEDONIA: Country = Country {
    name: "New Caledonia",
    iso_3166_1_a2: "NC",
    iso_3166_1_a3: "NCL",
};
const COUNTRY_NEWZEALAND: Country = Country {
    name: "New Zealand",
    iso_3166_1_a2: "NZ",
    iso_3166_1_a3: "NZL",
};
const COUNTRY_NICARAGUA: Country = Country {
    name: "Nicaragua",
    iso_3166_1_a2: "NI",
    iso_3166_1_a3: "NIC",
};
const COUNTRY_NIGER: Country = Country {
    name: "Niger",
    iso_3166_1_a2: "NE",
    iso_3166_1_a3: "NER",
};
const COUNTRY_NIGERIA: Country = Country {
    name: "Nigeria",
    iso_3166_1_a2: "NG",
    iso_3166_1_a3: "NGA",
};
const COUNTRY_NIUE: Country = Country {
    name: "Niue",
    iso_3166_1_a2: "NU",
    iso_3166_1_a3: "NIU",
};
const COUNTRY_NORFOLKISLAND: Country = Country {
    name: "Norfolk Island",
    iso_3166_1_a2: "NF",
    iso_3166_1_a3: "NFK",
};
const COUNTRY_NORTHMACEDONIA: Country = Country {
    name: "North Macedonia",
    iso_3166_1_a2: "MK",
    iso_3166_1_a3: "MKD",
};
const COUNTRY_NORTHERNMARIANAISLANDS: Country = Country {
    name: "Northern Mariana Islands",
    iso_3166_1_a2: "MP",
    iso_3166_1_a3: "MNP",
};
const COUNTRY_NORWAY: Country = Country {
    name: "Norway",
    iso_3166_1_a2: "NO",
    iso_3166_1_a3: "NOR",
};
const COUNTRY_OMAN: Country = Country {
    name: "Oman",
    iso_3166_1_a2: "OM",
    iso_3166_1_a3: "OMN",
};
const COUNTRY_PAKISTAN: Country = Country {
    name: "Pakistan",
    iso_3166_1_a2: "PK",
    iso_3166_1_a3: "PAK",
};
const COUNTRY_PALAU: Country = Country {
    name: "Palau",
    iso_3166_1_a2: "PW",
    iso_3166_1_a3: "PLW",
};
const COUNTRY_PALESTINESTATEOF: Country = Country {
    name: "Palestine, State of",
    iso_3166_1_a2: "PS",
    iso_3166_1_a3: "PSE",
};
const COUNTRY_PANAMA: Country = Country {
    name: "Panama",
    iso_3166_1_a2: "PA",
    iso_3166_1_a3: "PAN",
};
const COUNTRY_PAPUANEWGUINEA: Country = Country {
    name: "Papua New Guinea",
    iso_3166_1_a2: "PG",
    iso_3166_1_a3: "PNG",
};
const COUNTRY_PARAGUAY: Country = Country {
    name: "Paraguay",
    iso_3166_1_a2: "PY",
    iso_3166_1_a3: "PRY",
};
const COUNTRY_PERU: Country = Country {
    name: "Peru",
    iso_3166_1_a2: "PE",
    iso_3166_1_a3: "PER",
};
const COUNTRY_PHILIPPINES: Country = Country {
    name: "Philippines",
    iso_3166_1_a2: "PH",
    iso_3166_1_a3: "PHL",
};
const COUNTRY_PITCAIRN: Country = Country {
    name: "Pitcairn",
    iso_3166_1_a2: "PN",
    iso_3166_1_a3: "PCN",
};
const COUNTRY_POLAND: Country = Country {
    name: "Poland",
    iso_3166_1_a2: "PL",
    iso_3166_1_a3: "POL",
};
const COUNTRY_PORTUGAL: Country = Country {
    name: "Portugal",
    iso_3166_1_a2: "PT",
    iso_3166_1_a3: "PRT",
};
const COUNTRY_PUERTORICO: Country = Country {
    name: "Puerto Rico",
    iso_3166_1_a2: "PR",
    iso_3166_1_a3: "PRI",
};
const COUNTRY_QATAR: Country = Country {
    name: "Qatar",
    iso_3166_1_a2: "QA",
    iso_3166_1_a3: "QAT",
};
const COUNTRY_RÉUNION: Country = Country {
    name: "Réunion",
    iso_3166_1_a2: "RE",
    iso_3166_1_a3: "REU",
};
const COUNTRY_ROMANIA: Country = Country {
    name: "Romania",
    iso_3166_1_a2: "RO",
    iso_3166_1_a3: "ROU",
};
const COUNTRY_RUSSIANFEDERATION: Country = Country {
    name: "Russian Federation",
    iso_3166_1_a2: "RU",
    iso_3166_1_a3: "RUS",
};
const COUNTRY_RWANDA: Country = Country {
    name: "Rwanda",
    iso_3166_1_a2: "RW",
    iso_3166_1_a3: "RWA",
};
const COUNTRY_SAINTBARTHÉLEMY: Country = Country {
    name: "Saint Barthélemy",
    iso_3166_1_a2: "BL",
    iso_3166_1_a3: "BLM",
};
const COUNTRY_SAINTHELENAASCENSIONANDTRISTANDACUNHA: Country = Country {
    name: "Saint Helena, Ascension and Tristan da Cunha",
    iso_3166_1_a2: "SH",
    iso_3166_1_a3: "SHN",
};
const COUNTRY_SAINTKITTSANDNEVIS: Country = Country {
    name: "Saint Kitts and Nevis",
    iso_3166_1_a2: "KN",
    iso_3166_1_a3: "KNA",
};
const COUNTRY_SAINTLUCIA: Country = Country {
    name: "Saint Lucia",
    iso_3166_1_a2: "LC",
    iso_3166_1_a3: "LCA",
};
const COUNTRY_SAINTMARTIN: Country = Country {
    name: "Saint Martin (French part)",
    iso_3166_1_a2: "MF",
    iso_3166_1_a3: "MAF",
};
const COUNTRY_SAINTPIERREANDMIQUELON: Country = Country {
    name: "Saint Pierre and Miquelon",
    iso_3166_1_a2: "PM",
    iso_3166_1_a3: "SPM",
};
const COUNTRY_SAINTVINCENTANDTHEGRENADINES: Country = Country {
    name: "Saint Vincent and the Grenadines",
    iso_3166_1_a2: "VC",
    iso_3166_1_a3: "VCT",
};
const COUNTRY_SAMOA: Country = Country {
    name: "Samoa",
    iso_3166_1_a2: "WS",
    iso_3166_1_a3: "WSM",
};
const COUNTRY_SANMARINO: Country = Country {
    name: "San Marino",
    iso_3166_1_a2: "SM",
    iso_3166_1_a3: "SMR",
};
const COUNTRY_SAOTOMEANDPRINCIPE: Country = Country {
    name: "Sao Tome and Principe",
    iso_3166_1_a2: "ST",
    iso_3166_1_a3: "STP",
};
const COUNTRY_SAUDIARABIA: Country = Country {
    name: "Saudi Arabia",
    iso_3166_1_a2: "SA",
    iso_3166_1_a3: "SAU",
};
const COUNTRY_SENEGAL: Country = Country {
    name: "Senegal",
    iso_3166_1_a2: "SN",
    iso_3166_1_a3: "SEN",
};
const COUNTRY_SERBIA: Country = Country {
    name: "Serbia",
    iso_3166_1_a2: "RS",
    iso_3166_1_a3: "SRB",
};
const COUNTRY_SEYCHELLES: Country = Country {
    name: "Seychelles",
    iso_3166_1_a2: "SC",
    iso_3166_1_a3: "SYC",
};
const COUNTRY_SIERRALEONE: Country = Country {
    name: "Sierra Leone",
    iso_3166_1_a2: "SL",
    iso_3166_1_a3: "SLE",
};
const COUNTRY_SINGAPORE: Country = Country {
    name: "Singapore",
    iso_3166_1_a2: "SG",
    iso_3166_1_a3: "SGP",
};
const COUNTRY_SINTMAARTEN: Country = Country {
    name: "Sint Maarten (Dutch part)",
    iso_3166_1_a2: "SX",
    iso_3166_1_a3: "SXM",
};
const COUNTRY_SLOVAKIA: Country = Country {
    name: "Slovakia",
    iso_3166_1_a2: "SK",
    iso_3166_1_a3: "SVK",
};
const COUNTRY_SLOVENIA: Country = Country {
    name: "Slovenia",
    iso_3166_1_a2: "SI",
    iso_3166_1_a3: "SVN",
};
const COUNTRY_SOLOMONISLANDS: Country = Country {
    name: "Solomon Islands",
    iso_3166_1_a2: "SB",
    iso_3166_1_a3: "SLB",
};
const COUNTRY_SOMALIA: Country = Country {
    name: "Somalia",
    iso_3166_1_a2: "SO",
    iso_3166_1_a3: "SOM",
};
const COUNTRY_SOUTHAFRICA: Country = Country {
    name: "South Africa",
    iso_3166_1_a2: "ZA",
    iso_3166_1_a3: "ZAF",
};
const COUNTRY_SOUTHGEORGIAANDTHESOUTHSANDWICHISLANDS: Country = Country {
    name: "South Georgia and the South Sandwich Islands",
    iso_3166_1_a2: "GS",
    iso_3166_1_a3: "SGS",
};
const COUNTRY_SOUTHSUDAN: Country = Country {
    name: "South Sudan",
    iso_3166_1_a2: "SS",
    iso_3166_1_a3: "SSD",
};
const COUNTRY_SPAIN: Country = Country {
    name: "Spain",
    iso_3166_1_a2: "ES",
    iso_3166_1_a3: "ESP",
};
const COUNTRY_SRILANKA: Country = Country {
    name: "Sri Lanka",
    iso_3166_1_a2: "LK",
    iso_3166_1_a3: "LKA",
};
const COUNTRY_SUDAN: Country = Country {
    name: "Sudan",
    iso_3166_1_a2: "SD",
    iso_3166_1_a3: "SDN",
};
const COUNTRY_SURINAME: Country = Country {
    name: "Suriname",
    iso_3166_1_a2: "SR",
    iso_3166_1_a3: "SUR",
};
const COUNTRY_SVALBARDANDJANMAYEN: Country = Country {
    name: "Svalbard and Jan Mayen",
    iso_3166_1_a2: "SJ",
    iso_3166_1_a3: "SJM",
};
const COUNTRY_SWEDEN: Country = Country {
    name: "Sweden",
    iso_3166_1_a2: "SE",
    iso_3166_1_a3: "SWE",
};
const COUNTRY_SWITZERLAND: Country = Country {
    name: "Switzerland",
    iso_3166_1_a2: "CH",
    iso_3166_1_a3: "CHE",
};
const COUNTRY_SYRIANARABREPUBLIC: Country = Country {
    name: "Syrian Arab Republic",
    iso_3166_1_a2: "SY",
    iso_3166_1_a3: "SYR",
};
const COUNTRY_TAIWANPROVINCEOFCHINA: Country = Country {
    name: "Taiwan, Province of China",
    iso_3166_1_a2: "TW",
    iso_3166_1_a3: "TWN",
};
const COUNTRY_TAJIKISTAN: Country = Country {
    name: "Tajikistan",
    iso_3166_1_a2: "TJ",
    iso_3166_1_a3: "TJK",
};
const COUNTRY_TANZANIAUNITEDREPUBLICOF: Country = Country {
    name: "Tanzania, United Republic of",
    iso_3166_1_a2: "TZ",
    iso_3166_1_a3: "TZA",
};
const COUNTRY_THAILAND: Country = Country {
    name: "Thailand",
    iso_3166_1_a2: "TH",
    iso_3166_1_a3: "THA",
};
const COUNTRY_TIMORLESTE: Country = Country {
    name: "Timor-Leste",
    iso_3166_1_a2: "TL",
    iso_3166_1_a3: "TLS",
};
const COUNTRY_TOGO: Country = Country {
    name: "Togo",
    iso_3166_1_a2: "TG",
    iso_3166_1_a3: "TGO",
};
const COUNTRY_TOKELAU: Country = Country {
    name: "Tokelau",
    iso_3166_1_a2: "TK",
    iso_3166_1_a3: "TKL",
};
const COUNTRY_TONGA: Country = Country {
    name: "Tonga",
    iso_3166_1_a2: "TO",
    iso_3166_1_a3: "TON",
};
const COUNTRY_TRINIDADANDTOBAGO: Country = Country {
    name: "Trinidad and Tobago",
    iso_3166_1_a2: "TT",
    iso_3166_1_a3: "TTO",
};
const COUNTRY_TUNISIA: Country = Country {
    name: "Tunisia",
    iso_3166_1_a2: "TN",
    iso_3166_1_a3: "TUN",
};
const COUNTRY_TURKEY: Country = Country {
    name: "Turkey",
    iso_3166_1_a2: "TR",
    iso_3166_1_a3: "TUR",
};
const COUNTRY_TURKMENISTAN: Country = Country {
    name: "Turkmenistan",
    iso_3166_1_a2: "TM",
    iso_3166_1_a3: "TKM",
};
const COUNTRY_TURKSANDCAICOSISLANDS: Country = Country {
    name: "Turks and Caicos Islands",
    iso_3166_1_a2: "TC",
    iso_3166_1_a3: "TCA",
};
const COUNTRY_TUVALU: Country = Country {
    name: "Tuvalu",
    iso_3166_1_a2: "TV",
    iso_3166_1_a3: "TUV",
};
const COUNTRY_UGANDA: Country = Country {
    name: "Uganda",
    iso_3166_1_a2: "UG",
    iso_3166_1_a3: "UGA",
};
const COUNTRY_UKRAINE: Country = Country {
    name: "Ukraine",
    iso_3166_1_a2: "UA",
    iso_3166_1_a3: "UKR",
};
const COUNTRY_UNITEDARABEMIRATES: Country = Country {
    name: "United Arab Emirates",
    iso_3166_1_a2: "AE",
    iso_3166_1_a3: "ARE",
};
const COUNTRY_UNITEDKINGDOMOFGREATBRITAINANDNORTHERNIRELAND: Country = Country {
    name: "United Kingdom of Great Britain and Northern Ireland",
    iso_3166_1_a2: "GB",
    iso_3166_1_a3: "GBR",
};
const COUNTRY_UNITEDSTATESOFAMERICA: Country = Country {
    name: "United States of America",
    iso_3166_1_a2: "US",
    iso_3166_1_a3: "USA",
};
const COUNTRY_UNITEDSTATESMINOROUTLYINGISLANDS: Country = Country {
    name: "United States Minor Outlying Islands",
    iso_3166_1_a2: "UM",
    iso_3166_1_a3: "UMI",
};
const COUNTRY_URUGUAY: Country = Country {
    name: "Uruguay",
    iso_3166_1_a2: "UY",
    iso_3166_1_a3: "URY",
};
const COUNTRY_UZBEKISTAN: Country = Country {
    name: "Uzbekistan",
    iso_3166_1_a2: "UZ",
    iso_3166_1_a3: "UZB",
};
const COUNTRY_VANUATU: Country = Country {
    name: "Vanuatu",
    iso_3166_1_a2: "VU",
    iso_3166_1_a3: "VUT",
};
const COUNTRY_VENEZUELA: Country = Country {
    name: "Venezuela (Bolivarian Republic of)",
    iso_3166_1_a2: "VE",
    iso_3166_1_a3: "VEN",
};
const COUNTRY_VIETNAM: Country = Country {
    name: "Viet Nam",
    iso_3166_1_a2: "VN",
    iso_3166_1_a3: "VNM",
};
const COUNTRY_BRITISH_VIRGIN_ISLANDS: Country = Country {
    name: "Virgin Islands (British)",
    iso_3166_1_a2: "VG",
    iso_3166_1_a3: "VGB",
};
const COUNTRY_US_VIRGIN_ISLANDS: Country = Country {
    name: "Virgin Islands (U.S.)",
    iso_3166_1_a2: "VI",
    iso_3166_1_a3: "VIR",
};
const COUNTRY_WALLISANDFUTUNA: Country = Country {
    name: "Wallis and Futuna",
    iso_3166_1_a2: "WF",
    iso_3166_1_a3: "WLF",
};
const COUNTRY_WESTERNSAHARA: Country = Country {
    name: "Western Sahara",
    iso_3166_1_a2: "EH",
    iso_3166_1_a3: "ESH",
};
const COUNTRY_YEMEN: Country = Country {
    name: "Yemen",
    iso_3166_1_a2: "YE",
    iso_3166_1_a3: "YEM",
};
const COUNTRY_ZAMBIA: Country = Country {
    name: "Zambia",
    iso_3166_1_a2: "ZM",
    iso_3166_1_a3: "ZMB",
};
const COUNTRY_ZIMBABWE: Country = Country {
    name: "Zimbabwe",
    iso_3166_1_a2: "ZW",
    iso_3166_1_a3: "ZWE",
};

const COUNTRY_ALL_COUNT: usize = 248;

/// A list of all countries known and supported by the library.
pub const COUNTRY_ALL: [&Country; COUNTRY_ALL_COUNT] = [
    &COUNTRY_AFGHANISTAN,
    &COUNTRY_ALANDISLANDS,
    &COUNTRY_ALBANIA,
    &COUNTRY_ALGERIA,
    &COUNTRY_AMERICAN_SAMOA,
    &COUNTRY_ANDORRA,
    &COUNTRY_ANGOLA,
    &COUNTRY_ANGUILLA,
    &COUNTRY_ANTARCTICA,
    &COUNTRY_ANTIGUAANDBARBUDA,
    &COUNTRY_ARGENTINA,
    &COUNTRY_ARMENIA,
    &COUNTRY_ARUBA,
    &COUNTRY_AUSTRALIA,
    &COUNTRY_AUSTRIA,
    &COUNTRY_AZERBAIJAN,
    &COUNTRY_BAHAMAS,
    &COUNTRY_BAHRAIN,
    &COUNTRY_BANGLADESH,
    &COUNTRY_BARBADOS,
    &COUNTRY_BELARUS,
    &COUNTRY_BELGIUM,
    &COUNTRY_BELIZE,
    &COUNTRY_BENIN,
    &COUNTRY_BERMUDA,
    &COUNTRY_BHUTAN,
    &COUNTRY_BOLIVIA,
    &COUNTRY_BONAIRE,
    &COUNTRY_BOSNIAANDHERZEGOVINA,
    &COUNTRY_BOTSWANA,
    &COUNTRY_BOUVETISLAND,
    &COUNTRY_BRAZIL,
    &COUNTRY_BRITISHINDIANOCEANTERRITORY,
    &COUNTRY_BRUNEIDARUSSALAM,
    &COUNTRY_BULGARIA,
    &COUNTRY_BURKINAFASO,
    &COUNTRY_BURUNDI,
    &COUNTRY_CABO_VERDE,
    &COUNTRY_CAMBODIA,
    &COUNTRY_CAMEROON,
    &COUNTRY_CANADA,
    &COUNTRY_CAYMANISLANDS,
    &COUNTRY_CENTRALAFRICANREPUBLIC,
    &COUNTRY_CHAD,
    &COUNTRY_CHILE,
    &COUNTRY_CHINA,
    &COUNTRY_CHRISTMASISLAND,
    &COUNTRY_COCOS_ISLANDS,
    &COUNTRY_COLOMBIA,
    &COUNTRY_COMOROS,
    &COUNTRY_CONGO,
    &COUNTRY_COOKISLANDS,
    &COUNTRY_COSTARICA,
    &COUNTRY_CÔTEDIVOIRE,
    &COUNTRY_CROATIA,
    &COUNTRY_CUBA,
    &COUNTRY_CURAÇAO,
    &COUNTRY_CYPRUS,
    &COUNTRY_CZECH,
    &COUNTRY_DENMARK,
    &COUNTRY_DJIBOUTI,
    &COUNTRY_DOMINICA,
    &COUNTRY_DOMINICANREPUBLIC,
    &COUNTRY_ECUADOR,
    &COUNTRY_EGYPT,
    &COUNTRY_ELSALVADOR,
    &COUNTRY_EQUATORIALGUINEA,
    &COUNTRY_ERITREA,
    &COUNTRY_ESTONIA,
    &COUNTRY_ESWATINI,
    &COUNTRY_ETHIOPIA,
    &COUNTRY_FALKLANDISLANDS,
    &COUNTRY_FAROEISLANDS,
    &COUNTRY_FIJI,
    &COUNTRY_FINLAND,
    &COUNTRY_FRANCE,
    &COUNTRY_FRENCHGUIANA,
    &COUNTRY_FRENCHPOLYNESIA,
    &COUNTRY_FRENCHSOUTHERNTERRITORIES,
    &COUNTRY_GABON,
    &COUNTRY_GAMBIA,
    &COUNTRY_GEORGIA,
    &COUNTRY_GERMANY,
    &COUNTRY_GHANA,
    &COUNTRY_GIBRALTAR,
    &COUNTRY_GREECE,
    &COUNTRY_GREENLAND,
    &COUNTRY_GRENADA,
    &COUNTRY_GUADELOUPE,
    &COUNTRY_GUAM,
    &COUNTRY_GUATEMALA,
    &COUNTRY_GUERNSEY,
    &COUNTRY_GUINEA,
    &COUNTRY_GUINEABISSAU,
    &COUNTRY_GUYANA,
    &COUNTRY_HAITI,
    &COUNTRY_HEARDISLANDANDMCDONALDISLANDS,
    &COUNTRY_HOLYSEE,
    &COUNTRY_HONDURAS,
    &COUNTRY_HONGKONG,
    &COUNTRY_HUNGARY,
    &COUNTRY_ICELAND,
    &COUNTRY_INDIA,
    &COUNTRY_INDONESIA,
    &COUNTRY_IRAN,
    &COUNTRY_IRAQ,
    &COUNTRY_IRELAND,
    &COUNTRY_ISLEOFMAN,
    &COUNTRY_ISRAEL,
    &COUNTRY_ITALY,
    &COUNTRY_JAMAICA,
    &COUNTRY_JAPAN,
    &COUNTRY_JERSEY,
    &COUNTRY_JORDAN,
    &COUNTRY_KAZAKHSTAN,
    &COUNTRY_KENYA,
    &COUNTRY_KIRIBATI,
    &COUNTRY_KOREA,
    &COUNTRY_KOREAREPUBLICOF,
    &COUNTRY_KUWAIT,
    &COUNTRY_KYRGYZSTAN,
    &COUNTRY_LAOPEOPLESDEMOCRATICREPUBLIC,
    &COUNTRY_LATVIA,
    &COUNTRY_LEBANON,
    &COUNTRY_LESOTHO,
    &COUNTRY_LIBERIA,
    &COUNTRY_LIBYA,
    &COUNTRY_LIECHTENSTEIN,
    &COUNTRY_LITHUANIA,
    &COUNTRY_LUXEMBOURG,
    &COUNTRY_MACAO,
    &COUNTRY_MADAGASCAR,
    &COUNTRY_MALAWI,
    &COUNTRY_MALAYSIA,
    &COUNTRY_MALDIVES,
    &COUNTRY_MALI,
    &COUNTRY_MALTA,
    &COUNTRY_MARSHALLISLANDS,
    &COUNTRY_MARTINIQUE,
    &COUNTRY_MAURITANIA,
    &COUNTRY_MAURITIUS,
    &COUNTRY_MAYOTTE,
    &COUNTRY_MEXICO,
    &COUNTRY_MICRONESIA,
    &COUNTRY_MOLDOVAREPUBLICOF,
    &COUNTRY_MONACO,
    &COUNTRY_MONGOLIA,
    &COUNTRY_MONTENEGRO,
    &COUNTRY_MONTSERRAT,
    &COUNTRY_MOROCCO,
    &COUNTRY_MOZAMBIQUE,
    &COUNTRY_MYANMAR,
    &COUNTRY_NAMIBIA,
    &COUNTRY_NAURU,
    &COUNTRY_NEPAL,
    &COUNTRY_NETHERLANDS,
    &COUNTRY_NEWCALEDONIA,
    &COUNTRY_NEWZEALAND,
    &COUNTRY_NICARAGUA,
    &COUNTRY_NIGER,
    &COUNTRY_NIGERIA,
    &COUNTRY_NIUE,
    &COUNTRY_NORFOLKISLAND,
    &COUNTRY_NORTHMACEDONIA,
    &COUNTRY_NORTHERNMARIANAISLANDS,
    &COUNTRY_NORWAY,
    &COUNTRY_OMAN,
    &COUNTRY_PAKISTAN,
    &COUNTRY_PALAU,
    &COUNTRY_PALESTINESTATEOF,
    &COUNTRY_PANAMA,
    &COUNTRY_PAPUANEWGUINEA,
    &COUNTRY_PARAGUAY,
    &COUNTRY_PERU,
    &COUNTRY_PHILIPPINES,
    &COUNTRY_PITCAIRN,
    &COUNTRY_POLAND,
    &COUNTRY_PORTUGAL,
    &COUNTRY_PUERTORICO,
    &COUNTRY_QATAR,
    &COUNTRY_RÉUNION,
    &COUNTRY_ROMANIA,
    &COUNTRY_RUSSIANFEDERATION,
    &COUNTRY_RWANDA,
    &COUNTRY_SAINTBARTHÉLEMY,
    &COUNTRY_SAINTHELENAASCENSIONANDTRISTANDACUNHA,
    &COUNTRY_SAINTKITTSANDNEVIS,
    &COUNTRY_SAINTLUCIA,
    &COUNTRY_SAINTMARTIN,
    &COUNTRY_SAINTPIERREANDMIQUELON,
    &COUNTRY_SAINTVINCENTANDTHEGRENADINES,
    &COUNTRY_SAMOA,
    &COUNTRY_SANMARINO,
    &COUNTRY_SAOTOMEANDPRINCIPE,
    &COUNTRY_SAUDIARABIA,
    &COUNTRY_SENEGAL,
    &COUNTRY_SERBIA,
    &COUNTRY_SEYCHELLES,
    &COUNTRY_SIERRALEONE,
    &COUNTRY_SINGAPORE,
    &COUNTRY_SINTMAARTEN,
    &COUNTRY_SLOVAKIA,
    &COUNTRY_SLOVENIA,
    &COUNTRY_SOLOMONISLANDS,
    &COUNTRY_SOMALIA,
    &COUNTRY_SOUTHAFRICA,
    &COUNTRY_SOUTHGEORGIAANDTHESOUTHSANDWICHISLANDS,
    &COUNTRY_SOUTHSUDAN,
    &COUNTRY_SPAIN,
    &COUNTRY_SRILANKA,
    &COUNTRY_SUDAN,
    &COUNTRY_SURINAME,
    &COUNTRY_SVALBARDANDJANMAYEN,
    &COUNTRY_SWEDEN,
    &COUNTRY_SWITZERLAND,
    &COUNTRY_SYRIANARABREPUBLIC,
    &COUNTRY_TAIWANPROVINCEOFCHINA,
    &COUNTRY_TAJIKISTAN,
    &COUNTRY_TANZANIAUNITEDREPUBLICOF,
    &COUNTRY_THAILAND,
    &COUNTRY_TIMORLESTE,
    &COUNTRY_TOGO,
    &COUNTRY_TOKELAU,
    &COUNTRY_TONGA,
    &COUNTRY_TRINIDADANDTOBAGO,
    &COUNTRY_TUNISIA,
    &COUNTRY_TURKEY,
    &COUNTRY_TURKMENISTAN,
    &COUNTRY_TURKSANDCAICOSISLANDS,
    &COUNTRY_TUVALU,
    &COUNTRY_UGANDA,
    &COUNTRY_UKRAINE,
    &COUNTRY_UNITEDARABEMIRATES,
    &COUNTRY_UNITEDKINGDOMOFGREATBRITAINANDNORTHERNIRELAND,
    &COUNTRY_UNITEDSTATESOFAMERICA,
    &COUNTRY_UNITEDSTATESMINOROUTLYINGISLANDS,
    &COUNTRY_URUGUAY,
    &COUNTRY_UZBEKISTAN,
    &COUNTRY_VANUATU,
    &COUNTRY_VENEZUELA,
    &COUNTRY_VIETNAM,
    &COUNTRY_BRITISH_VIRGIN_ISLANDS,
    &COUNTRY_US_VIRGIN_ISLANDS,
    &COUNTRY_WALLISANDFUTUNA,
    &COUNTRY_WESTERNSAHARA,
    &COUNTRY_YEMEN,
    &COUNTRY_ZAMBIA,
    &COUNTRY_ZIMBABWE,
];

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.iso_3166_1_a2 == other.iso_3166_1_a2
    }
}

impl std::hash::Hash for Country {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iso_3166_1_a2.hash(state);
    }
}

impl Country {
    /// Finds a country by its two-letter ISO 3166-1 alpha-2 code.
    pub fn from_iso_3166_1_a2(iso_3166_1_a2: &str) -> Option<&'static Self> {
        COUNTRY_ALL
            .iter()
            .find(|c| c.iso_3166_1_a2 == iso_3166_1_a2)
            .copied()
    }

    /// Finds a country by its three-letter ISO 3166-1 alpha-3 code.
    pub fn from_iso_3166_1_a3(iso_3166_1_a3: &str) -> Option<&'static Self> {
        COUNTRY_ALL
            .iter()
            .find(|c| c.iso_3166_1_a3 == iso_3166_1_a3)
            .copied()
    }

    /// Finds a country by its full name.
    pub fn from_name(name: &str) -> Option<&'static Self> {
        COUNTRY_ALL
            .into_iter()
            .find(|country: &&Self| country.name.to_lowercase() == name.to_lowercase())
    }

    fn detect_from_id(s: &str) -> Option<&'static Self> {
        Self::from_iso_3166_1_a2(s)
            .or_else(|| Self::from_iso_3166_1_a3(s))
            .or_else(|| Self::from_name(s))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::detect_from_id(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("Unknown country: {s}")))
            .copied()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.iso_3166_1_a2.serialize(serializer)
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.iso_3166_1_a2)
    }
}
