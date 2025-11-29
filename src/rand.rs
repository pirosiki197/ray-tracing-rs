use std::cell::RefCell;

use rand::{
    Rng, SeedableRng,
    distr::{
        Distribution, StandardUniform,
        uniform::{SampleRange, SampleUniform},
    },
    rngs::SmallRng,
};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_rng(&mut rand::rng()));
}

pub fn random<T>() -> T
where
    StandardUniform: Distribution<T>,
{
    RNG.with_borrow_mut(|rng| rng.random())
}

pub fn random_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    RNG.with_borrow_mut(|rng| rng.random_range(range))
}
