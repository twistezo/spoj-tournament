mod coins;
mod diehard;
mod lastdig;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    {
        // lastdig::run();
        // coins::run();
        diehard::run();
    }

    println!("\nElapsed: {:.2?}", now.elapsed());
}
