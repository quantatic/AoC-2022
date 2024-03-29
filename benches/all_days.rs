use aoc_2022::day_1;
use aoc_2022::day_10;
use aoc_2022::day_11;
use aoc_2022::day_2;
use aoc_2022::day_3;
use aoc_2022::day_4;
use aoc_2022::day_5;
use aoc_2022::day_6;
use aoc_2022::day_7;
use aoc_2022::day_8;
use aoc_2022::day_9;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::{black_box, Criterion};

// DAY 1
fn benchmark_day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 1");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_1::part_one(black_box(day_1::INPUT)).unwrap();
            assert_eq!(result, 69_626);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_1::part_two(day_1::INPUT).unwrap();
            assert_eq!(result, 206_780);
        })
    });
}

criterion_group!(day_01, benchmark_day_1);

// DAY 2
fn benchmark_day_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 2");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_2::part_one(black_box(day_2::INPUT)).unwrap();
            assert_eq!(result, 11_150);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_2::part_two(black_box(day_2::INPUT)).unwrap();
            assert_eq!(result, 8_295);
        })
    });
}

criterion_group!(day_02, benchmark_day_2);

// DAY 3
fn benchmark_day_3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 3");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_3::part_one(black_box(day_3::INPUT)).unwrap();
            assert_eq!(result, 7_889);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_3::part_two(black_box(day_3::INPUT)).unwrap();
            assert_eq!(result, 2_825);
        })
    });
}

criterion_group!(day_03, benchmark_day_3);

// DAY 4
fn benchmark_day_4(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 4");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_4::part_one(black_box(day_4::INPUT)).unwrap();
            assert_eq!(result, 576);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_4::part_two(black_box(day_4::INPUT)).unwrap();
            assert_eq!(result, 905);
        })
    });
}

criterion_group!(day_04, benchmark_day_4);

// DAY 5
fn benchmark_day_5(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 5");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_5::part_one(black_box(day_5::INPUT)).unwrap();
            assert_eq!(result, "BWNCQRMDB");
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_5::part_two(black_box(day_5::INPUT)).unwrap();
            assert_eq!(result, "NHWZCBNBF");
        })
    });
}

criterion_group!(day_05, benchmark_day_5);

// DAY 6
fn benchmark_day_6(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 6");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_6::part_one(black_box(day_6::INPUT)).unwrap();
            assert_eq!(result, 1_658);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_6::part_two(black_box(day_6::INPUT)).unwrap();
            assert_eq!(result, 2_260);
        })
    });
}

criterion_group!(day_06, benchmark_day_6);

// DAY 7
fn benchmark_day_7(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 7");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_7::part_one(black_box(day_7::INPUT)).unwrap();
            assert_eq!(result, 1_723_892);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_7::part_two(black_box(day_7::INPUT)).unwrap();
            assert_eq!(result, 8_474_158);
        })
    });
}

criterion_group!(day_07, benchmark_day_7);

// DAY 8
fn benchmark_day_8(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 8");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_8::part_one(black_box(day_8::INPUT)).unwrap();
            assert_eq!(result, 1_669);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_8::part_two(black_box(day_8::INPUT)).unwrap();
            assert_eq!(result, 331_344);
        })
    });
}

criterion_group!(day_08, benchmark_day_8);

// DAY 9
fn benchmark_day_9(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 9");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_9::part_one(black_box(day_9::INPUT)).unwrap();
            assert_eq!(result, 5_878);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_9::part_two(black_box(day_9::INPUT)).unwrap();
            assert_eq!(result, 2_405);
        })
    });
}

criterion_group!(day_09, benchmark_day_9);

// DAY 10
fn benchmark_day_10(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 10");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_10::part_one(black_box(day_10::INPUT)).unwrap();
            assert_eq!(result, 16_060);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            const EXPECTED_RESULT: [[bool; day_10::CRT_WIDTH]; day_10::CRT_HEIGHT] = [
                [
                    true, true, true, false, false, false, true, true, false, false, false, true,
                    true, false, false, true, true, true, true, false, true, false, false, true,
                    false, true, false, false, false, false, true, false, false, true, false, true,
                    true, true, true, false,
                ],
                [
                    false, false, false, true, false, true, false, false, true, false, true, false,
                    false, true, false, true, false, false, false, false, true, false, true, false,
                    false, true, false, false, false, false, true, false, false, true, false, true,
                    false, false, false, false,
                ],
                [
                    true, true, true, false, false, true, false, false, true, false, true, false,
                    false, false, false, true, true, true, false, false, true, true, false, false,
                    false, true, false, false, false, false, true, true, true, true, false, true,
                    true, true, false, false,
                ],
                [
                    false, false, false, true, false, true, true, true, true, false, true, false,
                    false, false, false, true, false, false, false, false, true, false, true,
                    false, false, true, false, false, false, false, true, false, false, true,
                    false, true, false, false, false, false,
                ],
                [
                    true, false, false, true, false, true, false, false, true, false, true, false,
                    false, true, false, true, false, false, false, false, true, false, true, false,
                    false, true, false, false, false, false, true, false, false, true, false, true,
                    false, false, false, false,
                ],
                [
                    true, true, true, false, false, true, false, false, true, false, false, true,
                    true, false, false, true, true, true, true, false, true, false, false, true,
                    false, true, true, true, true, false, true, false, false, true, false, true,
                    false, false, false, false,
                ],
            ];
            let result = day_10::part_two(black_box(day_10::INPUT)).unwrap();
            assert_eq!(result, EXPECTED_RESULT);
        })
    });
}

criterion_group!(day_10, benchmark_day_10);

// DAY 11
fn benchmark_day_11(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 11");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_11::part_one(black_box(day_11::INPUT)).unwrap();
            assert_eq!(result, 90_294);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_11::part_two(black_box(day_11::INPUT)).unwrap();
            assert_eq!(result, 18_170_818_354);
        })
    });
}

criterion_group!(day_11, benchmark_day_11);

criterion_main!(
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10, day_11
);
