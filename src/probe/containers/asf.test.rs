//! Verifies bounded ASF detection and probing.

use super::*;

#[test]
fn file_properties_subtract_preroll() {
    let mut data = vec![0; 80];
    data[40..48].copy_from_slice(&120_000_000_u64.to_le_bytes());
    data[56..64].copy_from_slice(&2_000_u64.to_le_bytes());
    let mut media = MediaInfo::new(MediaFormat::Wmv);
    parse_file_properties(&data, &mut media).unwrap();
    assert_eq!(media.duration, Some(Duration::from_secs(10)));
}
