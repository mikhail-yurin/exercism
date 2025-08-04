pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_multipliers: Vec<u32> = Vec::new();

    for factor in factors {
        let mut multiplier = *factor;

        loop {
            if multiplier >= limit || multiplier == 0 {
                break;
            }
            if !all_multipliers.contains(&multiplier) {
                all_multipliers.push(multiplier);
            }
            multiplier += factor;
        }
    }

    all_multipliers.iter().copied().sum()
}
