use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env;
use rudolf_rs;
use solutions;

fn criterion_benchmark(c: &mut Criterion) {
    let year = 2024;
    let day = 4;

    let session_key = env::var("AOC_SESSION").unwrap();
    let client = rudolf_rs::Client::new(String::from(session_key));
    let input = client.get_cached(year, day).unwrap();
    let solution = solutions::get_solution(year as usize, day as usize);

    c.bench_function(
        &format!("year{}day{}part1", year, day),
        |b| {
            b.iter(|| solution.part1(black_box(&input)))
        }
    );

    c.bench_function(
        &format!("year{}day{}part2", year, day),
        |b| {
            b.iter(|| solution.part2(black_box(&input)))
        }
    );
}

criterion_group!{
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);
