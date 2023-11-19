
// txtbk def

use std::ops::Div;

pub fn drgmt_gen(n: usize, r: usize, k: usize) -> Result<u128, &'static str> {

    if (r > n) | (k > r) { return Ok(0); }

    // let (n,r,k) = (n as isize, r as isize, u as isize);

    let g= ((0..=r-k)
        .fold(0, |acc, i| {
            acc + ( pos_or_neg_cond(1, i & 1 == 0) ) as i128 * ( comb(r - k, i) * perm(n - k - i) ) as i128
        }) as u128)
        .overflowing_mul(comb(r, k));
    
    match g.1 {
        true => Err("!! Overflow occurred"),
        false => Ok(g.0 / perm(n-r)),
    }
}

fn pos_or_neg_cond(n: usize, condition: bool) -> i32 {
    match condition {
        true => n as i32,
        false => - (n as i32),
    }
}

fn perm(n: usize) -> u128 {
    if n <= 1 { return 1; }
    (2..n).fold(1, |acc, i| {
        acc * i as u128
    })
}

fn comb(n: usize, k: usize) -> u128 {

    (0..k)
        .fold(1u128, |acc, i| {
            acc * (n - i) as u128
        })
        .div(k as u128)
}