#[macro_use]
extern crate bencher;

use bencher::Bencher;
use salesman;

fn bench_4_points(bench: &mut Bencher) {
    bench.iter(|| {
        let rand_points = salesman::example::rand_points_from_chacha(4);
        salesman::anneal::shortest_path(&rand_points, 1);
    })
    // test bench_4_points ... bench: 503,884,679 ns/iter (+/- 7,877,154)
}

benchmark_group!(benches, bench_4_points);
benchmark_main!(benches);
