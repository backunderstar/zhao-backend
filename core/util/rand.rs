use std::iter;

use rand::Rng;

#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::rng().sample(rand::distr::Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}
