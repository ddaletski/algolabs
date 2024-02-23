use bencher::Bencher;
use largest_component_size_by_common_factor::Solution;

fn huge_input_bench(bench: &mut Bencher) {
    let input: Vec<i32> = include!("huge_vec.in");

    bench.iter(move || Solution::largest_component_size(input.clone()))
}

bencher::benchmark_group!(benches, huge_input_bench);
bencher::benchmark_main!(benches);
