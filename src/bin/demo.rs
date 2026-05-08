use broken_app::{algo, leak_buffer, normalize, sum_even, use_after_free};

fn main() {
    let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums));

    let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", leak_buffer(&data));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    for _ in 0..1000 {
        let fib = algo::slow_fib(30);
        std::hint::black_box(fib);

        let uniq = algo::slow_dedup(std::hint::black_box(&[
            1, 2, 2, 3, 1, 4, 4, 5, 6, 6, 7, 8, 1, 2, 3,
        ]));
        std::hint::black_box(uniq);
    }

    unsafe {
        use_after_free();
    }
}
