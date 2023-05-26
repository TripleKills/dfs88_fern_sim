extern crate dfs88_fern_sim;

use dfs88_fern_sim::{Fern, run_simulation};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001
    };
    run_simulation(&mut fern, 1000);
    println!("final fern2 size: {}", fern.size);
}