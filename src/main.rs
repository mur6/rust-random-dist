use rand::prelude::*;
use rand_distr::StandardNormal;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..=1000 {
        let val: f64 = rng.sample(StandardNormal);
        println!("{}", val);
    }
}
