//! Benchmarks filename tokenization and metadata analysis.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use mediakit::inspect::{FilenameInspector, Inspector};

const FILENAMES: &[(&str, &str)] = &[
    (
        "simple_movie",
        "Blade.Runner.2049.2017.2160p.UHD.BluRay.x265-FLAME.mkv",
    ),
    (
        "feature_rich_movie",
        "Oppenheimer.2023.2160p.UHD.BluRay.DTS-HD.MA.5.1.HEVC.Main10-ARCHiViST.mkv",
    ),
    (
        "television_episode",
        "The.Wire.S01E01.The.Target.1080p.BluRay.x264-FLAME.mkv",
    ),
    (
        "multi_episode_television",
        "Chernobyl.S01E01E02.1.23.45.1080p.BluRay.x264-ARCHiViST.mkv",
    ),
    (
        "air_date_television",
        "Dateline.NBC.2024.05.10.1080p.WEB.H264-FLAME.mkv",
    ),
    ("subtitle_language", "Ted.Lasso.S01E02.en.forced.srt"),
    (
        "unicode_title",
        "Shōgun.S01E01.Anjin.2160p.WEB.H265-FLAME.mkv",
    ),
];

fn benchmark_tokenize(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("tokenize");

    for &(label, filename) in FILENAMES {
        group.throughput(Throughput::Bytes(filename.len() as u64));
        group.bench_function(BenchmarkId::from_parameter(label), |bencher| {
            bencher.iter(|| black_box(FilenameInspector::new(black_box(filename))));
        });
    }

    group.finish();
}

fn benchmark_analyze(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("analyze");
    group.sample_size(60);

    for &(label, filename) in FILENAMES {
        group.throughput(Throughput::Bytes(filename.len() as u64));
        group.bench_function(BenchmarkId::from_parameter(label), |bencher| {
            bencher.iter(|| black_box(FilenameInspector::new(black_box(filename)).analyze()));
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_tokenize, benchmark_analyze);
criterion_main!(benches);
