use rayon::prelude::*;

fn transform() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // Transform sequentially
    nums.iter_mut().for_each(|n| *n *= 2);
    dbg!(nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // Transform in parallel
    nums.par_iter_mut().for_each(|n| *n *= 2);
    dbg!(nums);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_transform() {
        transform();
    }
}
