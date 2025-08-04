use num_bigint::BigUint;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let g_base = BigUint::from(g);
    let a_exp = BigUint::from(a);
    let p_mod = BigUint::from(p);

    let key = g_base.modpow(&a_exp, &p_mod);
    key.to_u64_digits().first().copied().unwrap_or(0)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let b_pub_base = BigUint::from(b_pub);
    let a_exp = BigUint::from(a);
    let p_mod = BigUint::from(p);

    let key = b_pub_base.modpow(&a_exp, &p_mod);
    key.to_u64_digits().first().copied().unwrap_or(0)
}
