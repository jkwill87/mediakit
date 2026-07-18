//! Verifies the curated public metadata-field facade.

use mediakit::meta::fields::{COUNTRY_ALL, Country};

#[test]
fn country_catalog_and_lookups_are_public() {
    fn assert_country_type(_: &Country) {}

    let canada = Country::from_iso_3166_1_a2("CA").expect("Canada by alpha-2 code");
    assert_country_type(canada);
    assert_eq!(canada.iso_3166_1_a3, "CAN");
    assert_eq!(Country::from_iso_3166_1_a3("CAN"), Some(canada));
    assert_eq!(Country::from_name("Canada"), Some(canada));
    assert!(COUNTRY_ALL.contains(&canada));
}
