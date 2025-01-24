use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;
use crate::interpolation;

extern crate rand;

pub struct ShamirSecretSharing {
    threshold: usize,
    shares: usize,
}

impl ShamirSecretSharing {
    pub fn new(threshold: usize, shares: usize) -> Self {
        ShamirSecretSharing { threshold, shares }
    }

    pub fn split_secret(&self, secret: u64) -> HashMap<usize, u64> {
        let mut rng = rand::thread_rng();
        let mut coefficients = vec![secret];
        for _ in 1..self.threshold {
            coefficients.push(rng.gen_range(0..u64::MAX));
        }

        let mut shares = HashMap::new();
        for i in 1..=self.shares {
            let mut share = 0;
            for (j, coeff) in coefficients.iter().enumerate() {
                share = share.add(coeff * (i as u64).pow(j as u32));
            }
            shares.insert(i, share);
        }

        shares
    }

    pub fn reconstruct_secret(shares: &HashMap<usize, u64>) -> u64 {
        let points: Vec<(u64, u64)> = shares.iter().map(|(&x, &y)| (x as u64, y)).collect();
        interpolation::lagrange_interpolation(&points, 0_u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_secret_sharing() {
        let sss = ShamirSecretSharing::new(3, 5);
        let secret = 123456789;
        let shares = sss.split_secret(secret);

        let selected_shares: HashMap<usize, u64> = shares.iter().take(3).map(|(&k, &v)| (k, v)).collect();
        let reconstructed_secret = ShamirSecretSharing::reconstruct_secret(&selected_shares);

        assert_eq!(secret, reconstructed_secret);
    }
}