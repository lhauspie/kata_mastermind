#[macro_use]
extern crate bencher;
use bencher::Bencher;

// `mastermind` is the name of the current crate (see Cargo.toml#[package].name)
extern crate mastermind;

// this allow to avoid writing womething like:
// mastermind::service::evaluate(mastermind::service::vec!(mastermind::service::Color::BLUE), vec!(mastermind::service::Color::YELLOW))
// to call the evaluate function
use mastermind::service::*;

fn evaluate_1_1(bench: &mut Bencher) {
    bench.iter(
        || evaluate(
            vec!(Color::BLUE),
            vec!(Color::YELLOW)
        )
    )
}

fn evaluate_4_4(bench: &mut Bencher) {
    bench.iter(
        || evaluate(
            vec!(Color::BLUE, Color::RED, Color::YELLOW, Color::YELLOW),
            vec!(Color::YELLOW, Color::YELLOW, Color::RED, Color::BLUE)
        )
    )
}

benchmark_group!(
    benches, 
    evaluate_1_1, 
    evaluate_4_4
);
benchmark_main!(benches);