//! Verifies file-property and container-content inspection.

use super::*;
use crate::meta::fields::{AudioCodec, VideoCodec, VideoProfile, VideoResolution};
use crate::probe::probe;
use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FILE: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    path: PathBuf,
}

impl Fixture {
    fn new(extension: &str, data: &[u8]) -> Self {
        let id = NEXT_FILE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "mediakit-file-inspector-{}-{id}.{extension}",
            std::process::id()
        ));
        fs::write(&path, data).unwrap();
        Self { path }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn inspect(extension: &str, data: &[u8]) -> FileInspector {
    let fixture = Fixture::new(extension, data);
    FileInspector::new(&fixture.path).analyze()
}

fn has_tag(inspector: &FileInspector, predicate: impl Fn(&Tag) -> bool) -> bool {
    inspector.tags().into_iter().any(predicate)
}

fn mp4_box(kind: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(&u32::try_from(payload.len() + 8).unwrap().to_be_bytes());
    output.extend_from_slice(kind);
    output.extend_from_slice(payload);
    output
}

fn mp4_header(timescale: u32, duration: u32) -> Vec<u8> {
    let mut data = vec![0; 20];
    data[12..16].copy_from_slice(&timescale.to_be_bytes());
    data[16..20].copy_from_slice(&duration.to_be_bytes());
    data
}

fn mp4_track(handler: &[u8; 4], codec: &[u8; 4], enabled: bool) -> Vec<u8> {
    let mut tkhd = vec![0; 28];
    tkhd[3] = u8::from(enabled);
    if handler == b"vide" {
        tkhd[20..24].copy_from_slice(&(1920_u32 << 16).to_be_bytes());
        tkhd[24..28].copy_from_slice(&(1080_u32 << 16).to_be_bytes());
    }
    let mut hdlr = vec![0; 12];
    hdlr[8..12].copy_from_slice(handler);
    let mut sample = if handler == b"vide" {
        let mut sample = vec![0; 78];
        sample[24..26].copy_from_slice(&1920_u16.to_be_bytes());
        sample[26..28].copy_from_slice(&1080_u16.to_be_bytes());
        let mut hvcc = vec![0; 18];
        hvcc[1] = 2;
        hvcc[17] = 2;
        sample.extend_from_slice(&mp4_box(b"hvcC", &hvcc));
        sample
    } else {
        let mut sample = vec![0; 28];
        sample[16..18].copy_from_slice(&6_u16.to_be_bytes());
        sample
    };
    sample = mp4_box(codec, &sample);
    let mut stsd = vec![0; 8];
    stsd[4..8].copy_from_slice(&1_u32.to_be_bytes());
    stsd.extend_from_slice(&sample);
    let mut stbl = mp4_box(b"stsd", &stsd);
    if handler == b"vide" {
        let mut stts = vec![0; 16];
        stts[4..8].copy_from_slice(&1_u32.to_be_bytes());
        stts[8..12].copy_from_slice(&300_u32.to_be_bytes());
        stts[12..16].copy_from_slice(&1000_u32.to_be_bytes());
        stbl.extend_from_slice(&mp4_box(b"stts", &stts));
    } else {
        let mut stsz = vec![0; 12];
        stsz[4..8].copy_from_slice(&1000_u32.to_be_bytes());
        stsz[8..12].copy_from_slice(&480_u32.to_be_bytes());
        stbl.extend_from_slice(&mp4_box(b"stsz", &stsz));
    }
    let stbl = mp4_box(b"stbl", &stbl);
    let minf = mp4_box(b"minf", &stbl);
    let mdhd = mp4_box(
        b"mdhd",
        &mp4_header(
            if handler == b"vide" { 30_000 } else { 48_000 },
            if handler == b"vide" { 300_000 } else { 480_000 },
        ),
    );
    let mut mdia = mdhd;
    mdia.extend_from_slice(&mp4_box(b"hdlr", &hdlr));
    mdia.extend_from_slice(&minf);
    let mut trak = mp4_box(b"tkhd", &tkhd);
    trak.extend_from_slice(&mp4_box(b"mdia", &mdia));
    mp4_box(b"trak", &trak)
}

fn mp4_fixture() -> Vec<u8> {
    let mut ftyp = b"isom\0\0\0\0mp42".to_vec();
    ftyp = mp4_box(b"ftyp", &ftyp);
    let mut moov = mp4_box(b"mvhd", &mp4_header(1000, 10_000));
    moov.extend_from_slice(&mp4_track(b"vide", b"avc1", false));
    moov.extend_from_slice(&mp4_track(b"vide", b"hvc1", true));
    moov.extend_from_slice(&mp4_track(b"soun", b"mp4a", false));
    moov.extend_from_slice(&mp4_track(b"soun", b"ec-3", true));
    ftyp.extend_from_slice(&mp4_box(b"moov", &moov));
    ftyp
}

fn ebml_size(size: usize) -> Vec<u8> {
    if size < 0x7F {
        vec![0x80 | size as u8]
    } else {
        (0x4000_u16 | u16::try_from(size).unwrap())
            .to_be_bytes()
            .to_vec()
    }
}

fn ebml(id: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut output = id.to_vec();
    output.extend_from_slice(&ebml_size(payload.len()));
    output.extend_from_slice(payload);
    output
}

fn ebml_uint(id: &[u8], value: u64) -> Vec<u8> {
    let bytes = value.to_be_bytes();
    let start = bytes.iter().position(|byte| *byte != 0).unwrap_or(7);
    ebml(id, &bytes[start..])
}

fn matroska_track(kind: u64, codec: &str, enabled: bool, default: bool) -> Vec<u8> {
    let mut track = ebml_uint(&[0x83], kind);
    track.extend_from_slice(&ebml_uint(&[0xB9], u64::from(enabled)));
    track.extend_from_slice(&ebml_uint(&[0x88], u64::from(default)));
    track.extend_from_slice(&ebml(&[0x86], codec.as_bytes()));
    if kind == 1 {
        track.extend_from_slice(&ebml_uint(&[0x23, 0xE3, 0x83], 33_333_333));
        let mut video = ebml_uint(&[0xB0], 1920);
        video.extend_from_slice(&ebml_uint(&[0xBA], 1080));
        video.extend_from_slice(&ebml_uint(&[0x9A], 2));
        track.extend_from_slice(&ebml(&[0xE0], &video));
    } else {
        track.extend_from_slice(&ebml(&[0xE1], &ebml_uint(&[0x9F], 6)));
    }
    ebml(&[0xAE], &track)
}

fn matroska_fixture() -> Vec<u8> {
    let header = ebml(&[0x1A, 0x45, 0xDF, 0xA3], &ebml(&[0x42, 0x82], b"webm"));
    let mut info = ebml_uint(&[0x2A, 0xD7, 0xB1], 1_000_000);
    info.extend_from_slice(&ebml(&[0x44, 0x89], &10_f64.to_be_bytes()));
    let info = ebml(&[0x15, 0x49, 0xA9, 0x66], &info);
    let mut tracks = matroska_track(1, "V_MPEG4/ISO/AVC", false, false);
    tracks.extend_from_slice(&matroska_track(1, "V_VP9", true, true));
    tracks.extend_from_slice(&matroska_track(2, "A_AAC", false, false));
    tracks.extend_from_slice(&matroska_track(2, "A_OPUS", true, true));
    let tracks = ebml(&[0x16, 0x54, 0xAE, 0x6B], &tracks);
    let mut segment = info;
    segment.extend_from_slice(&tracks);
    let mut output = header;
    output.extend_from_slice(&ebml(&[0x18, 0x53, 0x80, 0x67], &segment));
    output
}

fn riff_chunk(kind: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut output = kind.to_vec();
    output.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
    output.extend_from_slice(payload);
    if payload.len() & 1 != 0 {
        output.push(0);
    }
    output
}

fn avi_stream(kind: &[u8; 4], handler: &[u8; 4], format: &[u8], disabled: bool) -> Vec<u8> {
    let mut strh = vec![0; 36];
    strh[..4].copy_from_slice(kind);
    strh[4..8].copy_from_slice(handler);
    strh[8..12].copy_from_slice(&u32::from(disabled).to_le_bytes());
    strh[20..24].copy_from_slice(&1_u32.to_le_bytes());
    strh[24..28].copy_from_slice(&30_u32.to_le_bytes());
    strh[32..36].copy_from_slice(&300_u32.to_le_bytes());
    let mut list = b"strl".to_vec();
    list.extend_from_slice(&riff_chunk(b"strh", &strh));
    list.extend_from_slice(&riff_chunk(b"strf", format));
    riff_chunk(b"LIST", &list)
}

fn avi_fixture() -> Vec<u8> {
    let mut avih = vec![0; 40];
    avih[..4].copy_from_slice(&33_333_u32.to_le_bytes());
    avih[16..20].copy_from_slice(&300_u32.to_le_bytes());
    avih[32..36].copy_from_slice(&1920_u32.to_le_bytes());
    avih[36..40].copy_from_slice(&1080_u32.to_le_bytes());
    let mut video = vec![0; 40];
    video[..4].copy_from_slice(&40_u32.to_le_bytes());
    video[4..8].copy_from_slice(&1920_i32.to_le_bytes());
    video[8..12].copy_from_slice(&1080_i32.to_le_bytes());
    video[16..20].copy_from_slice(b"H264");
    let mut audio = vec![0; 16];
    audio[..2].copy_from_slice(&0x2000_u16.to_le_bytes());
    audio[2..4].copy_from_slice(&6_u16.to_le_bytes());
    audio[8..12].copy_from_slice(&48_000_u32.to_le_bytes());
    let mut hdrl = b"hdrl".to_vec();
    hdrl.extend_from_slice(&riff_chunk(b"avih", &avih));
    hdrl.extend_from_slice(&avi_stream(b"vids", b"XVID", &video, true));
    hdrl.extend_from_slice(&avi_stream(b"vids", b"H264", &video, false));
    hdrl.extend_from_slice(&avi_stream(b"auds", &[0; 4], &audio, true));
    hdrl.extend_from_slice(&avi_stream(b"auds", &[0; 4], &audio, false));
    let hdrl = riff_chunk(b"LIST", &hdrl);
    let mut output = b"RIFF".to_vec();
    output.extend_from_slice(&u32::try_from(hdrl.len() + 4).unwrap().to_le_bytes());
    output.extend_from_slice(b"AVI ");
    output.extend_from_slice(&hdrl);
    output
}

fn asf_object(guid: &[u8; 16], payload: &[u8]) -> Vec<u8> {
    let mut output = guid.to_vec();
    output.extend_from_slice(&u64::try_from(payload.len() + 24).unwrap().to_le_bytes());
    output.extend_from_slice(payload);
    output
}

fn asf_stream(stream_type: &[u8; 16], type_data: &[u8]) -> Vec<u8> {
    const STREAM_GUID: [u8; 16] = [
        0x91, 0x07, 0xDC, 0xB7, 0xB7, 0xA9, 0xCF, 0x11, 0x8E, 0xE6, 0, 0xC0, 0x0C, 0x20, 0x53, 0x65,
    ];
    let mut payload = vec![0; 54];
    payload[..16].copy_from_slice(stream_type);
    payload[40..44].copy_from_slice(&u32::try_from(type_data.len()).unwrap().to_le_bytes());
    payload.extend_from_slice(type_data);
    asf_object(&STREAM_GUID, &payload)
}

fn asf_fixture() -> Vec<u8> {
    const HEADER: [u8; 16] = [
        0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0, 0xAA, 0, 0x62, 0xCE, 0x6C,
    ];
    const FILE: [u8; 16] = [
        0xA1, 0xDC, 0xAB, 0x8C, 0x47, 0xA9, 0xCF, 0x11, 0x8E, 0xE4, 0, 0xC0, 0x0C, 0x20, 0x53, 0x65,
    ];
    const AUDIO: [u8; 16] = [
        0x40, 0x9E, 0x69, 0xF8, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
    ];
    const VIDEO: [u8; 16] = [
        0xC0, 0xEF, 0x19, 0xBC, 0x4D, 0x5B, 0xCF, 0x11, 0xA8, 0xFD, 0, 0x80, 0x5F, 0x5C, 0x44, 0x2B,
    ];
    let mut file = vec![0; 80];
    file[40..48].copy_from_slice(&100_000_000_u64.to_le_bytes());
    let file = asf_object(&FILE, &file);
    let mut audio = vec![0; 16];
    audio[..2].copy_from_slice(&0x2000_u16.to_le_bytes());
    audio[2..4].copy_from_slice(&6_u16.to_le_bytes());
    audio[8..12].copy_from_slice(&48_000_u32.to_le_bytes());
    let audio = asf_stream(&AUDIO, &audio);
    let audio2 = asf_stream(&AUDIO, &[0; 16]);
    let mut video = vec![0; 51];
    video[..4].copy_from_slice(&1920_u32.to_le_bytes());
    video[4..8].copy_from_slice(&1080_u32.to_le_bytes());
    video[9..11].copy_from_slice(&40_u16.to_le_bytes());
    video[11..15].copy_from_slice(&40_u32.to_le_bytes());
    video[15..19].copy_from_slice(&1920_u32.to_le_bytes());
    video[19..23].copy_from_slice(&1080_u32.to_le_bytes());
    video[27..31].copy_from_slice(b"WVC1");
    let mut video2 = video.clone();
    video2[27..31].copy_from_slice(b"XVID");
    let video = asf_stream(&VIDEO, &video);
    let video2 = asf_stream(&VIDEO, &video2);
    let size = 30 + file.len() + audio.len() + audio2.len() + video.len() + video2.len();
    let mut output = HEADER.to_vec();
    output.extend_from_slice(&u64::try_from(size).unwrap().to_le_bytes());
    output.extend_from_slice(&5_u32.to_le_bytes());
    output.extend_from_slice(&[1, 2]);
    output.extend_from_slice(&file);
    output.extend_from_slice(&audio);
    output.extend_from_slice(&audio2);
    output.extend_from_slice(&video);
    output.extend_from_slice(&video2);
    output
}

fn ts_packet(pid: u16, unit_start: bool, payload: &[u8]) -> Vec<u8> {
    let mut packet = vec![0xFF; 188];
    packet[0] = 0x47;
    packet[1] = ((pid >> 8) as u8 & 0x1F) | (u8::from(unit_start) * 0x40);
    packet[2] = pid as u8;
    packet[3] = 0x10;
    let mut offset = 4;
    if unit_start {
        packet[offset] = 0;
        offset += 1;
    }
    packet[offset..offset + payload.len()].copy_from_slice(payload);
    packet
}

fn ts_fixture() -> Vec<u8> {
    let pat = [
        0x00, 0xB0, 0x0D, 0, 1, 0xC1, 0, 0, 0, 1, 0xE1, 0, 0, 0, 0, 0,
    ];
    let pmt = [
        0x02, 0xB0, 0x21, 0, 1, 0xC1, 0, 0, 0xE1, 1, 0xF0, 0, 0x24, 0xE1, 1, 0xF0, 0, 0x1B, 0xE1,
        2, 0xF0, 0, 0x87, 0xE1, 3, 0xF0, 0, 0x0F, 0xE1, 4, 0xF0, 0, 0, 0, 0, 0,
    ];
    let mut output = ts_packet(0, true, &pat);
    output.extend_from_slice(&ts_packet(0x100, true, &pmt));
    for _ in 0..3 {
        output.extend_from_slice(&ts_packet(0x1FFF, false, &[]));
    }
    output
}

#[test]
fn public_probe_enumerates_streams_for_every_supported_container() {
    for (extension, data) in [
        ("mp4", mp4_fixture()),
        ("mkv", matroska_fixture()),
        ("avi", avi_fixture()),
        ("wmv", asf_fixture()),
        ("ts", ts_fixture()),
    ] {
        let fixture = Fixture::new(extension, &data);
        let info = probe(&fixture.path).unwrap();
        assert_eq!(info.video_streams.len(), 2, "{extension}");
        assert_eq!(info.audio_streams.len(), 2, "{extension}");
    }
}

#[test]
fn content_probe_overrides_misleading_extension_and_extracts_mp4_tracks() {
    let inspector = inspect("mkv", &mp4_fixture());
    assert!(has_tag(
        &inspector,
        |tag| matches!(tag, Tag::Container(value) if value == "mp4")
    ));
    assert!(has_tag(&inspector, |tag| matches!(
        tag,
        Tag::AudioCodec(AudioCodec::DolbyDigitalPlus)
    )));
    assert!(has_tag(&inspector, |tag| matches!(
        tag,
        Tag::VideoCodec(VideoCodec::H265)
    )));
    assert!(has_tag(&inspector, |tag| matches!(
        tag,
        Tag::VideoProfile(VideoProfile::Main10)
    )));
    assert!(has_tag(&inspector, |tag| matches!(
        tag,
        Tag::VideoResolution(VideoResolution::Hd1080p)
    )));
}

#[test]
fn parses_matroska_avi_transport_stream_and_asf_headers() {
    for (extension, data, container, video, audio) in [
        (
            "mkv",
            matroska_fixture(),
            "webm",
            VideoCodec::Vp9,
            AudioCodec::Opus,
        ),
        (
            "avi",
            avi_fixture(),
            "avi",
            VideoCodec::H264,
            AudioCodec::DolbyDigital,
        ),
        (
            "ts",
            ts_fixture(),
            "ts",
            VideoCodec::H265,
            AudioCodec::DolbyDigitalPlus,
        ),
        (
            "wmv",
            asf_fixture(),
            "wmv",
            VideoCodec::Vc1,
            AudioCodec::DolbyDigital,
        ),
    ] {
        let inspector = inspect(extension, &data);
        assert!(
            has_tag(
                &inspector,
                |tag| matches!(tag, Tag::Container(value) if value == container)
            ),
            "{extension}"
        );
        assert!(
            has_tag(
                &inspector,
                |tag| matches!(tag, Tag::VideoCodec(value) if *value == video)
            ),
            "{extension}"
        );
        assert!(
            has_tag(
                &inspector,
                |tag| matches!(tag, Tag::AudioCodec(value) if *value == audio)
            ),
            "{extension}"
        );
    }
}

#[test]
fn disabled_and_failed_probes_preserve_baseline_metadata() {
    let fixture = Fixture::new("mkv", &mp4_fixture());
    let disabled = FileInspector::new(&fixture.path)
        .with_content_inspection(false)
        .analyze();
    assert!(has_tag(
        &disabled,
        |tag| matches!(tag, Tag::FileFormat(value) if value.extension() == "mkv")
    ));
    assert!(!has_tag(&disabled, |tag| matches!(tag, Tag::AudioCodec(_))));
    assert!(has_tag(&disabled, |tag| matches!(tag, Tag::FileSize(_))));

    let malformed = inspect("mp4", b"\0\0\0\x10ftypisom");
    assert!(has_tag(
        &malformed,
        |tag| matches!(tag, Tag::FileFormat(value) if value.extension() == "mp4")
    ));
    assert!(!has_tag(&malformed, |tag| matches!(
        tag,
        Tag::VideoCodec(_)
    )));
}
