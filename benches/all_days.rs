use aoc_2022::day_one;
use aoc_2022::day_three;
use aoc_2022::day_two;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

// DAY 1
fn benchmark_day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 1");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_one::part_one(day_one::INPUT).unwrap();
            assert_eq!(result, 69_626);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_one::part_two(day_one::INPUT).unwrap();
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
            let result = day_two::part_one(day_two::INPUT).unwrap();
            assert_eq!(result, 11_150);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_two::part_two(day_two::INPUT).unwrap();
            assert_eq!(result, 8_295);
        })
    });
}

criterion_group!(day_02, benchmark_day_2);


// DAY 2
fn benchmark_day_3(c: &mut Criterion) {
    let mut group = c.benchmark_group("Day 3");

    group.bench_function("Part One", |b| {
        b.iter(|| {
            let result = day_three::part_one(day_three::INPUT).unwrap();
            assert_eq!(result, 7_889);
        })
    });

    group.bench_function("Part Two", |b| {
        b.iter(|| {
            let result = day_three::part_two(day_three::INPUT).unwrap();
            assert_eq!(result, 2_825);
        })
    });
}

criterion_group!(day_03, benchmark_day_3);

criterion_main!(day_01, day_02, day_03);
