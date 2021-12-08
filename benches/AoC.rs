use aoc_2021::advent_of_code::AdventOfCodeInput;
use aoc_2021::solutions::{day_five, day_four, day_one, day_seven, day_six, day_three, day_two};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn load_inp(day: u32) -> AdventOfCodeInput {
    AdventOfCodeInput::get_input(day)
}

fn bench_day_one(c: &mut Criterion) {
    let aoc_input = load_inp(1);

    let depths: Vec<_> = aoc_input
        .inp
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    c.bench_function("d1a", |b| b.iter(|| day_one::part_one(black_box(&depths))));
    c.bench_function("d1b", |b| b.iter(|| day_one::part_two(black_box(&depths))));
    c.bench_function("d1c", |b| {
        b.iter(|| day_one::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_two(c: &mut Criterion) {
    let aoc_input = load_inp(2);

    let inp = day_two::parse(aoc_input.clone().inp);

    c.bench_function("d2a", |b| b.iter(|| day_two::part_one(black_box(&inp))));
    c.bench_function("d2b", |b| b.iter(|| day_two::part_two(black_box(&inp))));
    c.bench_function("d2c", |b| {
        b.iter(|| day_two::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_three(c: &mut Criterion) {
    use day_three::BitArray;

    let aoc_input = load_inp(3);
    let codes: Vec<BitArray> = aoc_input
        .inp
        .lines()
        .map(|x| BitArray {
            arr: u64::from_str_radix(x, 2).unwrap(),
        })
        .collect();

    c.bench_function("d3a", |b| b.iter(|| day_three::part_one(black_box(&codes))));
    c.bench_function("d3b", |b| {
        b.iter(|| day_three::part_two(black_box(codes.clone())))
    });
    c.bench_function("d3c", |b| {
        b.iter(|| day_three::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_four(c: &mut Criterion) {
    let aoc_input = load_inp(4);
    let (mut boards, numbers) = day_four::parse(aoc_input.clone());

    c.bench_function("d4a", |b| {
        b.iter(|| day_four::part_one(black_box(&numbers), black_box(&mut boards.clone())))
    });
    c.bench_function("d4b", |b| {
        b.iter(|| day_four::part_two(black_box(&numbers), black_box(&mut boards.clone())))
    });
    c.bench_function("d4c", |b| {
        b.iter(|| day_four::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_five(c: &mut Criterion) {
    let aoc_input = load_inp(5);

    c.bench_function("d5a", |b| {
        b.iter(|| day_five::part_one(black_box(&aoc_input.inp)))
    });
    c.bench_function("d5b", |b| {
        b.iter(|| day_five::part_two(black_box(&aoc_input.inp)))
    });
    c.bench_function("d5c", |b| {
        b.iter(|| day_five::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_six(c: &mut Criterion) {
    use std::collections::HashMap;
    let aoc_input = load_inp(6);
    let fish: Vec<u64> = aoc_input
        .inp
        .split(',')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();
    let mut fish_ages = HashMap::new();
    for f in fish {
        let _ = match fish_ages.get(&f) {
            Some(val) => {
                let new_val = val + 1;
                fish_ages.insert(f, new_val)
            }
            None => fish_ages.insert(f, 1),
        };
    }

    c.bench_function("d6a", |b| {
        b.iter(|| day_six::part_one(black_box(&fish_ages)))
    });
    c.bench_function("d6b", |b| {
        b.iter(|| day_six::part_two(black_box(&fish_ages)))
    });
    c.bench_function("d6c", |b| {
        b.iter(|| day_six::solve(black_box(aoc_input.clone())))
    });
}

fn bench_day_seven(c: &mut Criterion) {
    let aoc_input = load_inp(7);

    let crabs: Vec<u64> = aoc_input
        .inp
        .split(',')
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    c.bench_function("d7a", |b| b.iter(|| day_seven::part_one(black_box(&crabs))));
    c.bench_function("d7b", |b| b.iter(|| day_seven::part_two(black_box(&crabs))));
    c.bench_function("d7c", |b| {
        b.iter(|| day_seven::solve(black_box(aoc_input.clone())))
    });
}

criterion_group!(day_1, bench_day_one);
criterion_group!(day_2, bench_day_two);
criterion_group!(day_3, bench_day_three);
criterion_group!(day_4, bench_day_four);
criterion_group!(day_5, bench_day_five);
criterion_group!(day_6, bench_day_six);
criterion_group!(day_7, bench_day_seven);
criterion_main!(day_1, day_2, day_3, day_4, day_5, day_6, day_7);
