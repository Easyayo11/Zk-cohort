use ark_ff::{PrimeField, UniformRand};
use std::collections::HashMap;
use std::ops::Add;
use crate::interpolation;

extern crate rand;

pub struct ShamirSecretSharing <F : PrimeField> {
    threshold: usize,
    shares: usize,
    _field: std::marker::PhantomData<F>,
}

impl<F: PrimeField> ShamirSecretSharing<F> {
    pub fn new(threshold: usize, shares: usize) -> Self {
        ShamirSecretSharing { 
            threshold,
             shares,
             _field: std::marker::PhantomData,}
    }

    pub fn split_secret(&self, secret: F) -> HashMap<usize, F> {
        let mut rng = rand::thread_rng();
        let mut coefficients = vec![secret];
        for _ in 1..self.threshold {
            coefficients.push(F::rand(&mut rng));
        }

        let mut shares = HashMap::new();
        for i in 1..=self.shares {
            let mut share = F::zero();
            let x = F::from(i as u64);
            for (j, coeff) in coefficients.iter().enumerate() {
                share += *coeff * x.pow(&[j as u64 ]);
            }
            shares.insert(i, share);
        }

        shares
    }

    pub fn reconstruct_secret(shares: &HashMap<usize, F>) -> F {
        let points: Vec<(F, F)> = shares.iter().map(|(&x, &y)| (F::from(x as u64), y)).collect();
        interpolation::lagrange_interpolation(&points, F::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use ark_bls12_381::Fr;
    use ark_bn254::Fr;

    #[test]
    fn test_shamir_secret_sharing() {
        let sss = ShamirSecretSharing::<Fr>::new(3, 5);
        let secret =Fr::from(123456u64);
        let shares = sss.split_secret(secret);

        let selected_shares: HashMap<usize, Fr> = shares.iter().take(3).map(|(&k, &v)| (k, v)).collect();
        let reconstructed_secret = ShamirSecretSharing::reconstruct_secret(&selected_shares);

        assert_eq!(secret, reconstructed_secret);
    }
}