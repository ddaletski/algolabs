pub mod binary_tree;
pub mod linked_list;

#[cfg(test)]
pub fn random_uniform_list<T: rand::distributions::uniform::SampleUniform>(
    n: usize,
    min: T,
    max: T,
) -> Vec<T> {
    use itertools::Itertools;
    use rand::{Rng, SeedableRng};

    rand::rngs::StdRng::seed_from_u64(0)
        .sample_iter(rand::distributions::Uniform::new_inclusive(min, max))
        .take(n)
        .collect_vec()
}
